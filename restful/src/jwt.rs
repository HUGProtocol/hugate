use chrono::{NaiveDate, NaiveDateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::{Cookie, Cookies};
use serde::{Deserialize, Serialize};
use std::{
    fmt::format,
    io::{Error, ErrorKind},
};
use web3::{
    futures::future::ok,
    signing::{keccak256, recover},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub address: String,
}

pub fn jwt_generate(address: String, timestamp: u64) -> String {
    let dur = 60 * 60 * 24;

    let now = timestamp as i64;
    let payload = UserRolesToken {
        iat: now,
        exp: now + dur,
        address: address,
    };
    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

pub fn check_cookies(cookies: &Cookies) -> Result<UserRolesToken, Error> {
    let token = cookies.get("jwt").map(|cookie| cookie.value());
    if token.is_none() {
        return Result::Err(Error::new(ErrorKind::Other, "no jwt cookie"));
    }

    let token_data = jsonwebtoken::decode::<UserRolesToken>(
        &token.unwrap(),
        &jsonwebtoken::DecodingKey::from_secret("secret".as_ref()),
        &jsonwebtoken::Validation::default(),
    );

    if token_data.is_err() {
        return Result::Err(Error::new(ErrorKind::Other, token_data.err().unwrap()));
    }
    let data = token_data.unwrap();
    return Result::Ok(data.claims);
}

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

pub fn login_message(address: &str, timestamp: u64) -> String {
    format!("login {} {}", address, timestamp)
}

pub fn verify_login_signature(
    address: String,
    timestamp: u64,
    signature: String,
) -> Result<(), std::io::Error> {
    //check time
    let now = Utc::now().timestamp();
    if timestamp as i64 > now {
        return Result::Err(Error::new(
            ErrorKind::Other,
            "timestamp later than current time",
        ));
    }

    //recover
    let msg = login_message(address.as_str(), timestamp);
    let hash = crate::jwt::eth_message(msg);
    let sig = hex::decode(signature);
    if sig.is_err() {
        return Result::Err(Error::new(ErrorKind::Other, sig.err().unwrap().to_string()));
    }
    let sig = sig.ok().unwrap();
    let pubkey = recover(&hash, &sig[..64], 0);
    if pubkey.is_err() {
        return Result::Err(Error::new(
            ErrorKind::Other,
            pubkey.err().unwrap().to_string(),
        ));
    }
    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    //check address
    if address.as_str().eq(pubkey.as_str()) {
        return Result::Err(Error::new(
            ErrorKind::Other,
            format!(
                "login by {}, but signed by {}",
                address.as_str(),
                pubkey.as_str()
            ),
        ));
    }
    Result::Ok(())
}

#[cfg(test)]
mod tests {
    use web3::{contract::tokens::Tokenizable, signing::recover};
    #[test]
    fn test_recover() {
        let account = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8".to_string();
        let message = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o".to_string();
        let message = crate::jwt::eth_message(message);

        let signature = hex::decode("382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b").unwrap();
        println!("{} {:?} {:?}", account, message, signature);
        let pubkey = recover(&message, &signature[..64], 0);
        assert!(pubkey.is_ok());
        let pubkey = pubkey.unwrap();
        let pubkey = format!("{:02X?}", pubkey);
        assert_eq!(account, pubkey)
    }
    #[test]
    fn test_check_pass() {
        let cnt = crate::thoughts::check_pass_balance(
            "0x5e79bBA16F80C5B290F1d6CF661Fbc22F57Bcb95".to_string(),
            8,
        );
        println!("{:?}", cnt);
    }
}
