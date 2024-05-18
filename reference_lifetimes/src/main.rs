//fn failing_lifetime_function<'a>(x: &i32,) -> &'a i32 {
//    let result: i32 = 42;
//    &result
//}
fn bigger_sum<'a>(first: &'a Vec<i32>, second: &'a Vec<i32>) -> &'a Vec<i32> {
    // find sums of each
    let sum_first: i32 = first.iter().sum();
    let sum_second: i32 = second.iter().sum();
    // return vector with larger sum
    if sum_first > sum_second {
        println!("The first vector has a larger sum: {}", sum_first);
        first
    } else {
        println!("The second vector has a larger sum: {}", sum_second);
        second
    }
}
fn use_bigger_sum() {
    let first: Vec<i32> = vec![1, 2, 3, 4];
    let second: Vec<i32> = vec![-1, 2, 3, 4, 5];

    bigger_sum(&first, &second);

    {
        let third: Vec<i32> = vec![2, 3, 4, 5];
        bigger_sum(&second, &third);
    }

    // notice how the below will not compile when uncommented
    //let should_be_second: &Vec<i32>;
    //{
    //    let forth: Vec<i32> = vec![2, 3, 4];
    //    should_be_second = bigger_sum(&second, &forth);
    //}
    //println!("{:?}", should_be_second);
}
//fn dangling_reference() {
//    let first;
//
//    {
//        let first_second = "Hello";
//        first = &first_second;
//    }
//    println!("{}, world!", first);
//}

fn main() {
    let first;

    let first_second = "Hello";

    first = &first_second;

    println!("{}, world!", first);
    let first: Vec<i32> = vec![1, 2, 3, 4];
    let second: Vec<i32> = vec![-1, 2, 3, 4];

    bigger_sum(&first, &second);

    let third: Vec<i32> = vec![2, 3, 4, 5];
    bigger_sum(&second, &third);

    use_bigger_sum();
}
