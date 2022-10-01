use reqwest::Client;
use std::format;
use serde::{Serialize, Deserialize};

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
        println!("{}", &url);
        let client = Client::new();
        let body = Message{status: message.to_string(), in_reply_to_id};
        let response = client
            .post(&url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await;
        println!("{:?}", response);
    }

    pub async fn search(&self, min_id: &str) -> Result<String, reqwest::Error>{
        let url = format!("{}/api/v2/search/", self.base_uri);
        let params = [
            ("min_id", min_id),
            ("q", "atareao"),
            ("type", "statuses")
        ];
        println!("========");
        println!("{}", &url);
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

    pub async fn notifications(&self) -> Result<String, reqwest::Error>{
        let url = format!("{}/api/v1/notifications",
            self.base_uri,
        );
        println!("{}", &url);
        let params = [
            ("exclude_types[]", "follow"),
            ("exclude_types[]", "favourite"),
            ("exclude_types[]", "reblog"),
            ("exclude_types[]", "poll"),
            ("exclude_types[]", "follow_request"),
        ];
        let client = Client::new();
        let res = client
            .get(&url)
            .query(&params)
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

    #[actix_rt::test]
    async fn name() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", &token);
        let mastodon = Mastodon::new(&base_uri, &token);
        mastodon.post("muchas gracias por tu idea @atareao", None).await;
    }

    #[actix_rt::test]
    async fn search() {
        dotenv().ok();
        let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
        let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
        println!("{}", base_uri);
        println!("{}", token);
        let mastodon = Mastodon::new(&base_uri, &token);
        let res = mastodon.search("0").await.unwrap();
        println!("{}", res);
    }

    #[actix_rt::test]
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

    #[actix_rt::test]
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
}
