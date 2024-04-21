fn median(mut vector: Vec<i32>) {
    match vector.len() {
        0 => println!("Median: NaN"),
        _other => {
            // sort the vector
            vector.sort();
            match vector.len() % 2 {
                1 => {
                    // return middle
                    let middle = vector.len() / 2;
                    println!("Median: {}", &vector[middle])
                }
                0 => {
                    // return arithmetic average of middle two
                    let middle_upper_index = vector.len() / 2;
                    let middle_lower_index = &middle_upper_index - 1;
                    let median_value =
                        (&vector[middle_lower_index] + &vector[middle_upper_index]) / 2;
                    println!("Median: {}.5", median_value)
                }
                _ => {}
            }
        }
    }
}
fn main() {
    let mut rates: Vec<i32> = vec![0, 1, 0, 12, 3, 49, 28];
    println!("Calculating median, mode and means for {:?}", rates);
    // find median
    median(rates);
    // find mode

    // find arithmetic mean

    // find geometric mean

    rates = vec![0, 1, 12, 5, 49, 28];
    println!("Calculating median, mode and means for {:?}", rates);
    median(rates);

    // find mode

    // find arithmetic mean

    // find geometric mean
}
