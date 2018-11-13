use serde_json::Value;
#[derive(Serialize, Deserialize)]
pub struct Regions {
    pub region: i64,
    pub cycles_summary: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockInfo {
    pub action_mroot: String,
    #[serde(default)]
    pub block_extensions: Vec<Value>,
    pub block_num: u64,
    pub confirmed: u64,
    #[serde(default)]
    pub header_extensions: Vec<Value>,
    pub id: String,
    #[serde(default)]
    pub new_producers: Option<String>,
    pub previous: String,
    pub producer: String,
    pub producer_signature: String,
    pub ref_block_prefix: u64,
    pub schedule_version: u64,
    pub timestamp: String,
    #[serde(default)]
    pub transaction_mroot: String,
    #[serde(default)]
    pub transactions: Vec<Value>,
}
