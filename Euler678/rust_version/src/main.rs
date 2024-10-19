use std::iter;

type Num = u32;

fn main() {
    println!("{:?}", get_tuples((10 as Num).pow(12)).len());
}

fn count(start: Num) -> impl Iterator<Item = Num> {
    iter::successors(Some(start), |n| Some(n + 1))
}

fn is_close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00000000001
}

fn calculate_left_side(a: Num, b: Num, e: Num) -> Option<Num> {
    a.checked_pow(e)?.checked_add(b.checked_pow(e)?)
}

fn get_tuples(N: Num) -> Vec<(Num, Num, Num, Num, Num)> {
    let min_e = 2;
    let min_f = 3;
    let mut out = vec![];
    for b in count(1) {
        if b.pow(min_e) > N {
            break;
        }
        for a in 1..b {
            if b.pow(min_e) + a.pow(min_e) > N {
                break;
            }
            for e in count(2) {
                let left_side = calculate_left_side(a, b, e);
                if left_side.is_none_or(|left| left > N) {
                    break;
                }
                for f in count(3) {
                    // There's surely a slick way to avoid this unwrap() but I'll do it later
                    let left_side: f64 = left_side.unwrap().into();
                    let potential_c = left_side.powf(1.0_f64 / f as f64);
                    if is_close(potential_c, potential_c.round()) {
                        out.push((a, b, potential_c.round() as Num, e, f))
                    } else if potential_c < 2.0 {
                        break;
                    }
                }
            }
        }
    }
    out
}
