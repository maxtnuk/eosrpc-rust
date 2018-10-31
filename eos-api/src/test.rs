#[test]
fn params_test(){
    use ParamSt;
    use create_transaction;
    
    let test=ParamSt::new(None);
    create_transaction(&test,Some(60),|x|{
        println!("{:?}",x);
    })
}