use serde_json::Value;
use structs::trs::Auth;

#[derive(Default, Clone, Serialize, Deserialize)]
struct GeneratedType {
    transaction_id: String,
    processed: Processed,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct Processed {
    id: String,
    receipt: Receipt,
    elapsed: i64,
    net_usage: i64,
    scheduled: bool,
    action_traces: Vec<ActionTrace>,
    except: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct Receipt {
    status: String,
    cpu_usage_us: i64,
    net_usage_words: i64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct ActionTrace {
    receipt: AReceipt,
    act: Act,
    elapsed: i64,
    cpu_usage: i64,
    console: String,
    total_cpu_usage: i64,
    trx_id: String,
    inline_traces: Vec<InlineTrace>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct AReceipt {
    receiver: String,
    act_digest: String,
    global_sequence: i64,
    recv_sequence: i64,
    auth_sequence: Vec<Vec<Value>>,
    code_sequence: i64,
    abi_sequence: i64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct Act {
    account: String,
    name: String,
    authorization: Vec<Auth>,
    data: Value,
    hex_data: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct InlineTrace {
    receipt: AReceipt,
    act: Act,
    elapsed: i64,
    cpu_usage: i64,
    console: String,
    total_cpu_usage: i64,
    trx_id: String,
    inline_traces: Vec<InlineTrace>,
}