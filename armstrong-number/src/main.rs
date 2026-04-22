fn main() {
    // let result = int_to_vec(123);
    // println!("{:?}", result.len());

    if armstrong(9) {
        println!("It's an armstrong number");
    } else {
        println!("It's not an armstrong number");
    }
}

fn armstrong(num: u32) -> bool {
    let mut n: u32 = num;
    let mut counter: u32 = 0;
    // count digits
    while n > 0 {
        counter += 1;
        n /= 10;
    }

    let mut n: u32 = num;
    let mut sum: u32 = 0;

    while n > 0 {
        let digit = n % 10; // get the digit
        sum += digit.pow(counter);
        n /= 10;
    }
    sum == num
}

// make collection of number
// fn int_to_vec(n: u32) -> Vec<u32> {
//     let mut num: u32 = n;
//     let mut result: Vec<u32> = Vec::new();
//
//     while num != 0 {
//         result.push(num % 10);
//         num = num / 10;
//     }
//     result
// }
