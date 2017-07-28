
// Not the best way to find primes, but it follows the rule

pub fn primes_up_to(limit: i32) -> Vec<i32> {
    let mut sieve = (2..limit+1).collect::<Vec<_>>();

    for n in 2..limit+1 {
        sieve = sieve.into_iter()
            .filter_map(|x| if x > n && x % n == 0 { None } else { Some(x) })
            .collect::<Vec<_>>();
    }

    sieve
}
