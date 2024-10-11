use reqwest::StatusCode;
use argon2::{self, Config};
use chrono::Utc;
use rand::{random};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::store::Store;
use crate::types::account::{Account, AccountId};

pub async fn register(
    store: Store,
    account: Account
) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password
    };

    match store.add_account(account).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Account added", StatusCode::OK))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

pub async fn login(
    store: Store,
    login: Account
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(account.id.expect("id not found"))))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(handle_errors::Error::ArgonLibraryError(e)))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

fn hash_password(
    password: &[u8]
) -> String {
    let salt = random::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(
    hash: &str,
    password: &[u8]
) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    account_id: String,
    exp: usize,
    nbf: usize,
}

fn issue_token(
    account_id: AccountId
) -> String {
    let expiration = Utc::now() + chrono::Duration::hours(1);
    let not_before = Utc::now();

    let claims = Claims {
        account_id: account_id.0.to_string(),
        exp: expiration.timestamp() as usize,
        nbf: not_before.timestamp() as usize,
    };

    let secret = "NESTO NASUMICNO RECENO JER ETO";

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).expect("Failed to create JWT");

    token
}
