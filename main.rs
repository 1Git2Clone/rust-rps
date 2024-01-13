use std::io::{self, Write};
use std::collections::HashMap;
use rand::Rng; // time to handle randomized rps.

fn handle_game(
    input: u8,
    computer_choice: u8,
    score: &mut i32,
    outcomes: &HashMap<(u8, u8), char>,
) -> bool {

    match outcomes.get(&(input, computer_choice)) {
        Some(result) => {
            match result {
                &'d' => println!("Draw."),
                &'l' => {
                    println!("Loss.");
                    *score -= 1;
                }
                &'w' => {
                    println!("Win.");
                    *score += 1;
                }
                _ => unreachable!(),
            }
        }
        None => unreachable!(),
    }

    println!("Current score: {}", *score);

    *score >= 0
}

fn generate_game_outcomes() -> HashMap<(u8, u8), char> {
    let mut outcomes = HashMap::new();

    // d = draw, w = win, l = loss
    outcomes.insert((1, 1), 'd');
    outcomes.insert((1, 2), 'l');
    outcomes.insert((1, 3), 'w');
    outcomes.insert((2, 1), 'w');
    outcomes.insert((2, 2), 'd');
    outcomes.insert((2, 3), 'l');
    outcomes.insert((3, 1), 'l');
    outcomes.insert((3, 2), 'w');
    outcomes.insert((3, 3), 'd');

    outcomes
}

fn main() {
    let mut rng = rand::thread_rng();
    let outcomes = generate_game_outcomes();
    // value decreases on loss and increases on win.
    let mut score: i32 = 0;

    loop {
        let mut input = String::new();
        print!("Enter 1 (rock), 2 (paper) or 3 (scissors) exit = 0: ");
        io::stdout().flush().unwrap(); // directly print the upper statement.

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let Ok(number) = input.trim().parse::<u8>() else {
            eprintln!("Error: invalid input");
            continue;
        };
        
        match number {
            0 => {
                break;
            }
            1..=3 => {
                let computer_choice: u8 = rng.gen_range(1..=3);

                // println!("Entered valid number. ({})", number);
                // println!("Computer choice: {}", computer_choice);

                if handle_game(number, computer_choice, &mut score, &outcomes) {
                    continue;
                } else {
                    println!("Skill issue, you got less than 0 points.");
                    break;
                }
            }
            // This one isn't `unreachable!()`
            // because the user could've had a typo.
            _ => eprintln!("Enter a number from 0 to 3 included."),
        }
    }
}
