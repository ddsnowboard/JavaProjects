fn main() {
    let best_solution = (10..=1000)
        .map(|perimeter| (perimeter, solutions(&perimeter)))
        .max_by_key(|(_, solutions)| solutions.len());
    println!("Maximal perimeter is {:?}", best_solution);
}

type Int = u32;

fn solutions(perimeter: &Int) -> Vec<(Int, Int, Int)> {
    let mut out: Vec<_> = vec![];

    for a in 1..*perimeter {
        for b in 1..=a {
            if let Some(c) = perimeter.checked_sub(a + b) {
                if a * a + b * b == c * c {
                    out.push((a, b, c));
                }
            }
        }
    }
    out
}
