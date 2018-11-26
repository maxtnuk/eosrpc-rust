use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EosError{
    pub code: i64,
    pub error: ErrorContent,
    pub message: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorContent{
    pub code: u64,
    pub details: Vec<Value>,
    pub name: String,
    pub what: String
}