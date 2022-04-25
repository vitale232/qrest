use std::process;

use qrest::cli::{self, count_query_get, feature_query_get};

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

    if !query.return_count_only {
        let response = feature_query_get(query).await?;
        let result = serde_json::to_string(&response).unwrap_or_else(|err| {
            eprintln!("Could not parse the server response!:\n{}", err);
            process::exit(1);
        });
        println!("{:#?}", result);
    } else {
        let response = count_query_get(query).await?;
        let result = serde_json::to_string(&response).unwrap_or_else(|err| {
            eprintln!("Could not parse the server response!:\n{}", err);
            process::exit(1);
        });
        println!("{:#?}", result);
    }

    Ok(())
}
