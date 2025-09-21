pub fn collatz(n: u64) -> Option<u64> {
    // unimplemented!(
    //     "return Some(x) where x is the number of steps required to reach 1 starting with {}",
    //     n,
    // )
    match n{
        0 => None,
        1 => Some(0),
        _ => match n % 2 {
            0 => collatz(n/2).map(|x| x+1),
            _ => collatz(n.checked_mul(3)?.checked_add(1)?).map(|x| x+1),
        }
    }
}
