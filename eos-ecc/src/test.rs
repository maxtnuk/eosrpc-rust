use prelude::*;
#[test]
fn key_unit_test() {
    use hash;
    use key::{PrivateKey, PublicKey};

    let (test_str, pub_str) = (
        "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss",
        "EOS859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVM",
    );
    let pvtError = "key comparison test failed on a known private key";
    let pubError = "pubkey string comparison test failed on a known public key";

    let test_data = hash::sha256(Data::from(""));
    let pvt = PrivateKey::from(test_data);
    //println!("{}",pvt.to_string().as_str());

    //let decode_data= test_str.from_base58().unwrap();
    //println!("decoded {:?}",decode_data);
    assert!(pvt.to_string() == test_str, pvtError);
    // assert.equal(pvt.toString(), 'PVT_K1_2jH3nnhxhR3zPUcsKaWWZC9ZmZAnKm3GAnFD1xynGJE1Znuvjd', pvtError)
    //assert!(false,"pause");
    let pubdata = PublicKey::from(&pvt);
    //println!("{}",pubdata.to_string());
    assert!(pubdata.to_string() == pub_str, pubError);

    // assert.equal(pub.toString(), 'PUB_K1_859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2Ht7beeX', pubError)
    // assert.equal(pub.toStringLegacy(), 'EOS859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVM', pubError)
}
#[test]
fn bigint_test() {
    use num_bigint::ToBigInt;
    use curve::curvetool::EcTools;

    let (r, s) = (10.to_bigint().unwrap(), 0xABCD_i32.to_bigint().unwrap());
    let vec_r = EcTools::integer_to_vec(r.clone(), 32);
    let vec_s = EcTools::integer_to_vec(s.clone(), 32);
    println!("{:?}\n{:?}", vec_r, vec_s);

    println!(
        "{}\n{}",
        EcTools::vec_to_integer(Data::new(vec_r)),
        EcTools::vec_to_integer(Data::new(vec_s))
    );
}
#[test]
fn random_key_test() {
    use key::{PrivateKey, PublicKey};

    let seed_err = "randome seed not same";

    let wif = "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss";

    let randomkey = PrivateKey::unsafe_randomkey(None);
    let pub_ran = PublicKey::from(&randomkey);
    println!(
        "\nprivate key: {}\npublic key: {}",
        randomkey.to_string(),
        pub_ran.to_string()
    );
    println!(
        "private valid {}",
        PrivateKey::is_valid(randomkey.to_string())
    );
    println!(
        "public valid {}",
        PublicKey::is_valid(pub_ran.to_string(), None, None)
    );

    let seed_randome = PrivateKey::from_seed("", None);
    assert!(seed_randome.to_string() == wif, seed_err);

    let o_pub = PublicKey::from(&PrivateKey::from(wif));
    let r_pub = "EOS859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVM".to_string();
    assert!(o_pub.to_string() == r_pub, seed_err);

}
#[test]
fn valid_test() {
    use key::{PrivateKey, PublicKey};

    let keys: Vec<(bool, &str, Option<&str>)> = vec![
        (
            true,
            "PUB_K1_859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2Ht7beeX",
            None
        ),
        (
            true,
            "EOS859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVM",
            None
        ),
        (
            false,
            "MMM859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVM",
            None
        ),
        (
            false,
            "EOS859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVm",
            Some("EOS")
        ),
        (
            true,
            "PUB859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVM",
            Some("PUB")
        ),
        (
            false,
            "PUB859gxfnXyUriMgUeThh1fWv3oqcpLFyHa3TfFYC4PK2HqhToVm",
            Some("PUB")
        ),
    ];
    for (b, key, prefix) in keys {
        assert!(
            b == PublicKey::is_valid(key.to_string(), None, prefix),
            key.to_string()
        );
    }

    let pvtkeys = vec![
        (true, "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss"),
        (false, "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjsm"),
    ];

    for (b, key) in pvtkeys {
        assert!(b == PrivateKey::is_valid(key.to_string()), key.to_string());
    }

}
#[test]
fn signature_test() {
    use key::{PrivateKey, PublicKey};
    use hash;
    use signature::Signature;

    let pvt = PrivateKey::from_seed("", None);
    //println!("{}", pvt.to_string());
    let pubkey = PublicKey::from(&pvt);
    //println!("{}", pubkey.to_string());
    let data = "hi";
    let hashed = hash::sha256(Data::from(data));

    let sigs = vec![
        Signature::sign(data, &pvt),
        Signature::signhash(hashed.clone(), &pvt),
    ];

    for sig in sigs {
        let ecsig = sig.get_ecsig();
        //assert!(Signature::verify(&ecsig, data.as_bytes().to_vec(), &pubkey), "verify data");
        //assert!(Signature::verifyhash(&ecsig, hashed.clone(), &pubkey), "verify hash");
        //assert!(pubkey.to_string()==sig.recover(data)., "recover from data");
        //assert!(pubkey.to_string()==sig.recoverHash(hashed), "recover from hash");
    }
}
