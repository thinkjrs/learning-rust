fn main() {
    //let x = 5;
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // change to ThREE_... and look at hover
    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    println!("The constant THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    // shadowing
    let y = 4;
    println!("The value of y is: {y}");

    let y = y + 1;
    println!("The value of y + 1 is: {y}");

    {
        let y = y * 2;
        println!("The value of y * 2 in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");
}
