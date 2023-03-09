use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub user_id: i32,
    pub exp: usize,
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
        let config = Config::new();
        let expired_time = config.map.get("EXPIRE_MINUTES")
            .unwrap().as_i64().unwrap();
        let exp = Utc::now().checked_add_signed(chrono::Duration::minutes(expired_time))
            .expect("valid timestamp")
            .timestamp();
        return JwtToken {
            user_id,
            exp: exp as usize,
        };
    }
    pub fn from_token(token: String) -> Result<Self, &'static str> {
        let key = jsonwebtoken::DecodingKey::from_secret(JwtToken::get_key().as_ref());

        let token_result = jsonwebtoken::decode::<JwtToken>(&token, &key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256));
        match token_result {
            Ok(data) => {
                Ok(data.claims)
            },
            Err(_) => {
                return Err("InvalidToken");
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
                    Ok(token) => {
                        futures::future::ok(token)
                    },
                    Err(_) => {
                        let error = ErrorUnauthorized("InvalidToken");
                        return futures::future::err(error);
                    }
                }

            },
            None => {
                let error = ErrorUnauthorized("InvalidToken");
                return futures::future::err(error);
            }
        }
    }
}

#[cfg(test)]
mod jwt_tests {
    use actix_web::{App, HttpRequest, HttpResponse, web};
    use actix_web::http::header::{HeaderValue, HeaderName, ContentType};
    use actix_web::test::{call_service, init_service, TestRequest};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use crate::config::Config;
    use crate::jwt::JwtToken;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ResponseFromTest {
        pub user_id: i32,
        pub exp_minutes: i32,
    }
    #[test]
    fn get_key() {
        assert_eq!(String::from("secret"), JwtToken::get_key());
    }

    #[test]
    fn get_exp() {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES")
            .unwrap().as_i64().unwrap();
        assert_eq!(120, minutes);
    }

    #[test]
    fn decode_incorrect_token() {
        let encoded_token: String = String::from("invalid_token");
        match JwtToken::from_token(encoded_token) {
            Ok(_) => panic!("Incorrect token should not be able to be encoded"),
            Err(msg) => assert_eq!("InvalidToken", msg),
        }
    }

    #[test]
    fn encode_decode() {
        let test_token = JwtToken::new(5);
        let encoded_token = test_token.encode();
        let new_token = JwtToken::from_token(encoded_token).unwrap();
        assert_eq!(5, new_token.user_id);
    }

    async fn test_handler(token: JwtToken, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id, "exp_minutes": 60}));
    }

    #[actix_web::test]
    async fn test_no_token_request() {
        let app = init_service(App::new().route("/", web::get()
            .to(test_handler))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = call_service(&app, req).await;
        assert_eq!("401", resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_passing_token_request() {
        let test_token = JwtToken::new(5);
        let encoded_token = test_token.encode();
        let app = init_service(App::new().route("/", web::get()
            .to(test_handler))).await;
        let mut req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let header_name = HeaderName::from_lowercase(b"bearer").unwrap();
        let header_value = HeaderValue::from_str(encoded_token.as_str()).unwrap();
        req.headers_mut().insert(header_name, header_value);
        let resp: ResponseFromTest = actix_web::test::call_and_read_body_json(&app, req).await;
        assert_eq!(5, resp.user_id);
    }

    #[actix_web::test]
    async fn test_false_token_request() {
        let app = init_service(App::new().route("/", web::get()
            .to(test_handler))).await;
        let mut req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let header_name = HeaderName::from_lowercase(b"bearer").unwrap();
        let header_value = HeaderValue::from_str("test").unwrap();
        req.headers_mut().insert(header_name, header_value);
        let resp = call_service(&app, req).await;
        assert_eq!("401", resp.status().as_str());
    }
}