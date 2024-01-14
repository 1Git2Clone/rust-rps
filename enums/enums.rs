// /enums/enums.rs 
// Author: 1Kill2Steal
// 14.01.2023 (DD/MM/YYYY)



// PartialEq - ./enum.rs -> match_value_to_enum()
// Eq && Hash - ../functions/functions.rs -> generate_game_outcome()
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GameOptions {
    Exit = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl GameOptions {
    pub fn match_value_to_option(value: u8) -> GameOptions {
        match value {
            0 => GameOptions::Exit,
            1 => GameOptions::Rock,
            2 => GameOptions::Paper,
            3 => GameOptions::Scissors,
            // in the case of user inputting an invalid
            // value, it gets checked beforehand in
            // the functions/functions.rs file
            _ => unreachable!(),
        }
    }
}



pub enum GameOutcomes {
    // The actual values don't really matter
    // as long as they're unique
    Lose = 0,
    Draw = 1,
    Win = 2,
}
