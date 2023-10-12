use reqwest::Client;
use std::format;
use serde::{Serialize, Deserialize};
use tracing::debug;

pub struct Mastodon{
    base_uri: String,
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct Message{
    status: String,
    in_reply_to_id: Option<String>,
}

impl Mastodon{
    pub fn new(base_uri: &str, access_token: &str) -> Self{
        Mastodon {
            base_uri: base_uri.to_string(),
            access_token: access_token.to_string(),
        }
    }

    pub async fn post(&self, message: &str, in_reply_to_id: Option<String>){
        let url = format!("{}/api/v1/statuses", self.base_uri);
        debug!("{}", &url);
        let client = Client::new();
        let body = Message{status: message.to_string(), in_reply_to_id};
        let response = client
            .post(&url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await;
        debug!("{:?}", response);
    }

    #[allow(unused)]
    pub async fn search(&self, min_id: &str) -> Result<String, reqwest::Error>{
        let url = format!("{}/api/v2/search/", self.base_uri);
        debug!("{}", &url);
        let params = [
            ("min_id", min_id),
            ("q", "atareao"),
            ("type", "statuses")
        ];
        let client = Client::new();
        let res = client
            .get(url)
            .query(&params)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }
    pub async fn notifications(&self, since_id: &str) -> Result<String, reqwest::Error>{
        let url = format!("{}/api/v1/notifications/", self.base_uri);
        debug!("{}", &url);
        let params = [
            ("types[]", "mention"),
            ("since_id", since_id)
        ];
        let client = Client::new();
        let res = client
            .get(url)
            .query(&params)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)

    }

    #[allow(unused)]
    pub async fn clear_notifications(&self) -> Result<String, reqwest::Error>{
        let url = format!("{}/api/v1/notifications/clear",
            self.base_uri,
        );
        let client = Client::new();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)

    }
}

#[cfg(test)]
mod tests{
    use crate::Mastodon;
    use dotenv::dotenv;

    /*
    #[actix_rt::test]
    async fn name() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", &token);
        let mastodon = Mastodon::new(&base_uri, &token);
        mastodon.post("muchas gracias por tu idea @atareao", None).await;
    }
    */

    #[tokio::test]
    async fn search() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", base_uri);
        println!("{}", token);
        let mastodon = Mastodon::new(&base_uri, &token);
        let id = "110758642668166239";
        let res = mastodon.search(id).await.unwrap();
        println!("{}", res);
    }

    #[tokio::test]
    async fn notifications() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", base_uri);
        println!("{}", token);
        let mastodon = Mastodon::new(&base_uri, &token);
        let res = mastodon.notifications("0").await.unwrap();
        println!("{}", res);
    }
    /*

    #[tokio::test]
    async fn notifications() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", base_uri);
        println!("{}", token);
        let mastodon = Mastodon::new(&base_uri, &token);
        let res = mastodon.notifications().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn notifications_clear() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", base_uri);
        println!("{}", token);
        let mastodon = Mastodon::new(&base_uri, &token);
        let res = mastodon.clear_notifications().await.unwrap();
        println!("{:?}", res);
    }
    */
}
