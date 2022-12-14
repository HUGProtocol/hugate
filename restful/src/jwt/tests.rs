#[test]
fn test_recover() {
    let account = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8".to_string();
    let message = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o".to_string();
    let message = eth_message(message);
    let signature = hex::decode("382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b").unwrap();
    println!("{} {:?} {:?}", account, message, signature);
    let pubkey = recover(&message, &signature[..64], 0);
    assert!(pubkey.is_ok());
    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    assert_eq!(account, pubkey)
}
