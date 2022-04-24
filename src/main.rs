use qrest::{CountQueryResponse, Query, QueryResponse, ResponseFormat};
use reqwest::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let query_url = String::from(
        "https://gisservices.its.ny.gov/arcgis/rest/services/NYS_Place_Points/FeatureServer/0/query",
    );
    let where_clause = String::from("County = 'Essex' AND PlaceType = 'Incorporated Town'");

    let feature_query = Query::new()
        .where_clause(&where_clause)
        .return_geometry(true)
        .out_fields(vec!["NAME", "County", "PlaceType"])
        .build();

    let url = feature_query.append_params(&query_url);
    let features: QueryResponse = Client::new().get(&url).send().await?.json().await?;

    println!("FEATURE QUERY\n{}\n{:#?}", feature_query, features);

    let count_query = Query::new()
        .where_clause(&where_clause)
        .return_count_only(true)
        .response_format(ResponseFormat::Json)
        .build();

    let count: CountQueryResponse = Client::new()
        .get(count_query.append_params(&query_url))
        .send()
        .await?
        .json()
        .await?;

    println!(
        "\nCOUNT QUERY\n{}\n{:#?}",
        count_query.append_params(&query_url),
        count
    );

    Ok(())
}
