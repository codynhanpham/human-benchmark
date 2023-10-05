mod tasks;

mod utils;
use utils::*;

fn main() {
    // ask for which program to run
    println!("Which program would you like to run?");
    println!();
    println!("0. Mouse Position Tracker\n");
    println!("1. Reaction Time");
    println!("2. Aim Trainer");
    println!("3. Sequence Memory");
    println!("4. Typing Test");

    println!();

    let program = get_input_int("Enter the number of the program: ");

    match program {
        0 => tasks::mouse_pos_tracker::mouse_pos_tracker(),
        1 => tasks::reaction_time::reaction_time(),
        2 => tasks::aim_trainer::aim_trainer(),
        3 => tasks::sequence_memory::sequence_memory(),
        4 => tasks::typing_test::typing_test(),
        _ => {
            println!("Please enter a valid number.");
            main();
        }
    }

    // wait for enter before exit
    println!("\n");
    let _ = get_input("Press enter to exit...");
}