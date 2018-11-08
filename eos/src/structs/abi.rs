#[derive(Serialize, Deserialize)]
pub struct ABI {
  pub version: String,
  pub types: Vec<Types>,
  #[serde(rename = "____comment")]
  #[serde(default)]
  pub comment: String,
  pub structs: Vec<Structs>,
  pub actions: Vec<Actions>,
  pub tables: Vec<Tables>,
  pub ricardian_clauses: Vec<String>,
  pub abi_extensions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Actions {
  pub name: String,
  #[serde(rename = "type")]
  _type: String,
  pub ricardian_contract: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fields {
  name: String,
  #[serde(rename = "type")]
  _type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Structs {
  name: String,
  base: String,
  fields: Vec<Fields>,
}

#[derive(Serialize, Deserialize)]
pub struct Tables {
  #[serde(default)]
  name: String,
  #[serde(rename = "type")]
  #[serde(default)]
  _type: String,
  #[serde(default)]
  index_type: String,
  #[serde(default)]
  key_names: Vec<String>,
  #[serde(default)]
  key_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Types {
  new_type_name: String,
  #[serde(rename = "type")]
  _type: String,
}



