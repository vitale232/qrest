use std::process;

use qrest::cli::{self, fetch_query};

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let query = cli::run().unwrap_or_else(|err| {
        eprintln!(
            "Could not generate a query URL from the input parameters! An error occurred:\n{}",
            err
        );
        process::exit(1);
    });

    let response = fetch_query(query).await?;

    let output = serde_json::to_string_pretty(&response).unwrap_or_else(|err| {
        eprintln!("Could not parse the server response!:\n{}", err);
        process::exit(1);
    });

    println!("{}", output);

    Ok(())
}
