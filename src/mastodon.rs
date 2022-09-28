use dotenv::dotenv;
use urlencoding::encode;
use reqwest::Client;
use std::format;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

pub struct Mastodon{
    base_uri: String,
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct Message{
    status: String,
}

impl Mastodon{
    pub fn new(base_uri: &str, access_token: &str) -> Self{
        Mastodon {
            base_uri: base_uri.to_string(),
            access_token: access_token.to_string(),
        }
    }

    pub async fn post(&self, message: &str){
        let url = format!("{}/api/v1/statuses", self.base_uri);
        println!("{}", &url);
        let client = Client::new();
        let body = Message{status: message.to_string()};
        let response = client
            .post(&url)
            .json(&body)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await;
        println!("{:?}", response);
    }

    pub async fn search(&self, min_id: &str){
        let query = "atareao";
        let url = format!("{}/api/v2/search?min_id={}&q={}&type=statuses",
            self.base_uri,
            min_id,
            query
        );
        println!("{}", &url);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();
        let mut response: Map<String, Value> = serde_json::from_str(res.as_str()).unwrap();
        println!("{:?}", response);
        let statuses = response.get_mut("statuses").unwrap().as_array().unwrap().to_owned();
        for status in statuses {
            println!("==================");
            println!("{:?}", status);
            let content = status.get("content").unwrap().as_str().unwrap();
            let created_at = status.get("created_at").unwrap().as_str().unwrap();
            let id = status.get("id").unwrap().as_str().unwrap();
            let account = status.get("account").unwrap();
            let name = account.get("display_name").unwrap().as_str().unwrap();
            let nickname = account.get("acct").unwrap().as_str().unwrap();
            println!("Id: {}", id);
            println!("created_at: {}", created_at);
            println!("content: {}", content);
            println!("name: {}", name);
            println!("nickname: {}", nickname);
        }
    }

    pub async fn notifications(&self, min_id: &str){
        let query = "atareao";
        //let url = format!("{}/api/v1/notifications?min_id={}&exclude_type=follow,favourite,reblog,poll,follow_request",
        let url = format!("{}/api/v1/notifications?exclude_types=follow,favourite,reblog,poll,follow_request",
            self.base_uri,
        );
        println!("{}", &url);
        let client = Client::new();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();
        println!("{:?}", res);
        let mut response: Map<String, Value> = serde_json::from_str(res.as_str()).unwrap();
        println!("{:?}", response);
        let statuses = response.get_mut("statuses").unwrap().as_array().unwrap().to_owned();
        for status in statuses {
            println!("==================");
            println!("{:?}", status);
            let content = status.get("content").unwrap().as_str().unwrap();
            let created_at = status.get("created_at").unwrap().as_str().unwrap();
            let id = status.get("id").unwrap().as_str().unwrap();
            let account = status.get("account").unwrap();
            let name = account.get("display_name").unwrap().as_str().unwrap();
            let nickname = account.get("acct").unwrap().as_str().unwrap();
            println!("Id: {}", id);
            println!("created_at: {}", created_at);
            println!("content: {}", content);
            println!("name: {}", name);
            println!("nickname: {}", nickname);
        }
    }
}

#[actix_rt::test]
async fn name() {
    dotenv();
    let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
    let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
    println!("{}", &token);
    let mastodon = Mastodon::new(&base_uri, &token);
    //mastodon.post("Sample").await;
}

#[actix_rt::test]
async fn search() {
    dotenv();
    let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
    let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
    let mastodon = Mastodon::new(&base_uri, &token);
    mastodon.search("109035315460271106").await;
}

#[actix_rt::test]
async fn notifications() {
    dotenv();
    let base_uri = std::env::var("MASTODON_BASE_URI").expect("BASE_URI not set");
    let token = std::env::var("MASTODON_ACCESS_TOKEN").expect("TOKEN not set");
    let mastodon = Mastodon::new(&base_uri, &token);
    mastodon.notifications("109035315460271106").await;
}
