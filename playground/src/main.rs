fn main() {
    // immutable and mutable references
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2} are {s}");

    let r3 = &mut s;
    println!("{r3} is {r3}");
    //println!("{r3} is {s}"); // compiler error: cannot borrow `s` as mutable

    // Slice example
    let s = String::from("hello world");

    let hello = first_word(&s);

    let world = second_word(&s);

    println!("{hello} {world}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}
