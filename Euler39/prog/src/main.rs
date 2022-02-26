use prog::solutions;

fn main() {
    let best_solution = (10..=1000)
        .map(|perimeter| (perimeter, solutions(&perimeter)))
        .max_by_key(|(_, solutions)| solutions.len());
    println!("Maximal perimeter is {:?}", best_solution);
}
