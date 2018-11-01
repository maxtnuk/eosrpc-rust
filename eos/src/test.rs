#[test]
fn format(){
    use format::*;
    
    let isname=vec!["isname111111", "a", "1", "5", "sam5", "sam", "adam.applejj"];
    let noname=vec!["isname111111j", "6", "a6", " "];
    
    for i in isname.iter(){
        assert!(is_name(i.to_string()),"error")
    }
    for i in noname.iter(){
        assert!(!is_name(i.to_string()),"error")
    }
    let eos_encode=encode_name("eos".to_string(),true).unwrap();
    //println!("{}",eos_encode);
    assert!("12373"==eos_encode.as_str(),"encode");
    assert!("3055"==encode_name_hex("eos".to_string()), "encode hex");
    assert!(decode_name(eos_encode,true)=="eos", "decode");

    assert!("572d3ccdcd".to_string()==encode_name_hex("transfer".to_string()), "encode");
    assert!(decode_name_hex("572d3ccdcd".to_string(),true)=="transfer".to_string(), "decode");
    struct TestSet<'a>{
        value: &'a str,
        precision: Option<usize>,
        answer: &'a str
    }
    let decFixtures = vec![
      TestSet{value: "-1", precision: None, answer: "-1"},
      TestSet{value: "1", precision: None, answer: "1"},

      TestSet{value: "1", precision: Some(0), answer: "1"},
      TestSet{value: "1", precision: Some(0), answer: "1"},
      TestSet{value: "1.", precision: Some(0), answer: "1"},
      TestSet{value: "1.0", precision: Some(0), answer: "1"},
      TestSet{value: "1456.0", precision: Some(0), answer: "1456"},
      TestSet{value: "1,456.0", precision: Some(0), answer: "1,456"},

      // does not validate commas
      TestSet{value: "1,4,5,6", precision: Some(0), answer: "1,4,5,6"},
      TestSet{value: "1,4,5,6.0", precision: Some(0), answer: "1,4,5,6"},

      TestSet{value: "1", precision: Some(1), answer: "1.0"},
      TestSet{value: "1", precision: Some(1), answer: "1.0"},
      TestSet{value: "1.", precision: Some(1), answer: "1.0"},
      TestSet{value: "1.0", precision: Some(1), answer: "1.0"},
      TestSet{value: "1.10", precision: Some(1), answer: "1.1"},

      TestSet{value: "1.1", precision: Some(2), answer: "1.10"},
      TestSet{value: "1.10", precision: Some(2), answer: "1.10"},
      TestSet{value: "1.01", precision: Some(2), answer: "1.01"},

      TestSet{value: "1", precision: Some(3), answer: "1.000"}
    ];
    
    for test in decFixtures{
        let r_answer=decimal_pad(test.value.to_string(),test.precision);
        //println!("{}=>{} : {}",test.value,r_answer,test.answer);
        assert!(r_answer==test.answer,"erros")
    }
    println!("next");
    let unimpltest = vec![
      TestSet{value: "-1", precision: Some(0), answer: "-1"},
      TestSet{value: "1", precision: Some(0), answer: "1"},
      TestSet{value: "1", precision: Some(0), answer: "1"},
      TestSet{value: "10", precision: Some(0), answer: "10"},

      TestSet{value: "1", precision: Some(1), answer: "0.1"},
      TestSet{value: "10", precision: Some(1), answer: "1.0"},

      TestSet{value: "11", precision: Some(2), answer: "0.11"},
      TestSet{value: "110", precision: Some(2), answer: "1.10"},
      TestSet{value: "101", precision: Some(2), answer: "1.01"},
      TestSet{value: "0101", precision: Some(2), answer: "1.01"},
      TestSet{value: "1", precision: Some(5), answer: "0.00001"},
    ];
    
    for test in unimpltest{
        let r_answer=decimal_unimply(test.value.to_string(),test.precision);
        //println!("{}=>{} : {}",test.value,r_answer,test.answer);
        assert!(r_answer==test.answer,"erros")
    }
    struct ParseSet<'a>{
        text: &'a str,
        amount: Option<&'a str>,
        precision: Option<usize>,
        symbol: &'a str,
        contract: Option<&'a str>
    }
    let parsetest=vec![
          ParseSet{text:"SYM",
                    amount: None,precision: None,symbol: "SYM",contract: None},
          ParseSet{text:"SYM@contract",
                    amount: None,precision: None,symbol: "SYM",contract: Some("contract")},
          ParseSet{text:"4,SYM",
                    amount: None,precision: Some(4),symbol: "SYM",contract: None},
          ParseSet{text:"4,SYM@contract",
                    amount: None,precision: Some(4),symbol: "SYM",contract: Some("contract")},
          ParseSet{text:"1 SYM",
                    amount: Some("1"),precision: Some(0),symbol: "SYM",contract: None},
          ParseSet{text:"-1 SYM",
                    amount: Some("-1"),precision: Some(0),symbol: "SYM",contract: None},
          ParseSet{text:"1.0 SYM",
                    amount: Some("1.0"),precision: Some(1),symbol: "SYM",contract: None},
          ParseSet{text:"1.0000 SYM@contract",
                    amount: Some("1.0000"),precision: Some(4),symbol: "SYM",contract: Some("contract")},
          ParseSet{text:"1.0000 SYM@tract.token",
                    amount: Some("1.0000"),precision: Some(4),symbol: "SYM",contract: Some("tract.token")},
          ParseSet{text:"1.0000 SYM@tr.act.token",
                    amount: Some("1.0000"),precision: Some(4),symbol: "SYM",contract: Some("tr.act.token")},
          ParseSet{text:"1.0000 SYM",
                    amount: Some("1.0000"),precision: Some(4),symbol: "SYM",contract: None}
    ];
    
    for test in parsetest{
        let r=parse_asset(test.text.to_string());
        println!("test: {}\n=>{:?}",test.text,r);
        let flag=(r.amount.as_ref().map(String::as_str)==test.amount) && (r.precision==test.precision) &&
                 (r.symbol==test.symbol) && (r.contract.as_ref().map(String::as_str)==test.contract);
        assert!(flag,"erros");
    }
}