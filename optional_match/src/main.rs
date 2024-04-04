fn did_eat_fruit(fruit: Option<&str>) -> bool {
    match fruit {
        None => false,
        _ => true,
    }
}
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()        
    }
}
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone()
    }
}
fn main() {
    let apple = Some("Apple");
    let banana = None;
    let monkey_eating_status = if did_eat_fruit(apple) {
        "ate"
    } else {
        "did not eat"
    };
    println!("The monkey {monkey_eating_status}.");

    let monkey_eating_status = if did_eat_fruit(banana) {
        "ate"
    } else {
        "did not eat"
    };
    println!("The monkey {monkey_eating_status}.");
    
}
