use clap::Parser;
use reqwest::{Client, Error};
use std::io::ErrorKind;

use crate::{CountQueryResponse, Query, QueryBuilder, QueryResponse, ResponseFormat};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    query_url: String,

    #[clap(
        short,
        long,
        value_name = "where",
        help = "Optional where clause. Defaults to `1=1`"
    )]
    _where: Option<String>,

    #[clap(
        short,
        long,
        help = "Optional comma-separated list of fields to return"
    )]
    out_fields: Option<String>,

    #[clap(short, long, help = "Optional flag, return the count of features")]
    count: bool,

    #[clap(short, long, help = "Optional flag, return the geometry")]
    geometry: bool,

    #[clap(arg_enum, short, long, default_value_t = ResponseFormat::Json)]
    format: ResponseFormat,
}

pub fn run() -> Result<Query, Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Validate the URL
    if !cli.query_url.ends_with("/query") & !cli.query_url.ends_with("/query/") {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidInput,
            "The URL does not appear to be a query endpoint!",
        )));
    }

    // Build the Query
    let mut qb = QueryBuilder::new(&cli.query_url);
    qb.response_format(cli.format);
    qb.return_count_only(cli.count);
    qb.return_geometry(cli.geometry);

    if let Some(wc) = cli._where {
        qb.where_clause(&wc);
    }

    if let Some(out_fields) = cli.out_fields {
        let fields: Vec<&str> = out_fields.split(',').collect();
        qb.out_fields(fields);
    }

    Ok(qb.build())
}

pub async fn feature_query_get(query: Query) -> Result<QueryResponse, Error> {
    let features: QueryResponse = Client::new()
        .get(query.to_string())
        .send()
        .await?
        .json()
        .await?;
    Ok(features)
}

pub async fn count_query_get(query: Query) -> Result<CountQueryResponse, Error> {
    let count: CountQueryResponse = Client::new()
        .get(query.to_string())
        .send()
        .await?
        .json()
        .await?;
    Ok(count)
}

pub async fn fetch_query(query: Query) -> Result<serde_json::Value, Error> {
    let res = Client::new()
        .get(query.to_string())
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn fetch_html_query(query: Query) -> Result<String, Error> {
    let res = Client::new()
        .get(query.to_string())
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
