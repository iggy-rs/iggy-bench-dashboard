use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InfluxResponse {
    pub results: Vec<ResultItem>,
}

#[derive(Debug, Deserialize)]
pub struct ResultItem {
    pub series: Option<Vec<Series>>,
}

#[derive(Debug, Deserialize)]
pub struct Series {
    pub values: Vec<Vec<serde_json::Value>>,
}
