fn multiply(lhs: &i32, rhs: &i32) -> i32 {
    lhs * rhs
}

fn main() {
    let result = multiply(&2, &2);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&2, &2), 4);
    }
}
