
pub mod game {
    use rand::Rng;
    use std::io;
    use crate::game_components::door::Door;
    use crate::game_components::door::door_builder;

    pub struct Game {
        pub doors: [Door; 3],
        pub guess: usize
    }

    impl Game {
        // put the prize behind a random door
        pub fn set_prize(&mut self) {
            let winner_id = rand::thread_rng().gen_range(0..=self.doors.len()-1);
            self.doors[winner_id].prize = true;
        }

        pub fn prompt_user_to_select_door(&mut self) {

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
                    self.doors[guess_number-1].selected = true;
                    self.guess = guess_number;
                    break;
                }      
            }    
        }

        // take one of the non-chosen, non-winning doors and open it
        pub fn open_losing_door(&mut self) {
            let mut losing_indices: Vec<usize> = Vec::new();

            for door in self.doors.iter() {
                if !door.prize && door.number != self.guess {
                    losing_indices.push(door.number-1)
                }
            }

            let random_losing_index = rand::thread_rng().gen_range(0..=losing_indices.len()-1) ;
            self.doors[losing_indices[random_losing_index]].open = true
        }

        // get number of door that user CAN switch to
        fn get_switch_number(&self) -> usize {
            for door in self.doors.iter() {
                if !door.open && !door.selected {
                    return door.number;
                }
            }
            return 10;
        }
        
        pub fn prompt_user_to_switch_doors(&mut self) {
            loop {
                let mut switch_doors_input = String::new();
                // Prompt the user to select Y or N
                io::stdin()
                    .read_line(&mut switch_doors_input)
                    .expect("Failed to read line");

                for character in switch_doors_input.chars() {
                    if character == 'Y' || character == 'y' {
                        self.switch_selection();
                        return;
                    } else if character == 'N' || character == 'n' {
                        return;
                    } else {
                        println!(" ");
                        println!("Please enter Y or N");
                        println!(" ");
                        break;
                    }
                }
            }
        }

        // deselect original selection and select the other non-open door
        fn switch_selection(&mut self) {
            for door in self.doors.iter_mut() {
                if !door.open {
                    door.selected = !door.selected;
                } 
            }
        }

        // After 2nd user input, show the prize location
        pub fn open_all_doors(&mut self) {
            for door in self.doors.iter_mut() {
                door.open = true
            }
        }

        // End the game and display win or loss
        pub fn conclude(&mut self) {
            for door in self.doors.iter() {
                if door.prize && door.selected {
                    println!("You Win!");
                    println!(" ");
                    return;
                }
            }
        
            println!("You Lose!");
            println!(" ");
        }

        // print entire game state
        pub fn print_doors(&mut self) {
            println!(" ");
            println!("The Doors:");
            println!(" ");
            let mut is_all_doors_open = true;
        
            for door in self.doors.iter() {
                door.print();
                if !door.open {
                    is_all_doors_open = false;
                }
            }
        
            println!(" ");
            if self.guess > 0 && !is_all_doors_open {
                println!(
                    "Do you want to switch your selection to door {}? (type Y or N)",
                    self.get_switch_number());
                println!(" ");
            }
        }
    }

    // make the game object
    pub fn game_builder() -> Game {
        Game {
            doors: [door_builder(1), door_builder(2), door_builder(3)],
            guess: 0,
        }
    }
        

}

pub mod door {
    // The doors to be opened
    pub struct Door {
        pub number: usize,
        pub prize: bool,
        pub open: bool,
        pub selected: bool,
    }

    impl Door {    
        // print door status
        pub fn print(&self) {
            let mut door_state_string = String::new();
            let door_number_string: String = self.number.to_string();
        
            // add the number (in parentheses if selected)
            door_state_string.push_str(&door_number_string);
            door_state_string.push_str(": ");
        
            // add door state (unopened, dud, or prize)
            door_state_string.push_str(&self.get_door_state_string() );
            door_state_string.push_str( if self.selected {" <<"} else {""});  
        
            println!("{door_state_string}");
            println!(" ");
        }
        
        // return door state (unopened, dud, or prize)
        pub fn get_door_state_string(&self) -> String {
            return String::from(if self.open {
                if self.prize { if self.selected { "[ PRIZE selected ]" } else { "[ PRIZE ]" } }
                else { if self.selected { "[ loser selected ]" } else { "[ loser ]" } }
            } else if self.selected { "[ ???? selected ]" } else { "[ ???? ]" })
        }
    }

    // make a basic door
    pub fn door_builder(number: usize) -> Door {
        Door {
            number,
            prize: false,
            open: false,
            selected: false
        }
    }
    

}
