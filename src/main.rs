mod tasks;

mod utils;
use utils::*;

fn main() {
    println!("PLEASE try not to change the humanbenchmark window size or position during the run.");
    println!("\n----------\n");
    
    // ask for which program to run
    println!("Which program would you like to run?");
    println!();
    println!("0. Mouse Position Tracker\n");
    println!("1. Reaction Time");
    println!("2. Aim Trainer");
    println!("3. Sequence Memory");
    println!("4. Typing Test");
    println!("5. Verbal Memory");


    println!();

    let program = get_input_int("Enter the number of the program: ", None);

    match program {
        0 => tasks::mouse_pos_tracker::mouse_pos_tracker(),

        1 => tasks::reaction_time::reaction_time(),
        10 => tasks::js_inject::reaction_time_inject(),

        2 => tasks::aim_trainer::aim_trainer(),
        20 => tasks::js_inject::aim_trainer_inject(),

        3 => tasks::sequence_memory::sequence_memory(),

        4 => tasks::typing_test::typing_test(),

        5 => tasks::verbal_memory::verbal_memory(),

        
        _ => {
            println!("Please enter a valid number.");
            main();
        }
    }

    // wait for enter before exit
    println!("\n");
    let _ = get_input("Press enter to exit...");
}