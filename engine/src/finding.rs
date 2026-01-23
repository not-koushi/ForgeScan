use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize)]
pub struct Finding {
    pub category: String,
    pub severity: Severity,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub similar_to: Option<String>,
}