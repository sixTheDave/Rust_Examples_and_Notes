use http::{Request, Response};

fn main() {
let mut request = Request::builder();
request.uri("https://api.coindesk.com/v1/bpi/currentprice.json")
       .header("User-Agent", "my-awesome-agent/1.0");

println!("{:#?}", request)
}

