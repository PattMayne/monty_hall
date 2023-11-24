use std::io;
use rand::Rng;

/**
 * TO DO:
 * * create impls for some of the functionality to operate on the structs.
 * * document the living fuck out of it.
 * * use maps and filters
 */

// The doors to be opened
struct Door {
    number: usize,
    prize: bool,
    open: bool,
    selected: bool,
}

struct Game {
    doors: [Door; 3],
    guess: usize
}

/**
 * When the main() function finishes running,
 * the player has already won or lost.
 * 
 * Two loops are used around the user inputs
 * to make sure we get a good response.
 */
fn main() {
    // Make a game and start it
    let mut game = game_builder();
    set_prize(&mut game);

    // Print the doors in their raw, unopened state
    print_doors(&mut game);

    // Prompt the user to choose a door
    game.guess = prompt_user_to_select_door(&mut game);

    println!(" ");
    println!("You chose Door {}!", game.guess);
    println!(" ");

    open_losing_door(&mut game);
    print_doors(&mut game);    
    prompt_user_to_switch_doors(&mut game);    
    open_all_doors(&mut game);
    print_doors(&mut game);

    // get winning number
    run_conclusion(&mut game);
    
}

fn run_conclusion(game: &mut Game) {
    for door in game.doors.iter() {
        if door.prize {
            if door.selected {
                println!("You Win!");
                return;
            } 
        }
    }
    println!("You Lose!");
}

fn prompt_user_to_switch_doors(game: &mut Game) {
    loop {
        let mut switch_doors_input = String::new();
        // Prompt the user to select Y or N
        io::stdin()
            .read_line(&mut switch_doors_input)
            .expect("Failed to read line");
        
        if ( switch_doors_input.contains("Y") || switch_doors_input.contains("y") )
            && !switch_doors_input.contains("N") && !switch_doors_input.contains("n") {
            switch_selection(game);
            break;
        } else if ( switch_doors_input.contains("N") || switch_doors_input.contains("n") )
            && !switch_doors_input.contains("Y") && !switch_doors_input.contains("y") {
            break;
        } else {
            println!(" ");
            println!("Please entery Y or N");
            println!(" ");
        }
    }
}

fn prompt_user_to_select_door(game: &mut Game) -> usize {

    loop {
        println!(" ");
        println!("Please choose Door 1, 2, or 3");
        println!(" ");

        let mut guess_string = String::new();

        // get the user input as a string
        io::stdin()
            .read_line(&mut guess_string)
            .expect("Failed to read line");
        
        // parse numerical character into number
        // if user entered non-numerical (parse fails), restart the loop
        let guess_number: usize = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // if number out of bounds, restart the loop
        // otherwise return validated value
        if guess_number > 0 && guess_number < 4 {
            game.doors[guess_number-1].selected = true;
            return guess_number;
        }      
    }    
}

// deselect original selection and select the other non-open door
fn switch_selection(game: &mut Game) {
    for door in game.doors.iter_mut() {
        if !door.open {
            door.selected = !door.selected;
        } 
    }
}

fn open_all_doors(game: &mut Game) {
    for door in game.doors.iter_mut() {
        door.open = true
    }
}


// take one of the non-chosen, non-winning doors and open it
fn open_losing_door(game: &mut Game) {
    let mut losing_indices: Vec<usize> = Vec::new();

    for door in game.doors.iter() {
        if !door.prize && door.number != game.guess {
            losing_indices.push(door.number-1)
        }
    }

    let random_losing_index = rand::thread_rng().gen_range(0..=losing_indices.len()-1) ;
    game.doors[losing_indices[random_losing_index]].open = true
}

// make the game object
fn game_builder() -> Game {
    Game {
        doors: [door_builder(1), door_builder(2), door_builder(3)],
        guess: 0,
    }
}

// make a basic door
fn door_builder(number: usize) -> Door {
    Door {
        number,
        prize: false,
        open: false,
        selected: false
    }
}


fn set_prize(game: &mut Game) {
    let winner_id = rand::thread_rng().gen_range(0..=game.doors.len()-1);
    game.doors[winner_id].prize = true;
}


fn print_doors(game: &mut Game) {
    println!(" ");
    println!(" ");

    // Building the string to display all doors' stats
    let mut doors_state_string = String::from(" ");

    for door in game.doors.iter() {
        let door_number_string: String = door.number.to_string();

        // add the number (in parentheses if selected)
        doors_state_string.push_str( if door.selected {">> "} else {""});        
        doors_state_string.push_str(&door_number_string);
        doors_state_string.push_str(": ");

        // add door state (unopened, dud, or prize)
        doors_state_string.push_str(&get_door_state_string(&door) );
        doors_state_string.push_str("   ");
    }
    doors_state_string.push_str("   ");
    println!("The Doors: {doors_state_string}");
    println!(" ");
    println!(" ");
    if game.guess > 0 {
        println!("Do you want to switch doors? (type Y or N)");
        println!(" ");
    }
}

// return door state (unopened, dud, or prize)
fn get_door_state_string(door: &Door) -> String {
    return String::from(if door.open {
        if door.prize { "[ prize ]" }
        else { "[ loser ]" }
    } else if door.selected { "[ selected ]" } else { "[ ???? ]" })
}
