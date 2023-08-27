use serde_json::{json, Value};
use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::Error;
use std::str::FromStr;

pub struct Mattermost{
    base_uri: String,
    token: String,
}

impl Mattermost{
    pub fn new(base_uri: &str, token: &str) -> Mattermost{
        Self {
            base_uri: base_uri.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn post_message(&self, channel_id: &str, message: &str, root_id: Option<&str>) -> Result<Response, Error>{
        let url = format!("{}/api/v4/posts", self.base_uri);
        let body = if let Some(value) = root_id{
            json!({
                "channel_id": channel_id,
                "message": message,
                "root_id": value
            })
        }else{
            json!({
                "channel_id": channel_id,
                "message": message,
            })
        };
        self.post(&url, Some(body)).await
    }

    pub async fn list_channels(&self) ->Result<Vec<Value>, Error>{
        let url = format!("{}/api/v4/channels", self.base_uri);
        let content = self.get(&url).await?;
        let json: Vec<Value> = serde_json::from_str(&content).unwrap();
        Ok(json)
    }

    pub async fn get_channel_by_name(&self, name: &str)-> Option<String>{
        let channels = self.list_channels().await.unwrap();
        for channel in channels{
            let channel_name = channel.get("name").unwrap().as_str().unwrap();
            let channel_id = channel.get("id").unwrap().as_str().unwrap();
            if channel_name == name{
                return Some(channel_id.to_string());
            }
        }
        None
    }

    async fn get(&self, url: &str)->Result<String, Error>{
        println!("URL: {}", url);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let res = client.get(url).send().await?.text().await?;
        Ok(res)
    }

    async fn post(&self, url: &str, body: Option<Value>)->Result<Response, Error>{
        println!("URL: {}", url);
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
                let res = client.post(url).body(content).send().await?;
                Ok(res)
            },
            None => {
                let res = client.post(url).send().await?;
                Ok(res)
            },
        }
    }
}
#[cfg(test)]
mod tests{
    use crate::mattermost::Mattermost;
    use dotenv::dotenv;

    #[tokio::test]
    async fn list_channels() {
        dotenv().ok();
        let base_uri = std::env::var("MATTERMOST_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MATTERMOST_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", &token);
        let mattermost = Mattermost::new(&base_uri, &token);
        let res = mattermost.list_channels().await.unwrap();
        for channel in &res{
            println!("{:?}", channel);
        }
        println!("{:?}", res);
    }
    #[tokio::test]
    async fn find_channel() {
        dotenv().ok();
        let base_uri = std::env::var("MATTERMOST_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MATTERMOST_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", &token);
        let mattermost = Mattermost::new(&base_uri, &token);
        let res = mattermost.get_channel_by_name("correo").await;
        println!("{:?}", res);
    }
    #[tokio::test]
    async fn post_message() {
        dotenv().ok();
        let base_uri = std::env::var("MATTERMOST_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MATTERMOST_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", &token);
        let mattermost = Mattermost::new(&base_uri, &token);
        let res = mattermost.get_channel_by_name("atareao_correo").await;
        let res = mattermost.post_message(&res.unwrap(), "Esto es una prueba", None).await;
        println!("{:?}", res);
        for name in ["atareao_idea", "atareao_pregunta", "atareao_comentario"]{
            let id = mattermost.get_channel_by_name(name).await.unwrap();
            println!("{} => {}", name, id);
        }
    }
}
