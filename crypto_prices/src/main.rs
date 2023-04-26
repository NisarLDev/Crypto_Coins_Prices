use serde::{Deserialize, Serialize};

fn main() {
    let mut coin = String::new();
    println!("What cryptocurrency do you want to consult?");
    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("An error occurred reading stdin");

    let result_price = get_price(&coin);
    match result_price {
        Ok(price) => println!("The price is: ${}", price),
        Err(error) => println!(" Error searching for price: {}", error),
    }
}

fn get_price(coin: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin
    ))
    .call()?
    .into_string()?;
    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    Ok(coin_data.market_data.current_price.usd.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
    clp: f32,
}
