fn main() {
    println!("{:#?}", nth(6));
}

pub fn nth(n: u32) -> u32 {
    // let mut primes: Vec<u32> = Vec::new();
    // let mut num = 2;
    // for i in 0..=n {
    //
    // }
    // for i in 3..=n {
    //     if i % 2 != 0 && is_prime(i){
    //         prime_collection.push(i);
    //     }
    // }
    // return prime_collection;

    // while primes.len < n {
    //     if is_prime(num) {
    //         primes.push(num);
    //     }
    //     num += 1;
    // }
    if n == 0 {
        return 2;
    }
    let mut count = 1;
    let mut candidate = 1;

    loop {
        candidate += 2; // only check odd numbers, step 2

        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
    }
}


pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
