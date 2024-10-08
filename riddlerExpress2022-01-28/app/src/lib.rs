use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

pub fn run_generations(generations: usize, print: bool) {
    let starting_grid = {
        let mut starting_cells = HashSet::new();
        starting_cells.insert((0, 0));
        starting_cells.insert((1, 0));
        starting_cells.insert((-1, 0));
        starting_cells.insert((0, 1));
        starting_cells.insert((0, -1));
        Grid::new(starting_cells)
    };

    let mut curr = starting_grid;
    for generation in 2..=generations {
        curr = curr.next_grid();
        if print {
            println!(
                "On generation {}, there are {} live cells",
                generation,
                curr.live_squares.len()
            );
        }
    }
}

/// Since the grid is infinite, we can't just have a regular 2d array for this. I'm just going
/// to store a set of coordinates that are filled in. There might be some compression scheme we
/// could use for this, but I'll save that for later.
struct Grid {
    live_squares: HashSet<Square>,
}

impl Grid {
    fn new(starting_squares: HashSet<Square>) -> Self {
        Self {
            live_squares: starting_squares,
        }
    }

    fn is_alive(&self, x: Coordinate, y: Coordinate) -> bool {
        let start = Instant::now();
        let out = self.live_squares.contains(&(x, y));
        // println!("{}", start.elapsed().as_nanos());
        out
    }

    fn neighbors(&self, x: Coordinate, y: Coordinate) -> HashSet<Square> {
        let mut out = HashSet::with_capacity(8);
        out.insert((x, y + 1));
        out.insert((x, y - 1));
        out.insert((x + 1, y));
        out.insert((x - 1, y));

        out.insert((x + 1, y + 1));
        out.insert((x - 1, y - 1));
        out.insert((x + 1, y - 1));
        out.insert((x - 1, y + 1));
        assert!(out.len() == 8);
        out
    }

    fn should_be_alive_next(&self, x: Coordinate, y: Coordinate) -> bool {
        self.neighbors(x, y)
            .iter()
            .map(|(x, y)| self.is_alive(*x, *y))
            .filter(|x| *x)
            .count()
            >= MIN_LIVE_NEIGHBORS_TO_LIVE_NEXT_ROUND
    }

    fn next_grid(&self) -> Self {
        let potential_live_squares: HashSet<Square> = self
            .live_squares
            .iter()
            .flat_map(|(x, y)| self.neighbors(*x, *y))
            .filter(|(x, y)| !self.is_alive(*x, *y))
            .collect();

        let squares_coming_alive = potential_live_squares
            .into_iter()
            .filter(|(x, y)| self.should_be_alive_next(*x, *y))
            .collect();

        let new_live_squares = self.live_squares.union(&squares_coming_alive).copied();
        Self {
            live_squares: new_live_squares.collect(),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match (
            self.live_squares.iter().map(|(x, _)| x).min(),
            self.live_squares.iter().map(|(x, _)| x).max(),
            self.live_squares.iter().map(|(_, y)| y).min(),
            self.live_squares.iter().map(|(_, y)| y).max(),
        ) {
            (Some(&min_x), Some(&max_x), Some(&min_y), Some(&max_y)) => (min_x..=max_x)
                .map(|x| {
                    let inner_cells = (min_y..=max_y)
                        .map(|y| String::from(if self.is_alive(x, y) { "[x]" } else { "[ ]" }));
                    inner_cells.intersperse(String::from(" ")).collect()
                })
                .intersperse(String::from("\n"))
                .collect(),
            _ => String::from("[Empty grid]"),
        };
        write!(f, "{}", s)
    }
}

type Coordinate = i16;
type Square = (Coordinate, Coordinate);

const MIN_LIVE_NEIGHBORS_TO_LIVE_NEXT_ROUND: usize = 3;
