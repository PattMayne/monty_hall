use std::io;
use rand::Rng;
use std::cmp::Ordering;

struct Door {
    winner: bool,
    open: bool
}

struct Game {
    door_a: Door,
    door_b: Door,
    door_c: Door,
}


fn main() {
    
    let game = start_game();


    println!("Hello, world!");
}

fn start_game() {
    // create the game and its doors
    let game = &mut Game {
        door_a: door_builder(),
        door_b: door_builder(),
        door_c: door_builder(),
    };

    //randomly make one a winner
    set_winner(game);

}


fn door_builder() -> Door {
    Door {
        winner: false,
        open: false
    }
}


fn set_winner(game: &mut Game) {
    let winner_id = rand::thread_rng().gen_range(1..=3);
    println!("The number is {}", winner_id)
}
