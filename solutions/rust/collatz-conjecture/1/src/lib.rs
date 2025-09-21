pub fn collatz(n: u64) -> Option<u64> {
    // unimplemented!(
    //     "return Some(x) where x is the number of steps required to reach 1 starting with {}",
    //     n,
    // )
    if n == 0 {
        return None;
    } else if n == 1 {
        return Some(0);
    } else if n % 2 == 0 {
        let next = u64::checked_div(n, 2);
        match next {
            Some(x) => {
                let next_step = collatz(x);
                match next_step {
                    Some(y) => Some(y + 1),
                    None => None,
                }
            }
            None => None,
        }
    } else if n % 2 == 1 {
        let next = u64::checked_mul(3, n);
        match next {
            Some(next) => {
                let next = u64::checked_add(next, 1);
                match next {
                    Some(next) => {
                        let next_step = collatz(next);
                        match next_step {
                            Some(y) => Some(y + 1),
                            None => None,
                        }
                    }
                    None => return None,
                }
            }
            None => return None,
        }
    } else {
        return None;
    }
}
