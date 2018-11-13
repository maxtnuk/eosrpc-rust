extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use serde_json::Value;
use serde::ser::Serialize;
use std::fs::{self, File};
use std::collections::HashMap;
use std::path::PathBuf;

use reqwest::Error;

#[cfg(test)]
mod test;

type Method = HashMap<String, Argu>;

#[derive(Clone)]
pub struct ApiConfig<'a> {
    pub http_endpoint: &'a str,
    pub verbose: bool,
    pub debug: bool,
    pub broadcast: bool,
    pub sign: bool,
    pub chain_id: &'a str,
}
impl<'a> Default for ApiConfig<'a> {
    fn default() -> Self {
        ApiConfig {
            http_endpoint: "http://127.0.0.1:8888",
            verbose: false,
            debug: false,
            broadcast: true,
            sign: true,
            chain_id: "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Argu {
    #[serde(default)]
    brief: String,
    params: Value,
    results: Value,
}
#[derive(Clone)]
pub struct EosApi<'a> {
    def: HashMap<String, Method>,
    pub optional: ApiConfig<'a>,
}
impl<'a> EosApi<'a> {
    pub fn new(config: ApiConfig<'a>) -> Self {
        let mut apis: HashMap<String, Method> = HashMap::new();

        for entry in fs::read_dir("./eos-api/src/apis").unwrap() {
            let entry = entry.unwrap();
            let version_str = entry
                .path()
                .as_path()
                .iter()
                .skip(4)
                .collect::<PathBuf>()
                .to_string_lossy()
                .into_owned();
            for api_file in fs::read_dir(entry.path()).unwrap() {
                let api_file = api_file.unwrap();
                let file_name = api_file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let api_path = version_str.clone() + "/" + &file_name;

                //println!("{}",api_path);

                let file = File::open(api_file.path()).unwrap();
                let u: Method = serde_json::from_reader(file).unwrap();
                apis.insert(api_path, u);
            }
        }

        EosApi {
            def: apis,
            optional: config,
        }
    }
    pub fn http_request<S>(&self, name: &str, body: &S) -> Result<Value, Error>
    where
        S: Serialize,
    {
        let httpurl = self.optional.http_endpoint;
        let mut url = String::new();
        for (k, v) in self.def.iter() {
            if v.get(&name.to_string()).is_some() {
                url = format!("{}/{}/{}", httpurl, k, name);
                break;
            }
        }
        //println!("url: {}",url);
        let res = reqwest::Client::new().post(&url).json(body).send()?.json()?;
        Ok(res)
    }
}
