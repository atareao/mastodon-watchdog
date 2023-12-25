use serde_json::{json, Value};
use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use std::str::FromStr;
use urlencoding::encode;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use super::Error;

pub struct Matrix{
    base_url: String,
    token: String,
}

impl Matrix{
    pub fn new(base_url: String, token: String) -> Self{
        Self {
            base_url,
            token,
        }
    }

    pub async fn post_message(&self, room_id: &str, message: &str, html: &str) -> Result<String, Error>{
        let room = encode(room_id);
        let now = SystemTime::now();
        let ts = now.duration_since(UNIX_EPOCH).expect("Time went backwrds").as_secs();
        let url = format!(
            "https://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}",
            self.base_url,
            room,
            self.base_url,
            ts
        );
        let body = json!({
            "msgtype": "m.text",
            "format": "org.matrix.custom.html",
            "body": message,
            "formatted_body": html
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        Self::put(&url, header_map, &body).await
    }

    #[allow(unused)]
    async fn post(&self, url: &str, body: Option<Value>)->Result<String, Error>{
        debug!("URL: {}", url);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        match body{
            Some(value) => {
                let content = serde_json::to_string(&value).unwrap();
                Ok(client.post(url).body(content).send()
                    .await?
                    .text()
                    .await?)
            },
            None => {
                Ok(client
                    .post(url)
                    .send()
                    .await?
                    .text()
                    .await?)
            },
        }
    }
    async fn put(url: &str, header_map: HeaderMap, body: &Value) -> Result<String, Error>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        Ok(client.put(url)
            .body(content)
            .send()
            .await?
            .text()
            .await?
        )
    }
}
#[cfg(test)]
mod tests{
    use super::Matrix;
    use dotenv::dotenv;

    #[tokio::test]
    async fn post_message() {
        dotenv().ok();
        let base_url = std::env::var("MATRIX_BASE_URL").expect("BASE_URL not set");
        let token = std::env::var("MATRIX_TOKEN").expect("TOKEN not set");
        let room_id = std::env::var("MATRIX_ROOM_ID").expect("ROOM_ID not set");
        println!("{}", &token);
        let matrix_client = Matrix::new(base_url, token);
        let res = matrix_client.post_message(
            &room_id,
            "Esto es una prueba",
            "Esto es una prueba").await;
        println!("{:?}", res);
    }
}


