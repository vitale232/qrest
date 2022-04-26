pub mod cli;

use std::fmt::{self, Display};

use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Query Types
#[derive(Clone, Debug, Serialize, Deserialize, ArgEnum)]
pub enum ResponseFormat {
    Html,
    Json,
    Geojson,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    #[serde(skip_serializing)]
    query_url: String,
    pub f: ResponseFormat,
    #[serde(rename = "where")]
    pub where_clause: String,
    #[serde(rename = "returnGeometry")]
    pub return_geometry: bool,
    #[serde(rename = "returnCountOnly")]
    pub return_count_only: bool,
    #[serde(rename = "outFields")]
    pub out_fields: String,
}

impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let q = serde_urlencoded::to_string(self).unwrap_or_default();
        write!(f, "{}?{}", self.query_url, q)
    }
}

pub struct QueryBuilder {
    query_url: String,
    response_format: Option<ResponseFormat>,
    where_clause: Option<String>,
    return_geometry: Option<bool>,
    return_count_only: Option<bool>,
    out_fields: Option<Vec<String>>,
}

impl QueryBuilder {
    pub fn new(query_url: &str) -> Self {
        Self {
            query_url: query_url.to_string(),
            response_format: None,
            where_clause: None,
            return_geometry: None,
            return_count_only: None,
            out_fields: None,
        }
    }

    pub fn response_format(&mut self, format: ResponseFormat) -> &mut Self {
        self.response_format = Some(format);
        self
    }

    pub fn where_clause(&mut self, wc: &str) -> &mut Self {
        self.where_clause = Some(wc.to_string());
        self
    }

    pub fn return_geometry(&mut self, return_geo: bool) -> &mut Self {
        self.return_geometry = Some(return_geo);
        self
    }

    pub fn return_count_only(&mut self, return_count: bool) -> &mut Self {
        self.return_count_only = Some(return_count);
        self
    }

    pub fn out_fields(&mut self, fields: Vec<&str>) -> &mut Self {
        self.out_fields = Some(fields.clone().iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn build(&mut self) -> Query {
        Query {
            query_url: self.query_url.clone(),
            f: self.response_format.clone().unwrap_or(ResponseFormat::Json),
            where_clause: self
                .where_clause
                .clone()
                .unwrap_or_else(|| "1=1".to_string()),
            return_geometry: self.return_geometry.unwrap_or_default(),
            return_count_only: self.return_count_only.unwrap_or_default(),
            out_fields: self
                .out_fields
                .clone()
                .unwrap_or_else(|| vec![String::from("")])
                .join(","),
        }
    }
}

// Return Types
#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
    alias: String,
    #[serde(rename = "sqlType")]
    sql_type: Option<String>,
    length: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    attributes: Value,
    geometry: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpatialReference {
    wkid: u64,
    #[serde(rename = "latestWkid")]
    latest_wkid: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    #[serde(rename = "objectIdFieldName")]
    oid_field_name: String,
    #[serde(rename = "globalIdFieldName")]
    guid_field_name: String,
    #[serde(rename = "geometryType")]
    geometry_type: String,
    #[serde(rename = "spatialReference")]
    spatial_reference: SpatialReference,
    #[serde(rename = "hasZ")]
    has_z: Option<bool>,
    #[serde(rename = "hasM")]
    has_m: Option<bool>,
    fields: Vec<Field>,
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountQueryResponse {
    count: u128,
}
