mod mastodon;
mod feedback;
mod config;
mod message;
mod mattermost;

use dotenv::dotenv;
use std::{thread, time, env};
use tokio;
use crate::{mastodon::Mastodon, config::Config, feedback::Feedback};
use serde_json::{Map, Value};
use crate::message::{check_key, check_comment};

const FILENAME: &str = "lastid.toml";


#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = env::var("URL")
        .expect("Not found URL");
    let token = env::var("TOKEN")
        .expect("Not found TOKEN");
    let sleep_time_in_seconds = env::var("SLEEP_TIME")
        .expect("Not found SLEEP_TIME")
        .parse::<u64>()
        .unwrap();
    let mastodon_base_uri = env::var("MASTODON_BASE_URI").expect("Not found Mastodon Base Uri");
    let mastodon_token = env::var("MASTODON_ACCESS_TOKEN").expect("Not found Mastodon token");
    let sleep_time = time::Duration::from_secs(sleep_time_in_seconds);
    let mastodon = Mastodon::new(&mastodon_base_uri, &mastodon_token);
    loop {
        thread::sleep(sleep_time);
        search(&url, &token, &mastodon).await;
    }
}
async fn search(url: &str, token: &str, mastodon: &Mastodon){
    let res = mastodon.notifications().await;
    if res.is_ok(){
        let notifications: Vec<Value> = serde_json::from_str(&res.unwrap()).unwrap();
        for notification in notifications {
            //println!("{}", status);
            let status = notification.get("status").unwrap();
            let content = status.get("content").unwrap().as_str().unwrap();
            let created_at = status.get("created_at").unwrap().as_str().unwrap();
            let id = status.get("id").unwrap().as_str().unwrap();
            let account = notification.get("account").unwrap();
            let name = account.get("username").unwrap().as_str().unwrap();
            let nickname = account.get("acct").unwrap().as_str().unwrap();
            println!("==========");
            println!("Text: {}", content);
            println!("Id: {}", id);
            println!("created_at: {}", created_at);
            println!("Name: {}", name);
            println!("Screen Name: {}", nickname);
            if let Some(message) = check_key("idea", content){
                let feedback = Feedback::new("idea", &id, &message, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
                let message = format!("Gracias por tu idea @{}", nickname);
                mastodon.post(&message, Some(id.to_string())).await;
            }else if let Some(message) = check_key("pregunta", content){
                let feedback = Feedback::new("pregunta", &id, &message, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
                let message = format!("Gracias por tu pregunta @{}", nickname);
                mastodon.post(&message, Some(id.to_string())).await;
            }else if let Some(option) = check_comment("comentario", content){
                let (commentario, reference) = option;
                if let Some(message) = commentario{
                    let feedback = Feedback::new("comentario", &id, &message, name, nickname, 0, "Mastodon");
                    feedback.post(url, token).await;
                    let message = format!("Gracias por tu comentario @{}", nickname);
                    mastodon.post(&message, Some(id.to_string())).await;
                }
            }else{
                let feedback = Feedback::new("mencion", &id, content, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
            }
        }
    }
    mastodon.clear_notifications().await;
}
