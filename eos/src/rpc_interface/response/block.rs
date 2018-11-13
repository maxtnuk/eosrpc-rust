use serde_json::Value;
#[derive(Serialize, Deserialize)]
pub struct Regions {
    pub region: i64,
    pub cycles_summary: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockInfo {
    pub previous: String,
    pub timestamp: String,
    pub transaction_mroot: String,
    pub action_mroot: String,
    pub block_mroot: String,
    pub producer: String,
    pub schedule_version: i64,
    pub new_producers: String,
    pub producer_signature: String,
    pub regions: Vec<Regions>,
    pub input_transactions: Vec<Value>,
    pub id: String,
    pub block_num: u64,
    pub ref_block_prefix: u64,
}
