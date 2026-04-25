fn main() {
    println!("Hello, world!");
}

// Ten green bottles hanging on the wall,
// Ten green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be nine green bottles hanging on the wall.

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();

    for _ in 0..take_down {
        result.push_str(num_words_upper(start_bottles));
        result.push_str(" hanging on the wall,\n");
        result.push_str(num_words_upper(start_bottles));
        result.push_str(" hanging on the wall,\n");

        result.push_str("And if one green bottle should accidentally fall,\n");
        result.push_str("There'll be ");
        result.push_str(num_words_lower(start_bottles));
        result.push_str(" hanging on the wall.\n\n");
    }

    result
}

pub fn num_words_upper(n: u32) -> &'static str {
    match n {
        10 => "Ten green bottles",
        9 => "Nine green bottles",
        8 => "Eight green bottles",
        7 => "Seven green bottles",
        6 => "Six green bottles",
        5 => "Five green bottles",
        4 => "Four green bottles",
        3 => "Three green bottles",
        2 => "Two green bottles",
        1 => "One green bottle",
        _ => panic!("Invalid number"),
    }
}

pub fn num_words_lower(n: u32) -> &'static str {
    match n {
        10 => "ten green bottles",
        9 => "nine green bottles",
        8 => "eight green bottles",
        7 => "seven green bottles",
        6 => "six green bottles",
        5 => "five green bottles",
        4 => "four green bottles",
        3 => "three green bottles",
        2 => "two green bottles",
        1 => "one green bottles",
        0 => "no green bottles",
        _ => panic!("Invalid number"),
    }
}
