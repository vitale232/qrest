use clap::{arg, Command};
use reqwest::{Client, Error};
use std::io::ErrorKind;

use crate::{CountQueryResponse, Query, QueryBuilder, QueryResponse};

pub fn run() -> Result<Query, Box<dyn std::error::Error>> {
    let matches = Command::new("qrest")
        .version("0.1.0")
        .author("Andrew Vitale <vitale232@gmail.com>")
        .about("Query an ArcGIS REST FeatureService Layer")
        .arg(arg!([query_url] "The Service URL's /query endpoint").required(true))
        .arg(arg!(-w --where <CLAUSE> ... "Where clause to use in query").required(false))
        .arg(arg!(-g - -geometry "Return geometry with response").required(false))
        .arg(arg!(-c - -count "Return Count only, in lieu of features").required(false))
        .arg(arg!(-o --fields <FIELDS> "Out fields, the list of fields to be returned by the server (comma separated)").required(false))
        .get_matches();

    let url = matches
        .value_of("query_url")
        .ok_or("The Service URL is required")?;

    if !url.ends_with("/query") & !url.ends_with("/query/") {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidInput,
            "The URL does not appear to be a query endpoint!",
        )));
    }

    let mut qb = QueryBuilder::new(url);

    if let Some(wc) = matches.value_of("where") {
        qb.where_clause(wc);
    }

    if matches.is_present("geometry") {
        qb.return_geometry(true);
    }

    if matches.is_present("count") {
        qb.return_count_only(true);
    }

    if let Some(out_fields) = matches.value_of("fields") {
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
