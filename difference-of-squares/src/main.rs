fn main() {
    println!("{}", square_of_sum(10));
    println!("{}", sum_of_squares(10));

    println!("{}", difference(10));
}


pub fn square_of_sum(n: u32) -> u32 {
    // todo!("square of sum of 1...{n}")
    let mut num = n;
    let mut sum = 0;

    while num > 0 {
        sum += num;
        num -= 1;
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    let mut num = n;
    let mut sum = 0;

    while num > 0 {
        sum += num.pow(2);
        num -= 1;
    }
    sum
}


pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);

    square_of_sum - sum_of_squares
}
