// Using a hash map and vectors, create a text interface to allow a user to
// add ticker symbols to a portfolio in a fund. For example, “Add AAPL to
// Alpha Fund I” or “Add GILD to Global Value Fund II.”
// Then let the user retrieve a list of all tickers in a portfolio or all
// tickers in the fund by portfolio name, sorted alphabetically.
use std::collections::hash_map::Entry;
use std::collections::HashMap;

// TODO implement adding a stock to a portfolio
fn add_to_portfolio(
    ticker: String,
    portfolio_name: String,
    portfolios: &mut HashMap<String, Vec<String>>,
) {
    match portfolios.entry(portfolio_name) {
        Entry::Vacant(e) => {
            e.insert(vec![ticker]);
        }
        Entry::Occupied(mut e) => {
            let mut contains_ticker: bool = false;
            for portfolio_ticker in e.get() {
                if portfolio_ticker == &ticker {
                    contains_ticker = true;
                }
            }
            match contains_ticker {
                false => {
                    e.get_mut().push(ticker);
                }
                true => {
                    println!("Ticker {} already in portfolio so stop trying to add it.", &ticker);
                }
            }
        }
    }
}
// TODO implement listing a portfolio's tickers
// TODO implement listing all tickers in the fund by portfolio name
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

}
