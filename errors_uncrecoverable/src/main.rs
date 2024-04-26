#[allow(unconditional_panic)]
fn main() {
    //panic!("Crash the program!");
    let y = 0;
    let should_panic = 1 / y;

    println!("{}", should_panic);
}
