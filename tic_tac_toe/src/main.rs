use lazy_static::lazy_static;
use std::fmt;
use std::io;
type WinCondition = Vec<usize>;

lazy_static! {
    static ref WIN_CONDITIONS: Vec<WinCondition> = vec![
        // Horizontal
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],

        // Vertical
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],

        // diagonal
        vec![0, 4, 8],
        vec![2, 4, 6]
    ];
}

fn main() {
    let mut game = GameState::start_state();
    while game.get_winner().is_none() && !game.is_tied() {
        println!("{}", game);
        let mut response = String::new();
        let _ = io::stdin().read_line(&mut response);
        let number: usize = response
            .trim()
            .parse::<usize>()
            .expect("You must give a number")
            - 1;
        game = game.play(true, number).expect("Your play was invalid");
        if game.is_tied() {
            break;
        }
        let computer_play = game
            .next_play(false)
            .expect("The computer couldn't play. Did you win?");
        game = game.play(false, computer_play).unwrap();
    }

    if game.is_tied() {
        println!("It's a tie!");
    } else if let Some(who) = game.get_winner() {
        println!("{} wins!", if who { "X" } else { "Y" });
    } else {
        panic!("Hmm.");
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum CellValue {
    Filled(bool),
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Outcome {
    Winner(bool),
    Tie,
}
const N_CELLS: usize = 9;

#[derive(Clone, Eq, PartialEq, Debug)]
struct GameState {
    cells: [CellValue; N_CELLS],
}

impl GameState {
    fn start_state() -> Self {
        GameState {
            cells: [CellValue::Empty; N_CELLS],
        }
    }

    fn from_strings(strings: [&str; 3]) -> Self {
        fn map_to_value(c: char) -> CellValue {
            match c {
                'X' | 'x' => CellValue::Filled(true),
                'O' | 'o' => CellValue::Filled(false),
                ' ' => CellValue::Empty,
                _ => panic!("{} is not a legal character", c),
            }
        }
        let cells = strings
            .iter()
            .flat_map(|s| {
                if s.len() != 3 {
                    panic!("String was not of length 3!")
                } else {
                    s.chars().map(map_to_value)
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { cells }
    }

    fn get_winner(&self) -> Option<bool> {
        let matches_win_condition = |cond: &WinCondition| {
            let values = cond.iter().map(|idx| self.cells[*idx]);
            if let Some(CellValue::Filled(winning_player)) = all_match(values) {
                Some(winning_player)
            } else {
                None
            }
        };
        for cond in WIN_CONDITIONS.iter() {
            let winner = matches_win_condition(cond);
            if winner.is_some() {
                return winner;
            }
        }
        None
    }

    fn is_tied(&self) -> bool {
        self.get_winner().is_none() && self.cells.iter().all(|v| *v != CellValue::Empty)
    }

    fn get_ultimate_outcome(&self, current_player: bool) -> Outcome {
        if self.is_tied() {
            Outcome::Tie
        } else if let Some(winner) = self.get_winner() {
            Outcome::Winner(winner)
        } else {
            let all_available_plays = self.all_plays();
            let mut all_possible_outcomes: Vec<_> = all_available_plays
                .into_iter()
                .filter_map(|p| {
                    Some((
                        p,
                        self.play(current_player, p)?
                            .get_ultimate_outcome(!current_player),
                    ))
                })
                .collect();
            all_possible_outcomes
                .sort_by_key(|(_, outcome)| outcome_sort_key(outcome, current_player));
            all_possible_outcomes.first().unwrap().1
        }
    }
    fn play(&self, is_x: bool, cell: usize) -> Option<Self> {
        if cell >= N_CELLS {
            return None;
        }
        if self.cells[cell] != CellValue::Empty {
            None
        } else {
            let mut new_state = self.clone();
            new_state.cells[cell] = CellValue::Filled(is_x);
            Some(new_state)
        }
    }

    fn all_plays(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, state)| **state == CellValue::Empty)
            .map(|(idx, _)| idx)
            .collect()
    }

    fn next_play(&self, is_x: bool) -> Option<usize> {
        if self.is_tied() {
            return None;
        }
        let all_available_plays = self.all_plays();
        let mut all_next_games: Vec<(usize, GameState)> = all_available_plays
            .into_iter()
            .filter_map(|p| self.play(is_x, p).map(|new_game| (p, new_game)))
            .collect();
        all_next_games
            .sort_by_key(|(_, game)| outcome_sort_key(&game.get_ultimate_outcome(!is_x), is_x));
        all_next_games.first().map(|(play, _)| *play)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_values: Vec<String> = self
            .cells
            .iter()
            .enumerate()
            .map(|(idx, v)| match v {
                CellValue::Filled(which) => {
                    if *which {
                        "X".to_string()
                    } else {
                        "O".to_string()
                    }
                }
                CellValue::Empty => (1 + idx).to_string(),
            })
            .collect();
        write!(
            f,
            "{} {} {}\n{} {} {}\n{} {} {}\n",
            string_values[0],
            string_values[1],
            string_values[2],
            string_values[3],
            string_values[4],
            string_values[5],
            string_values[6],
            string_values[7],
            string_values[8]
        )
    }
}

fn all_match<T, I>(mut it: I) -> Option<T>
where
    I: Iterator<Item = T>,
    T: Eq + PartialEq,
{
    let expected = it.next()?;
    if it.all(|x| x == expected) {
        Some(expected)
    } else {
        None
    }
}

#[test]
fn all_match_returns_some_if_all_match() {
    let v = vec![1, 1, 1, 1];
    assert_eq!(all_match(v.into_iter()), Some(1));
}

#[test]
fn all_match_returns_none_if_some_dont_match() {
    let v = vec![1, 1, 1, 1, 2];
    assert_eq!(all_match(v.into_iter()), None);
}

#[test]
fn all_match_returns_none_for_empty_list() {
    let v: Vec<u32> = vec![];
    assert_eq!(all_match(v.into_iter()), None);
}

fn outcome_sort_key(o: &Outcome, player_is_x: bool) -> i32 {
    match o {
        Outcome::Winner(w) if *w == player_is_x => -1, // This should come first
        Outcome::Tie => 0,
        _ => 1,
    }
}

#[test]
fn from_string_parses_correctly() {
    let game = GameState::from_strings(["XxX", "OOO", "   "]);
    assert_eq!(
        game,
        GameState {
            cells: [
                CellValue::Filled(true),
                CellValue::Filled(true),
                CellValue::Filled(true),
                CellValue::Filled(false),
                CellValue::Filled(false),
                CellValue::Filled(false),
                CellValue::Empty,
                CellValue::Empty,
                CellValue::Empty
            ]
        }
    );
}

#[test]
fn get_ultimate_outcome_works_for_simple_situations() {
    let simple_situations = vec![
        (["X X", "o o", "x o"], Outcome::Winner(true)),
        (["XOX", "oxo", "oxo"], Outcome::Tie),
        (["Xoo", "o o", "o x"], Outcome::Winner(true)),
    ];
    simple_situations
        .into_iter()
        .for_each(|(game, expected_outcome)| {
            let game = GameState::from_strings(game);
            assert_eq!(game.get_ultimate_outcome(true), expected_outcome);
        });
}

#[test]
fn all_plays_lists_all_plays() {
    let tests = vec![
        (["   ", "   ", "   "], vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        (["X  ", "   ", "   "], vec![1, 2, 3, 4, 5, 6, 7, 8]),
        (["XXX", "XX ", "XXX"], vec![5]),
    ];
    tests.into_iter().for_each(|(game, expected)| {
        let game = GameState::from_strings(game);
        assert_eq!(game.all_plays(), expected);
    });
}

#[test]
fn ultimate_outcome_gets_more_complicated_outcomes() {
    let game = GameState::from_strings(["X X", "OXO", "OO "]);
    assert_eq!(game.get_ultimate_outcome(true), Outcome::Winner(true));
    assert_eq!(game.get_ultimate_outcome(false), Outcome::Winner(false));

    let game = GameState::from_strings(["X  ", " O ", " XO"]);
    assert_eq!(game.get_ultimate_outcome(true), Outcome::Tie);
    assert_eq!(game.get_ultimate_outcome(false), Outcome::Winner(false));

    let game = GameState::from_strings(["XOX", "OOX", "OX "]);
    assert_eq!(game.get_ultimate_outcome(true), Outcome::Winner(true));
}

#[test]
fn next_play_chooses_correctly() {
    let game = GameState::from_strings(["XOX", "OOX", " X "]);
    assert_eq!(game.next_play(false), Some(8));
}
