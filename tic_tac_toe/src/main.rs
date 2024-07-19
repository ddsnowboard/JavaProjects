use lazy_static::lazy_static;
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
    println!("Hello, world!");
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellValue {
    Filled(bool),
    Empty,
}
const N_CELLS: usize = 9;

#[derive(Clone)]
struct GameState {
    cells: [CellValue; N_CELLS],
}

impl GameState {
    fn start_state() -> Self {
        GameState {
            cells: [CellValue::Empty; N_CELLS],
        }
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
        self.cells.iter().all(|v| *v != CellValue::Empty)
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
    // TODO: Add a function to determine what the next play is give which player plays
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
