#[test]
fn params_test(){
    use EosApi;
    use create_transaction;
    
    let test=EosApi::new(None);
    create_transaction(&test,Some(60),|x|{
        println!("{:?}",x);
    })
}