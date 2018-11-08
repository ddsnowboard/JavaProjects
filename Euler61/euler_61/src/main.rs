fn number_length(n: u32) -> u32 {
    let l = (n as f32).log10();
    (l as u32) + 1
}
fn main() {
    let triangle = |n: u32| n * (n + 1) / 2;
    let square = |n: u32| n * n;
    let penta = |n: u32| n * (3 * n - 1) / 2;
    let hex = |n: u32| n * (2 * n - 1);
    let hepta = |n: u32| n * (5 * n - 3) / 2;
    let octa = |n: u32| n * (3 * n - 2);

    println!("Hello, world!");
}

#[test]
fn test_number_length() {
    assert_eq!(number_length(10000), 5);
    assert_eq!(number_length(10001), 5);
    assert_eq!(number_length(0), 1);
    assert_eq!(number_length(10), 2);
    assert_eq!(number_length(11), 2);
}
