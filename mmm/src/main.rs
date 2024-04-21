fn median(vector: &mut Vec<i32>) {
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

fn mode(vector: &Vec<i32>) {
    use std::collections::HashMap;
    let mut counter: HashMap<&i32, i32> = HashMap::new();
    for val in vector {
        let count = counter.entry(val).or_insert(0);
        *count += 1;
    }

    let mut max_val: i32 = 0;
    let mut max_key: i32 = 0;
    for (key, val) in counter.iter() {
        if val > &max_val {
            max_key = **key;
            max_val = *val;
        }
    }
    println!("{:?}", counter);
    println!("Mode: {}", max_key);
}
fn main() {
    let mut rates: Vec<i32> = vec![0, 1, 0, 12, 3, 49, 28];
    println!("Calculating median, mode and means for {:?}", rates);
    // find median
    median(&mut rates);
    // find mode

    mode(&mut rates);
    // find arithmetic mean

    // find geometric mean

    rates = vec![0, 1, 5, 5, 49, 28];
    println!("Calculating median, mode and means for {:?}", rates);
    median(&mut rates);

    // find mode
    mode(&mut rates);
    // find arithmetic mean

    // find geometric mean
}
