use bevy::prelude::*;
use messages::MySharedNetworkMessagesPlugin;
use networking_state::MyNetworkingState;
use networking_system_sets::MyNetworkingSet;

pub mod messages;
pub mod networking_state;
pub mod networking_system_sets;
pub mod lobby_management;

pub struct MySharedNetworkingPlugin;

impl Plugin for MySharedNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MySharedNetworkMessagesPlugin,))
            .add_sub_state::<MyNetworkingState>()
            .configure_sets(
                Update,
                (
                    MyNetworkingSet::AcceptConnections,
                    (
                        MyNetworkingSet::ReadingMessages,
                        MyNetworkingSet::SendingMessages,
                    ),
                )
                    .run_if(in_state(MyNetworkingState::Running))
                    .chain(),
            );

        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<MyNetworkingState>,
        );
    }
}
