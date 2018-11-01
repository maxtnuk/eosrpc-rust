#[derive(Serialize, Deserialize)]
pub struct ABI {
  version: String,
  types: Vec<Types>,
  #[serde(rename = "____comment")]
  comment: String,
  structs: Vec<Structs>,
  actions: Vec<Actions>,
  tables: Vec<Tables>,
  ricardian_clauses: Vec<String>,
  abi_extensions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Actions {
  name: String,
  #[serde(rename = "type")]
  _type: String,
  ricardian_contract: String,
}

#[derive(Serialize, Deserialize)]
struct Fields {
  name: String,
  #[serde(rename = "type")]
  _type: String,
}

#[derive(Serialize, Deserialize)]
struct Structs {
  name: String,
  base: String,
  fields: Vec<Fields>,
}

#[derive(Serialize, Deserialize)]
struct Tables {
  name: String,
  #[serde(rename = "type")]
  _type: String,
  index_type: String,
  key_names: Vec<String>,
  key_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Types {
  new_type_name: String,
  #[serde(rename = "type")]
  _type: String,
}

