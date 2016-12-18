#![allow(non_snake_case)]
fn count_paths(max_x: usize, max_y: usize) -> usize
{
    let actual_width = max_x + 1;
    let actual_height = max_y + 1;
    let mut paths: Vec<Vec<usize>> = vec![vec![0; actual_height];actual_width];
    let retval = _count_paths(&mut paths, actual_width, actual_height, 0, 0);
    for row in paths {
        for cell in row {
            print!("{} | ", cell);
        }
        println!("");
    }
    return retval;
}
fn _count_paths(mut paths: &mut Vec<Vec<usize>>, max_x: usize, max_y: usize, x: usize, y: usize) -> usize
{
    if x == max_x - 1 && y == max_y - 1 {
        return 1;
    }

    if paths[x][y] == 0
    {
        let mut out = 0;
        if x + 1 < max_x
        {
            out += _count_paths(&mut paths, max_x, max_y, x + 1, y);
        }
        if y + 1 < max_y
        {
            out += _count_paths(&mut paths, max_x, max_y, x, y + 1);
        }
        paths[x][y] = out;
        return out;
    }
    else 
    {
        return paths[x][y];
    }

}
fn main() {
    println!("{}", count_paths(20, 20));
}

#[test]
fn two_by_two_test()
{
    assert_eq!(6, count_paths(2, 2));
}
