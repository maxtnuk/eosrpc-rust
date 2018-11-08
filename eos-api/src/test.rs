#[test]
fn params_test(){
    use EosApi;
    use create_transaction;
    
    let test=EosApi::new(None);
    test.create_transaction(Some(60),|x|{
        println!("{:?}",x);
    })
}