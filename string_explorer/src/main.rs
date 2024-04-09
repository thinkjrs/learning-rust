fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);
    //println!("{}", s1);
    let mut s = format!("{s3} You are crazy, {s2}");
    println!("{}", s);
    // Borrow a reference to the entire String as a slice
    let slice_entire = &s3[..];

    // Borrow a reference to part of the String
    let slice_part = &s3[0..5];

    println!("Entire slice: {}", slice_entire); // Prints "Hello, world!"
    println!("Part of slice: {}", slice_part); // Prints "Hello"
   
    let not_owned = "blah";
    s.push_str(not_owned);
    println!("Pushed: {}", s);
    println!("This isn't owned: {}", not_owned);

    for c in not_owned.chars() {
        println!("Character: {}", c);
    }
    
    for b in not_owned.bytes() {
        println!("Bytes: {}", b);
    }
}
