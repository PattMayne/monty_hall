mod game_components;
use crate::game_components::game::game_builder;

/**
 * TO DO:
 * * use maps and filters
 * * Put structs and impls in their own files
 *   (door.rs & game.rs)
 * 
 * * RUST IMPORTS:
 * * I must make hierarchical trees.
 * * Doors should be inside the Game folder.
 * 
 * "Rust lets you split a package into multiple crates and a crate into modules
 * so you can refer to items defined in one module from another module."
 */

 /**
  * In the classic Monty Hall problem:
  * -- you hide a prize behind one of three doors.
  * -- ask the player to pick one door.
  * -- At least one of the other two must be empty.
  * -- randomly open one of the duds
  * -- Ask player if they want to switch their selection for the other unopened door.
  * -- Show final results
  */

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
    game.set_prize();

    // Print the doors in their raw, unopened state
    game.print_doors();

    // Prompt the user to choose a door
    game.prompt_user_to_select_door();

    println!(" ");
    println!("You chose Door {}!", game.guess);
    println!(" ");

    // Show the user one non-selected, non-prize door
    game.open_losing_door();
    game.print_doors();

    // Does the user want to keep their selection, or switch?
    game.prompt_user_to_switch_doors();

    // Use has chosen. Show the result.
    game.open_all_doors();
    game.print_doors();
    game.conclude();    
}
