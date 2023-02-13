// enums are types which have a few definite values
// kind of little tricksy, but they're pretty cool

#[derive(Debug)]

enum Direction {
    N,
    S,
    E,
    W,
}

enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

fn main() {
    let demo_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 4,
    };
    match demo_player_action {
        PlayerAction::Wait => println!("Player wants to rest!"),
        PlayerAction::Move { direction, speed } => {
            println!(
                "Player wants to move in direction {:?} with speed {:?}",
                direction, speed
            )
        }
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
    };
}
