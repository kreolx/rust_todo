use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use chrono::serde::ts_seconds;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub user_id: i32,
    #[serde(with = "ts_seconds")]
    pub minted: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

impl JwtToken {
    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY")
            .unwrap().as_str().unwrap();
        return key_str.to_owned();
    }
    pub fn encode(&self) -> String {
        let key = jsonwebtoken::EncodingKey::from_secret(JwtToken::get_key().as_ref());
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &self, &key).unwrap();
        return token;
    }
    pub fn new(user_id: i32) -> Self {
        let timestamp = Utc::now();
        let exp = timestamp + chrono::Duration::minutes(10);
        println!("{:#?} exp result is", exp);
        return JwtToken {
            user_id,
            minted: timestamp,
            exp: timestamp,
        };
    }
    pub fn from_token(token: String) -> Option<Self> {
        let key = jsonwebtoken::DecodingKey::from_secret(JwtToken::get_key().as_ref());

        let token_result = jsonwebtoken::decode::<JwtToken>(&token, &key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256));
        println!("{:#?} token result is", token_result);
        match token_result {
            Ok(data) => {
                Some(data.claims)
            },
            Err(_) => {
                return None;
            }
        }
    }
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = futures::future::Ready<Result<JwtToken, Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("bearer") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                println!("{:?} key is", raw_token);
                let token_result = JwtToken::from_token(raw_token);
                match token_result {
                    Some(token) => {
                        futures::future::ok(token)
                    },
                    None => {
                        let error = ErrorUnauthorized("token can't be decoded");
                        return futures::future::err(error);
                    }
                }

            },
            None => {
                let error = ErrorUnauthorized("token can't be decoded");
                return futures::future::err(error);
            }
        }
    }
}