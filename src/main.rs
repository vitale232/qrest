use qrest::{CountQueryResponse, Query, QueryResponse, ResponseFormat};
use reqwest::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service_url = String::from(
        "https://gisservices.its.ny.gov/arcgis/rest/services/NYS_Place_Points/FeatureServer/0/query",
    );
    let where_clause = String::from("County = 'Essex' AND PlaceType = 'Incorporated Town'");

    let feature_query = Query::new()
        .where_clause(&where_clause)
        .return_geometry(true)
        .out_fields(vec!["NAME", "County", "PlaceType"])
        .build();
    let feature_request_url = format!("{}?{}", service_url, feature_query);

    let features: QueryResponse = Client::new()
        .get(feature_request_url)
        .send()
        .await?
        .json()
        .await?;

    println!("Feature Query\n  ({}):\n{:#?}", feature_query, features);

    let count_query = Query::new()
        .where_clause(&where_clause)
        .return_count_only(true)
        .response_format(ResponseFormat::Json)
        .build();

    let count_request_url = format!("{}?{}", service_url, count_query);

    let count: CountQueryResponse = Client::new()
        .get(count_request_url)
        .send()
        .await?
        .json()
        .await?;

    println!("\nFeature Count Query\n  ({}):\n{:#?}", count_query, count);

    Ok(())
}
