use std::io::{self, Write};
use rand::Rng; // time to handle randomized rps.

fn handle_game(input: i32, computer_choice: i32) -> i32 {
    let mut win_or_loss: i32 = 0; // 0 = loss, 1 = draw, 2 = win
    match input {
        1 => {
            match computer_choice {
                1 => {
                    println!("Draw.");
                    win_or_loss = 1;
                }
                2 => {
                    println!("Loss.");
                }
                3 => {
                    println!("Win!");
                    win_or_loss = 2;
                }
                _ => {
                    eprintln!("Undefined behavior.");
                }
            }
        }
        2 => {
            match computer_choice {
                1 => {
                    println!("Win!");
                    win_or_loss = 2;
                }
                2 => {
                    println!("Draw.");
                    win_or_loss = 1;
                }
                3 => {
                    println!("Loss.");
                }
                _ => {
                    eprintln!("Undefined behavior.");
                }
            }
        }
        3 => {
            match computer_choice {
                1 => {
                    println!("Loss.");
                }
                2 => {
                    println!("Win!");
                    win_or_loss = 2;
                }
                3 => {
                    println!("Draw.");
                    win_or_loss = 1;
                }
                _ => {
                    eprintln!("Undefined behavior.");
                }
            }
        }
        _ => {
            eprintln!("Undefined behavior.");
        }
    }


    win_or_loss
}

fn main() {
    let mut rng = rand::thread_rng();
    // value decreases on loss and increases on win.
    let mut score: i32 = 0;
    loop {
        let mut input = String::new();
        print!("Enter 1 (rock), 2 (paper) or 3 (scissors) exit = 0: ");
        io::stdout().flush().unwrap(); // directly print the upper statement.

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<i32>() {
            Ok(number) => {
                if number == 0 {
                    break;
                } else if number > 0 && number < 4 {
                    let computer_choice: i32 = rng.gen_range(1..=3);

                    println!("Entered valid number. ({})", number);
                    println!("Computer choice: {}", computer_choice);

                    let result: i32 = handle_game(number, computer_choice);
                    if result == 2 {
                        score += 1;
                    } else if result == 0 {
                        score -= 1;
                    }
                    if score < 0 {
                        println!("Skill issue, you got less than 0 points.");
                        break;
                    } else {
                        println!("Current score: {}", score);
                    }
                } else {
                    println!("Invalid number.");
                }
            }
            Err(_) => {
                eprintln!("Error: invalid input")
            }
        }
    }
}
