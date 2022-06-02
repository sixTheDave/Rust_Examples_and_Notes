// [dependencies]
// reqwest = "0.11"

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://api.coindesk.com/v1/bpi/currentprice.json")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
