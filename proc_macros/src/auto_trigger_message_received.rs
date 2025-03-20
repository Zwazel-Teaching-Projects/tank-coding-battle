use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, DeriveInput, Ident, ItemEnum, Token,
};

// Similar to the "target" attribute, but now for "behaviour".
fn get_allowed_behaviour(attrs: &[Attribute]) -> Option<Vec<Ident>> {
    for attr in attrs {
        if attr.path().is_ident("behaviour") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                let nested = Punctuated::<syn::Meta, Token![,]>::parse_terminated
                    .parse2(meta_list.tokens.clone())
                    .ok()?;
                let mut behaviour_vals = Vec::new();
                for meta in nested {
                    if let syn::Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            behaviour_vals.push(ident.clone());
                        }
                    }
                }
                return Some(behaviour_vals);
            }
        }
    }
    None
}

// Use an enum to keep code readable. We handle only 'Local' or 'Forward'.
#[derive(Debug)]
enum MessageBehaviour {
    // Any Message received will just be handled locally, not being forwarded to the clients/Bots (Default behaviour).
    Local,
    // Any Message received will be forwarded to the clients/Bots, not being handled locally.
    Forward,
}

// Structure to capture macro args.
struct AutoTriggerArgs {
    target_enum: ItemEnum,
    message_enum: ItemEnum,
}

impl Parse for AutoTriggerArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_ident: Ident = input.parse()?;
        if target_ident != "target" {
            return Err(syn::Error::new(target_ident.span(), "Expected 'target'"));
        }
        input.parse::<Token![=]>()?;
        let content;
        braced!(content in input);
        let target_enum: ItemEnum = content.parse()?;

        input.parse::<Token![,]>()?;
        let message_ident: Ident = input.parse()?;
        if message_ident != "message" {
            return Err(syn::Error::new(message_ident.span(), "Expected 'message'"));
        }
        input.parse::<Token![=]>()?;
        let content_msg;
        braced!(content_msg in input);
        let message_enum: ItemEnum = content_msg.parse()?;

        Ok(AutoTriggerArgs {
            target_enum,
            message_enum,
        })
    }
}

// Extract function name from #[get_targets(...)].
fn get_get_targets_fn(attrs: &[Attribute]) -> Option<Ident> {
    for attr in attrs {
        if attr.path().is_ident("get_targets") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                let nested = Punctuated::<syn::Meta, Token![,]>::parse_terminated
                    .parse2(meta_list.tokens.clone())
                    .ok()?;
                if let Some(syn::Meta::Path(path)) = nested.into_iter().next() {
                    return path.get_ident().cloned();
                }
            }
        }
    }
    None
}

// Extract allowed targets from #[target(...)].
fn get_allowed_targets(attrs: &[Attribute]) -> Option<Vec<Ident>> {
    for attr in attrs {
        if attr.path().is_ident("target") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                let nested = Punctuated::<syn::Meta, Token![,]>::parse_terminated
                    .parse2(meta_list.tokens.clone())
                    .ok()?;
                let mut targets = Vec::new();
                for meta in nested {
                    if let syn::Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            targets.push(ident.clone());
                        }
                    }
                }
                return Some(targets);
            }
        }
    }
    None
}

// Extract allowed player states from #[player_state(...)].
fn get_allowed_player_states(attrs: &[Attribute]) -> Option<Vec<Ident>> {
    for attr in attrs {
        if attr.path().is_ident("player_state") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                let nested = Punctuated::<syn::Meta, Token![,]>::parse_terminated
                    .parse2(meta_list.tokens.clone())
                    .ok()?;
                let mut state_vals = Vec::new();
                for meta in nested {
                    if let syn::Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            state_vals.push(ident.clone());
                        }
                    }
                }
                return Some(state_vals);
            }
        }
    }
    None
}

pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AutoTriggerArgs);
    let input_ast = parse_macro_input!(item as DeriveInput);

    let target_enum = args.target_enum;
    let message_enum = args.message_enum;

    let target_enum_ident = &target_enum.ident;
    let message_enum_ident = &message_enum.ident;

    // Clean target enum from #[get_targets].
    let mut cleaned_target_enum = target_enum.clone();
    for variant in cleaned_target_enum.variants.iter_mut() {
        variant
            .attrs
            .retain(|attr| !attr.path().is_ident("get_targets"));
    }

    // We'll build the target-match arms once and store them.
    let target_match_arm_tokens: Vec<_> = target_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let get_targets_fn = get_get_targets_fn(&variant.attrs)
                .unwrap_or_else(|| format_ident!("undefined_get_targets"));

            let pattern = match &variant.fields {
                syn::Fields::Unit => quote! { #target_enum_ident::#variant_ident },
                syn::Fields::Unnamed(_) => quote! { #target_enum_ident::#variant_ident (..) },
                syn::Fields::Named(_) => quote! { #target_enum_ident::#variant_ident { .. } },
            };

            quote! {
                #pattern => lobby_management.#get_targets_fn(lobby_management_arg)
            }
        })
        .collect();

    // Combine into a single token tree for repeated usage.
    let target_match_arms = quote! {
        #( #target_match_arm_tokens, )*
    };

    // Clean message enum of #[target], #[behaviour], #[player_state], and #[unique] attributes.
    let mut cleaned_message_enum = message_enum.clone();
    for variant in cleaned_message_enum.variants.iter_mut() {
        variant.attrs.retain(|attr| !attr.path().is_ident("target"));
        variant
            .attrs
            .retain(|attr| !attr.path().is_ident("behaviour"));
        variant
            .attrs
            .retain(|attr| !attr.path().is_ident("player_state"));
        variant.attrs.retain(|attr| !attr.path().is_ident("unique"));
    }

    let message_enum_for_match = message_enum.clone();
    let mut message_match_arms = Vec::new();

    for variant in message_enum_for_match.variants.iter() {
        let variant_ident = &variant.ident;
        let trigger_struct_ident = format_ident!("{}Trigger", variant_ident);

        // Allowed targets from #[target(...)]
        let allowed_targets = get_allowed_targets(&variant.attrs);

        // Allowed behaviour from #[behaviour(...)] (single or none).
        let behaviour_list = get_allowed_behaviour(&variant.attrs);
        // We'll interpret the first ident if present; else default to Local.
        let behaviour = if let Some(beh) = behaviour_list {
            if beh.is_empty() {
                MessageBehaviour::Local
            } else {
                match beh[0].to_string().as_str() {
                    "Forward" => MessageBehaviour::Forward,
                    "Local" => MessageBehaviour::Local,
                    other => {
                        return syn::Error::new_spanned(
                            &beh[0],
                            format!("Invalid behaviour: '{}'", other),
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
        } else {
            // No attribute => Local
            MessageBehaviour::Local
        };

        let is_forward_code = match behaviour {
            MessageBehaviour::Forward => quote! { true },
            MessageBehaviour::Local => quote! { false },
        };

        let allowed_states = get_allowed_player_states(&variant.attrs);
        let sender_state_check = if let Some(ref allowed_states) = allowed_states {
            let allowed_state_tokens = allowed_states.iter().map(|s| quote! { PlayerState::#s });
            quote! {
                if let Some(state) = lobby_management_arg.sender_state {
                    if !matches!(state, #( #allowed_state_tokens )|* ) {
                        return Err(ErrorMessageTypes::InvalidSenderState(
                            concat!("Invalid sender state for ", stringify!(#variant_ident)).to_string()
                        ));
                    }
                } else {
                    return Err(ErrorMessageTypes::InvalidSenderState(
                        concat!("Sender state not provided for ", stringify!(#variant_ident)).to_string()
                    ));
                }
            }
        } else {
            quote! {}
        };

        let match_arm = if let Some(targets) = allowed_targets {
            let allowed_patterns = targets.iter().map(|allowed_ident| {
                let target_variant = target_enum
                    .variants
                    .iter()
                    .find(|v| v.ident == *allowed_ident);
                if let Some(var) = target_variant {
                    match &var.fields {
                        syn::Fields::Unit => quote! { #target_enum_ident::#allowed_ident },
                        syn::Fields::Unnamed(_) => {
                            quote! { #target_enum_ident::#allowed_ident (..) }
                        }
                        syn::Fields::Named(_) => {
                            quote! { #target_enum_ident::#allowed_ident { .. } }
                        }
                    }
                } else {
                    quote! { #target_enum_ident::#allowed_ident }
                }
            });

            quote! {
                #message_enum_ident::#variant_ident(data) => {
                    // Check target validity
                    if !matches!(self.target, #( #allowed_patterns )|*) {
                        return Err(ErrorMessageTypes::InvalidTarget(
                            concat!("Invalid target for ", stringify!(#variant_ident)).to_string()
                        ));
                    }

                    // --- New: Sender state check ---
                    #sender_state_check
                    // --- End sender state check ---

                    let is_forward = #is_forward_code;
                    let targets = match self.target {
                        #target_match_arms
                    }.map_err(|e| ErrorMessageTypes::LobbyManagementError(e))?;

                    if is_forward {
                        // Forward the message to out_message_queues
                        if !targets.is_empty() {
                            for target in targets {
                                let mut queue = out_message_queues.get_mut(target)
                                    .map_err(|_| ErrorMessageTypes::LobbyManagementError(
                                        "Failed to get out message queue".to_string()
                                    ))?;
                                queue.push_back(self.clone());
                            }
                        }
                    } else {
                        // Process locally: trigger global or target-specific
                        if targets.is_empty() {
                            commands.trigger(#trigger_struct_ident {
                                message: data.clone(),
                                sender: self.sender.clone(),
                            });
                        } else {
                            commands.trigger_targets(#trigger_struct_ident {
                                message: data.clone(),
                                sender: self.sender.clone(),
                            }, targets);
                        }
                    }
                }
            }
        } else {
            quote! {
                #message_enum_ident::#variant_ident(_data) => {
                    return Err(ErrorMessageTypes::InvalidTarget(
                        concat!("No allowed target defined for ", stringify!(#variant_ident))
                            .to_string()
                    ));
                }
            }
        };

        message_match_arms.push(match_arm);
    }

    let struct_ident = &input_ast.ident;
    let generated_impl = quote! {
        impl #struct_ident {
            pub fn trigger_message_received(
                &self,
                commands: &mut Commands,
                lobby_management: &LobbyManagementSystemParam,
                lobby_management_arg: LobbyManagementArgument,
                out_message_queues: &mut Query<&mut OutMessageQueue>,
            ) -> Result<(), ErrorMessageTypes> {
                match &self.message {
                    #( #message_match_arms , )*
                }
                Ok(())
            }
        }
    };

    // Generate client match arms: simply trigger the message on the provided target.
    let client_message_match_arms: Vec<_> = message_enum_for_match
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let trigger_struct_ident = format_ident!("{}Trigger", variant_ident);
            quote! {
                #message_enum_ident::#variant_ident(data) => {
                    // Client: directly trigger using the given target.
                    commands.trigger_targets(#trigger_struct_ident {
                        message: data.clone(),
                        sender: self.sender.clone(),
                    }, target);
                }
            }
        })
        .collect();

    // Append the client function in an additional impl block.
    let client_generated_impl = quote! {
        impl #struct_ident {
            pub fn trigger_message_received_client(
                &self,
                commands: &mut Commands,
                target: Entity, // Use the client-provided target.
            ) -> Result<(), ErrorMessageTypes> {
                match &self.message {
                    #( #client_message_match_arms, )*
                }
                Ok(())
            }
        }
    };

    let unique_match_arms: Vec<_> = message_enum_for_match
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let pattern = match &variant.fields {
                syn::Fields::Unit => quote! { #message_enum_ident::#variant_ident },
                syn::Fields::Unnamed(_) => quote! { #message_enum_ident::#variant_ident (..) },
                syn::Fields::Named(_) => quote! { #message_enum_ident::#variant_ident { .. } },
            };
            let unique_value = if variant
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("unique"))
            {
                quote! { true }
            } else {
                quote! { false }
            };
            quote! {
                #pattern => #unique_value
            }
        })
        .collect();

    let unique_impl = quote! {
        impl #struct_ident {
            pub fn is_unique(&self) -> bool {
                match &self.message {
                    #( #unique_match_arms, )*
                }
            }
        }
    };

    let output = quote! {
        #cleaned_target_enum
        #cleaned_message_enum
        #input_ast
        #generated_impl
        #client_generated_impl
        #unique_impl
    };
    output.into()
}
