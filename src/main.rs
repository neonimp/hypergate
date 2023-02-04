use std::error::Error;

#[derive(Parser, Debug)]
#[command()]
struct Cli {

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

}
