#[derive(Debug)]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct Finding {
    pub file: String,
    pub entropy: f64,
    pub severity: Severity,
}