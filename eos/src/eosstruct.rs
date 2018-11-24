use eosformat;
/*
!!!!!todo
Name o
Symbol
Symbolcode
ExtemdedSymbol
SignatureType
*/
struct Name {
    nameu64: u64,
}
impl Name {
    fn origin_name(&self) -> String {
        let u64_str = format!("{}", &self.nameu64);
        eosformat::decode_name(u64_str, false)
    }
    fn de_name(&self) -> u64 {
        self.nameu64
    }
}
impl From<String> for Name {
    fn from(name: String) -> Self {
        let value = eosformat::encode_name(name, false).unwrap();
        Name { nameu64: value.as_str().parse().unwrap() }
    }
}
impl<'a> From<&'a str> for Name {
    fn from(name: &'a str) -> Self {
        Self::from(name.to_string())
    }
}
impl Default for Name {
    fn default() -> Self {
        Self::from("")
    }
}

struct Symbol {
    symbol_code: String,
    precision: Option<usize>,
}
impl Symbol {
    fn get_symbol(&self) -> String {
        self.symbol_code.clone()
    }
    fn get_precision(&self) -> Option<usize> {
        self.precision.clone()
    }
}
impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self.precision {
            Some(e) => format!("{},{}", e, self.symbol_code),
            None => format!("{}", self.symbol_code),
        }
    }
}
impl Default for Symbol {
    fn default() -> Self {
        Symbol {
            symbol_code: "SYS".to_string(),
            precision: None,
        }
    }
}
impl From<String> for Symbol {
    fn from(txt: String) -> Self {
        let result = eosformat::parse_asset(txt);
        Symbol {
            symbol_code: result.symbol.clone(),
            precision: result.precision.clone(),
        }
    }
}

struct ExtemdedSymbol {
    symbol: Symbol,
    contract: Option<String>,
}
impl ToString for ExtemdedSymbol {
    fn to_string(&self) -> String {
        format!(
            "{}@{}",
            self.symbol.symbol_code,
            self.contract.clone().unwrap()
        )
    }
}
impl From<String> for ExtemdedSymbol {
    fn from(txt: String) -> Self {
        let result = eosformat::parse_asset(txt);
        ExtemdedSymbol {
            symbol: Symbol {
                symbol_code: result.symbol.clone(),
                precision: result.precision.clone(),
            },
            contract: result.contract.clone(),
        }
    }
}
impl Default for ExtemdedSymbol {
    fn default() -> Self {
        ExtemdedSymbol {
            contract: Some("contract".to_string()),
            symbol: Default::default(),
        }
    }
}

struct EosAsset {
    pub amount: Option<String>,
    pub symbol: Symbol,
}
impl From<String> for EosAsset {
    fn from(txt: String) -> Self {
        let result = eosformat::parse_asset(txt);
        let amount = eosformat::decimal_pad(result.amount.unwrap(), result.precision.clone());
        EosAsset {
            amount: Some(amount),
            symbol: Symbol {
                symbol_code: result.symbol.clone(),
                precision: result.precision.clone(),
            },
        }
    }
}
impl ToString for EosAsset {
    fn to_string(&self) -> String {
        let amount = self.amount.clone().unwrap();
        let precision = self.symbol.get_precision();
        let symbol = self.symbol.get_symbol();
        let dec = eosformat::decimal_pad(amount, precision);

        format!("{} {}", dec, symbol)
    }
}
impl Default for EosAsset {
    fn default() -> Self {
        EosAsset {
            amount: Some("0.0001".to_string()),
            symbol: Default::default(),
        }
    }
}

struct ExtemdedEosAsset {
    pub asset: EosAsset,
    pub contract: String,
}
impl From<String> for ExtemdedEosAsset {
    fn from(txt: String) -> Self {
        let result = eosformat::parse_asset(txt);
        ExtemdedEosAsset {
            asset: EosAsset {
                amount: result.amount.clone(),
                symbol: Symbol {
                    symbol_code: result.symbol.clone(),
                    precision: result.precision.clone(),
                },
            },
            contract: result.contract.clone().unwrap(),
        }
    }
}
impl Default for ExtemdedEosAsset {
    fn default() -> Self {
        ExtemdedEosAsset {
            contract: "eosio.token".to_string(),
            asset: EosAsset {
                amount: Some("1.000".to_string()),
                symbol: Symbol {
                    symbol_code: "SYS".to_string(),
                    precision: Some(4),
                },
            },
        }
    }
}
