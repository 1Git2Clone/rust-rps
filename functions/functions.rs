// functions.rs
// Author: 1Kill2Steal
// 14.01.2023 (DD/MM/YYYY)

use std::io::{self, Write};
use std::collections::HashMap;
use rand::Rng;

// ../enums/enums.rs imports
use crate::enums::enums::{GameOptions, GameOutcomes};
//


// Match the user input to the hash table's game state char.
pub fn handle_game(
    user_choice: GameOptions,
    computer_choice: GameOptions,
    score: &mut i32,
    outcomes: &HashMap<(GameOptions, GameOptions), GameOutcomes>,
) -> bool {

    match outcomes.get(&(user_choice, computer_choice)) {
        Some(result) => {
            match result {
                GameOutcomes::Draw => println!("Draw."),
                GameOutcomes::Lose => {
                    println!("Loss.");
                    *score -= 1;
                }
                GameOutcomes::Win => {
                    println!("Win.");
                    *score += 1;
                }
                // actually unreachable with enums 
            }
        }
        None => unreachable!(),
    }

    println!("Current score: {}", *score);

    *score >= 0
}

// Tuple of two enums (../enums/enums.rs) for the player vs computer variants
pub fn generate_game_outcomes() -> HashMap<(GameOptions, GameOptions), GameOutcomes> {

    use crate::enums::enums::{GameOptions::*, GameOutcomes::*};

    let mut outcomes = HashMap::new();

    outcomes.insert((Rock, Rock), Draw);
    outcomes.insert((Rock, Paper), Lose);
    outcomes.insert((Rock, Scissors), Win);

    outcomes.insert((Paper, Rock), Win);
    outcomes.insert((Paper, Paper), Draw);
    outcomes.insert((Paper, Scissors), Lose);

    outcomes.insert((Scissors, Rock), Lose);
    outcomes.insert((Scissors, Paper), Win);
    outcomes.insert((Scissors, Scissors), Draw);

    outcomes

}

pub fn main_game_loop(
    rng: &mut rand::rngs::ThreadRng,
    outcomes: &HashMap<(GameOptions, GameOptions), GameOutcomes>,
    score: &mut i32
) {
    loop {
        let mut input = String::new();
        print!("Enter 1 (rock), 2 (paper) or 3 (scissors) exit = 0: ");
        io::stdout().flush().unwrap(); // directly print the upper statement.

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let number: u8 = match input.trim().parse() {
            Ok(result) if result <= 3 => result,
            _ => {
                eprintln!("Error: invalid input");
                continue;
            }
        };

        let user_choice = GameOptions::match_value_to_option(number);

        match user_choice {
            GameOptions::Exit => {
                break;
            }
            GameOptions::Rock | GameOptions::Paper | GameOptions::Scissors => {
                let computer_choice: GameOptions = GameOptions::match_value_to_option(rng.gen_range(1..=3));

                // println!("Entered valid number. ({})", number);
                // println!("Computer choice: {}", computer_choice);

                if handle_game(user_choice, computer_choice, score, &outcomes) {
                    continue;
                } else {
                    println!("Skill issue, you got less than 0 points.");
                    break;
                }
            }
        }
    }
}
