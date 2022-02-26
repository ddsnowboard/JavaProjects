use rayon::prelude::*;
pub type Int = u32;

pub fn solutions(perimeter: &Int) -> Vec<(Int, Int, Int)> {
    (1..*perimeter)
        .into_par_iter()
        .flat_map(|a| {
            let mut solutions: Vec<_> = vec![];
            for b in 1..=a {
                if let Some(c) = perimeter.checked_sub(a + b) {
                    if a * a + b * b == c * c {
                        solutions.push((a, b, c));
                    }
                }
            }
            solutions
        })
        .collect::<Vec<_>>()
}
