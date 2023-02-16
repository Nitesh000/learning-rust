// imple for the structs
// Path: structs.rs
#[allow(dead_code)]
struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

// in the impl block we can define methods for the struct
impl Player {
    fn with_name(name: &str) -> Player {
        // this is like a default constructor for creating new
        // Objects
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 0,
        }
    }

    // some methods
    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

fn main() {
    let mut player = Player::with_name("Nitesh");
    player.set_friends(10);
    println!("{} has {} friends", player.name, player.get_friends()); // Nitesh has 10 friends
}
