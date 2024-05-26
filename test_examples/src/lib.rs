pub fn multiply(lhs: &i32, rhs: &i32) -> i32 {
    lhs * rhs
}

fn a_function_using_result() -> Result<bool, String> {
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&2, &2), 4);
    }

    #[test]
    fn this_is_not_equal() {
        // this will pass
        assert_ne!(multiply(&2, &2), 5);
    }

    #[test]
    fn this_is_true() {
        // this will pass
        assert!(multiply(&2, &2) == 4);
    }

    #[test]
    fn test_using_result() -> Result<(), String> {
        let ran_successfully = a_function_using_result()?; // Err value will fail here

        if ran_successfully {
            Ok(())
        } else {
            Err("Something terrible happened and we got an error. Spam your own phone number, developer, until someone picks up. Yell at them.".into())
        }
    }

    #[test]
    fn this_is_not_true() {
        // this will not pass
        assert!(multiply(&2, &2) == 5, "2 * 2 != 5");
    }
}
