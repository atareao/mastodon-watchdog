mod mastodon;
mod feedback;
mod config;
mod message;
mod mattermost;
mod zinc;
use html2md::parse_html;

use dotenv::dotenv;
use std::{thread, time, env};
use tokio;
use crate::{mastodon::Mastodon, mattermost::Mattermost, config::Config,
            feedback::Feedback, zinc::Zinc};
use serde_json::{Value, json};
use crate::message::{check_key, check_comment};

const FILENAME: &str = "lastid.toml";


#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut config = Config::read("lastid.toml").expect("Can not read last id");
    let mut last_id = config.get_last_id().to_string();
    let url = env::var("URL")
        .expect("Not found URL");
    let token = env::var("TOKEN")
        .expect("Not found TOKEN");
    let sleep_time_in_seconds = env::var("SLEEP_TIME")
        .expect("Not found SLEEP_TIME")
        .parse::<u64>()
        .unwrap();
    let sleep_time = time::Duration::from_secs(sleep_time_in_seconds);
    let mastodon_base_uri = env::var("MASTODON_BASE_URI").expect("Not found Mastodon Base Uri");
    let mastodon_token = env::var("MASTODON_ACCESS_TOKEN").expect("Not found Mastodon token");
    let mastodon = Mastodon::new(&mastodon_base_uri, &mastodon_token);
    let mattermost_base_uri = env::var("MATTERMOST_BASE_URI").expect("Not found Mattermost Base Uri");
    let mattermost_token = env::var("MATTERMOST_ACCESS_TOKEN").expect("Not found Mattermost token");
    let mattermost = Mattermost::new(&mattermost_base_uri, &mattermost_token);
    let idea_channel = mattermost.get_channel_by_name("atareao_idea").await.unwrap();
    let pregunta_channel = mattermost.get_channel_by_name("atareao_pregunta").await.unwrap();
    let comentario_channel = mattermost.get_channel_by_name("atareao_comentario").await.unwrap();
    let mencion_channel = mattermost.get_channel_by_name("atareao_mencion").await.unwrap();
    let zinc_base_url = env::var("ZINC_BASE_URL").expect("Not found zinc base url");
    let zinc_indice = env::var("ZINC_INDICE").expect("Not found zinc indice");
    let zinc_token = env::var("ZINC_TOKEN").expect("Not found token");
    let zinc = Zinc::new(&zinc_base_url, &zinc_indice, &zinc_token);
    loop {
        match search(&url, &token, &mastodon, &last_id, &mattermost,
                     &idea_channel, &pregunta_channel, &comentario_channel,
                     &mencion_channel, &zinc).await{
                Some(new_last_id) => {
                    config.last_id = new_last_id.to_string();
                    config.save(&FILENAME);
                    last_id = new_last_id.to_string();
                },
                _ => {},
            }
        thread::sleep(sleep_time);
    }
}
async fn search(url: &str, token: &str, mastodon: &Mastodon, last_id: &str,
        mattermost: &Mattermost, idea_channel: &str, pregunta_channel: &str,
        comentario_channel: &str, mencion_channel: &str, zinc: &Zinc) -> Option<String>{
    let mut new_last_id: String = "".to_string();
    let res = mastodon.search(last_id).await;
    if res.is_ok(){
        zinc.publish(&json!([{
            "src": "Mastodon",
            "type": "search",
            "message": res.as_ref().unwrap(),
        }])).await.unwrap();
        let data: Value =  match serde_json::from_str(&res.unwrap()){
            Ok(value) => value,
            Err(_) => json!({"statuses": []}),
        };
        let statuses: Vec<Value> = data.get("statuses").unwrap().as_array().unwrap().to_vec();
        for status in statuses {
            //println!("{}", status);
            let content = status.get("content").unwrap().as_str().unwrap();
            let created_at = status.get("created_at").unwrap().as_str().unwrap();
            new_last_id = status.get("id").unwrap().as_str().unwrap().to_string();
            let account = status.get("account").unwrap();
            let name = account.get("username").unwrap().as_str().unwrap();
            let nickname = account.get("acct").unwrap().as_str().unwrap();
            println!("==========");
            println!("Text: {}", parse_html(content));
            println!("Id: {}", &new_last_id);
            println!("created_at: {}", created_at);
            println!("Name: {}", name);
            println!("Screen Name: {}", nickname);
            if let Some(message) = check_key("idea", content){
                let feedback = Feedback::new("idea", &new_last_id, &message, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
                let thanks_message = format!("Gracias por tu idea @{}", nickname);
                mastodon.post(&thanks_message, Some(new_last_id.to_string())).await;
                let mm_message = format!("Src: Mastodon. From: @{}. Content: {}", &nickname, &parse_html(&content));
                mattermost.post_message(idea_channel, &mm_message, None).await;
                zinc.publish(&json!([{
                    "src": "Mastodon",
                    "type": "idea",
                    "from": format!("@{}", &nickname),
                    "message": &parse_html(&content),
                }])).await.unwrap();
            }else if let Some(message) = check_key("pregunta", content){
                let feedback = Feedback::new("pregunta", &new_last_id, &message, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
                let thanks_message = format!("Gracias por tu pregunta @{}", nickname);
                mastodon.post(&thanks_message, Some(new_last_id.to_string())).await;
                let mm_message = format!("Src: Mastodon. From: @{}. Content: {}", &nickname, &parse_html(&content));
                mattermost.post_message(pregunta_channel, &mm_message, None).await;
                zinc.publish(&json!([{
                    "src": "Mastodon",
                    "type": "pregunta",
                    "from": format!("@{}", &nickname),
                    "message": &parse_html(&content),
                }])).await.unwrap();
            }else if let Some(option) = check_comment("comentario", content){
                let (commentario, reference) = option;
                if let Some(message) = commentario{
                    let feedback = Feedback::new("comentario", &new_last_id, &message, name, nickname, 0, "Mastodon");
                    feedback.post(url, token).await;
                    let thanks_message = format!("Gracias por tu comentario @{}", nickname);
                    mastodon.post(&thanks_message, Some(new_last_id.to_string())).await;
                let mm_message = format!("Src: Mastodon. From: @{}. Content: {}", &nickname, &parse_html(&content));
                mattermost.post_message(comentario_channel, &mm_message, None).await;
                zinc.publish(&json!([{
                    "src": "Mastodon",
                    "type": "comentario",
                    "from": format!("@{}", &nickname),
                    "message": &parse_html(&content),
                }])).await.unwrap();
                }
            }else{
                let feedback = Feedback::new("mencion", &new_last_id, content, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
                let mm_message = format!("Src: Mastodon. From: @{}. Content: {}", &nickname, &parse_html(&content));
                mattermost.post_message(mencion_channel, &mm_message, None).await;
                zinc.publish(&json!([{
                    "src": "Mastodon",
                    "type": "mencion",
                    "from": format!("@{}", &nickname),
                    "message": &parse_html(&content),
                }])).await.unwrap();
            }
        }
    }else{
        zinc.publish(&json!([{
            "src": "Mastodon",
            "type": "search",
            "message": "Something goes wrong!!",
        }])).await.unwrap();
    }
    if new_last_id != "" && new_last_id != last_id{
        return Some(new_last_id);
    }
    None
}
