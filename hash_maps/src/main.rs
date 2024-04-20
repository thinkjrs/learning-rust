use std::collections::HashMap;

fn main() {
    let mut prices = HashMap::new();
    
    let stock_ticker_1 = "AAPL";

    prices.insert(stock_ticker_1, 163.23);
    prices.insert("GILD", 66.76);

    let ticker_symbol = "GILD";
    let gild_price = prices.get(&ticker_symbol).copied().unwrap_or(0.0);

    println!("{}: {}", ticker_symbol, gild_price);
    
    prices.insert("GILD", 66.77);

    for ( key, value ) in &prices {
        println!("{}: {}", key, value);
    }
    

    //prices.insert(String::from("TSLA"), 0.01); will error
    let mut prices_strings = HashMap::new();
    
    let stock_ticker_1_moveable = String::from("AAPL");
    prices_strings.insert(stock_ticker_1_moveable, 163.23);
    prices_strings.insert(String::from("AAPL"), 163.23);

    // trying to user stock_ticker_1_moveable results in an error "borrow of moved value"
    //println!("{}", stock_ticker_1_moveable);
    // however we can use the "copied" value from before 
    println!("copied ticker {}", stock_ticker_1);

    // let's update this bad boy 
    prices_strings.insert("AAPL".to_string(), 162.01);

    for ( key, value ) in &prices_strings {
        println!("{}: {}", key, value);
    }
    // update if exists otherwise ad value      
    prices_strings.entry("TSLA".to_string()).or_insert(0.02);
    for ( key, value ) in &prices_strings {
        println!("{}: {}", key, value);
    }

    // update if exists and add a penny
    let tsla_price = prices_strings.entry("TSLA".to_string()).or_insert(0.02);
    *tsla_price += 0.01;
    println!("TSLA: {}", tsla_price);
}
