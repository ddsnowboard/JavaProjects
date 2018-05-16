fn main() {
    const MAX: u64 = 1000000;
    // It's pretty much same speed as C. Which is impressive. Can we build a threadpool and somehow
    // get better performance? Let's find out.
    for i in 1..MAX {
        collatz(i);
    }
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
fn collatz(i: u64) -> bool
{
    let originali = i;
    let mut i = i;
    if i == 0  {
        return true;
    }
    // These are to make sure that we aren't just going in a circle.
    let mut newest = 0;
    let mut second_newest = 0;
    while i != 1
    {
        if i % 2 == 0  {
            i /= 2;
        } else if i % 2 != 0  {
            i = 3 * i + 1;
        } else {
            println!("Something really bad just happened...");
            return false;
        }

        if i == newest || i == second_newest  {
            println!("Doesn't work for {}", originali);
            return false;
        } else {
            second_newest = newest;
            newest = i;
        }
    }
    return true;
}
