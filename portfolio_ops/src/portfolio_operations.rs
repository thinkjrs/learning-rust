use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn add_to_portfolio(
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
                    println!(
                        "Ticker {} already in portfolio so stop trying to add it.",
                        &ticker
                    );
                }
            }
        }
    }
}

pub fn list_portfolio_tickers(
    portfolio_name: String,
    portfolios: &mut HashMap<String, Vec<String>>,
) {
    if let Some(portfolio_tickers) = portfolios.get(&portfolio_name) {
        println!("Tickers for {}: {:?}", portfolio_name, portfolio_tickers);
    }
}

pub fn list_all_portfolios(portfolios: &mut HashMap<String, Vec<String>>) {
    println!("All fund portfolios and holdings: {:?}", portfolios);
}
