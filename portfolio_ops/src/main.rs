// Using a hash map and vectors, create a text interface to allow a user to
// add ticker symbols to a portfolio in a fund. For example, “Add AAPL to
// Alpha Fund I” or “Add GILD to Global Value Fund II.”
// Then let the user retrieve a list of all tickers in a portfolio or all
// tickers in the fund by portfolio name, sorted alphabetically.
use crate::portfolio_operations::add_to_portfolio;
use crate::portfolio_operations::list_all_portfolios;
use crate::portfolio_operations::list_portfolio_tickers;
use std::collections::HashMap;

pub mod portfolio_operations;

fn main() {
    let mut portfolios: HashMap<String, Vec<String>> = HashMap::new();
    println!("{:?}", portfolios);
    add_to_portfolio(
        String::from("AAPL"),
        String::from("Bland Growth"),
        &mut portfolios,
    );
    println!("{:?}", portfolios);
    add_to_portfolio(
        String::from("GILD"),
        String::from("Bland Growth"),
        &mut portfolios,
    );
    println!("{:?}", portfolios);
    add_to_portfolio(
        String::from("GILD"),
        String::from("Bland Growth"),
        &mut portfolios,
    );
    println!("{:?}", portfolios);

    add_to_portfolio(
        String::from("TSLA"),
        String::from("Money-losing Growth"),
        &mut portfolios,
    );
    println!("{:?}", portfolios);

    list_portfolio_tickers(String::from("Money-losing Growth"), &mut portfolios);

    list_portfolio_tickers(String::from("Bland Growth"), &mut portfolios);

    list_all_portfolios(&mut portfolios);
}
