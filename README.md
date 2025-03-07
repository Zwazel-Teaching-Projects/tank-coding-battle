# tank-coding-battle

## Project Setup

The project is split into multiple members.

- server: Responsible for handling the communication between the clients and the game simulation. Manages lobbies in which the games are played. handles game sim.
- spectator_client: A simple client that can connect to the server and watch the games being played.
- shared: Contains all the shared code between the server and the clients. This includes the game state, the game logic and the communication protocol.

## Tank Ideas

### Light Tank
- Fast
- Turret can rotate independently
- Low damage
- Low health and armor
- Fast reload
- Can shoot while moving
- Can't shoot over obstacles
- Can't shoot at long range
- Can't switch between types of ammunition
- Good for scouting and hit-and-run tactics
- Good for flanking
- Good camouflage
- Good overall Vision
  - But mediocre turret vision

### Heavy Tank
- Not very mobile
- Turret can rotate independently
- High damage
- High health and armor
- Slow reload
- Can't shoot precisely while moving
- Can't shoot over obstacles
- Mediocre overall Vision
  - But good turret vision

### Selbstfahrlafette (Self-Propelled Gun/Artillery)
`self-propelled gun` is a type of artillery that is mounted on a motorized wheeled or tracked chassis. It is used for long-range bombardment support of infantry and tank units.
- Not very mobile
- Turrent can not rotate independently
- Long range
- High damage
- Slow reload
- Low health and armor
- Can't move and shoot precisely at the same time
- Can't shoot at close range
- Can shoot over obstacles
- Can switch between several types of ammunition 
    - Normal: high damage, low splash
    - Smoke: low damage, high splash, low visibility (great for cover)
- Generally bad vision
  - Depends much on the vision of other units

# Credit
- Tank Models made by Alexander Siegmann