mod mastodon;
mod feedback;
mod config;
mod message;
mod matrix;
mod zinc;
use html2md::parse_html;

use dotenv::dotenv;
use std::{thread, time, env, str::FromStr};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use tokio;
use crate::{mastodon::Mastodon, config::Config, feedback::Feedback, zinc::Zinc,
    matrix::Matrix};
use serde_json::{Value, json};
use crate::message::{check_key, check_comment};
use tracing::{debug, error};

const FILENAME: &str = "lastid.toml";


#[tokio::main]
async fn main() {
    dotenv().ok();
    let log_level = env::var("LOG_LEVEL").unwrap_or("DEBUG".to_string());
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

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
    let matrix_base_url = env::var("MATRIX_BASE_URL").expect("Not found Matrix base url");
    let matrix_token = env::var("MATRIX_TOKEN").expect("Not found Matrix token");
    let matrix_room_id = env::var("MATRIX_ROOM_ID").expect("Not found Matrix room_id");
    let matrix = Matrix::new(matrix_base_url, matrix_token);

    let zinc_base_url = env::var("ZINC_BASE_URL").expect("Not found zinc base url");
    let zinc_indice = env::var("ZINC_INDICE").expect("Not found zinc indice");
    let zinc_token = env::var("ZINC_TOKEN").expect("Not found token");
    let zinc = Zinc::new(&zinc_base_url, &zinc_indice, &zinc_token);
    loop {
        match search(&url, &token, &mastodon, &matrix, &matrix_room_id,
                     &last_id, &zinc).await{
                Some(new_last_id) => {
                    config.last_id = new_last_id.to_string();
                    debug!("Save: {:?}", config.save(&FILENAME));
                    last_id = new_last_id.to_string();
                },
                _ => {},
            }
        thread::sleep(sleep_time);
    }
}
async fn search(url: &str, token: &str, mastodon: &Mastodon, matrix: &Matrix, 
        room_id: &str, last_id: &str, zinc: &Zinc) -> Option<String>{
    let mut new_last_id: String = "".to_string();
    let res = mastodon.notifications(last_id).await;
    //let res = mastodon.search(last_id).await;
    if res.is_ok(){
        let message = match res {
            Ok(value) => value,
            Err(e) => format!("Error: {}", e.to_string()),
        };
        match zinc.publish(&json!([{
            "src": "Mastodon",
            "type": "search",
            "message": &message,
        }])).await{
                Ok(response) => debug!("Response: {:?}", response),
                Err(e) => {
                    error!("Message: {}", message);
                    error!("Error: {:?}", e)
                },
            };
        let data: Value =  match serde_json::from_str(&message){
            Ok(value) => value,
            Err(_) => json!([]),
        };
        //let statuses: Vec<Value> = data.get("statuses").unwrap().as_array().unwrap().to_vec();
        let mentions: Vec<Value> = data.as_array().unwrap().to_vec();
        let mentions_reversed: Vec<Value> = mentions.into_iter().rev().collect();
        //mentions.sort_by(|m1, m2| m1.get("id").unwrap().as_str().unwrap().cmp(m2.get("id").unwrap().as_str().unwrap()));
        //for status in statuses {
        for mention in mentions_reversed {
            new_last_id = mention.get("id").unwrap().as_str().unwrap().to_string();
            let status = mention.get("status").unwrap();
            let content = status.get("content").unwrap().as_str().unwrap();
            let created_at = status.get("created_at").unwrap().as_str().unwrap();
            let account = mention.get("account").unwrap();
            let name = account.get("username").unwrap().as_str().unwrap();
            let nickname = account.get("acct").unwrap().as_str().unwrap();
            debug!("==========");
            debug!("Text: {}", parse_html(content));
            debug!("Id: {}", &new_last_id);
            debug!("created_at: {}", created_at);
            debug!("Name: {}", name);
            debug!("Screen Name: {}", nickname);
            if let Some(message) = check_key("idea", content){
                let feedback = Feedback::new("idea", &new_last_id, &message, name, nickname, 0, "Mastodon");
                feedback.post(url, token).await;
                let thanks_message = format!("Gracias por tu idea @{}", nickname);
                mastodon.post(&thanks_message, Some(new_last_id.to_string())).await;
                let mm_message = format!("Src: Mastodon. From: @{}. Content: {}", &nickname, &parse_html(&content));
                let html_message = format!(
                    "<h6>Src: Mastodon</h6><ul><li>Id: {}</li><li>From: @{}</li><li>Content:</li></ul>{}",
                    &new_last_id,
                    &nickname,
                    &content
                );
                debug!("Response: {:?}", matrix.post_message(&room_id, &mm_message, &html_message).await);
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
                let html_message = format!(
                    "<h6>Src: Mastodon</h6><ul><li>Id: {}</li><li>From: @{}</li><li>Content:</li></ul>{}",
                    &new_last_id,
                    &nickname,
                    &content
                );
                debug!("Response: {:?}", matrix.post_message(&room_id, &mm_message, &html_message).await);
                zinc.publish(&json!([{
                    "src": "Mastodon",
                    "type": "pregunta",
                    "from": format!("@{}", &nickname),
                    "message": &parse_html(&content),
                }])).await.unwrap();
            }else if let Some(option) = check_comment("comentario", content){
                let (commentario, _reference) = option;
                if let Some(message) = commentario{
                    let feedback = Feedback::new("comentario", &new_last_id, &message, name, nickname, 0, "Mastodon");
                    feedback.post(url, token).await;
                    let thanks_message = format!("Gracias por tu comentario @{}", nickname);
                    mastodon.post(&thanks_message, Some(new_last_id.to_string())).await;
                let mm_message = format!("Src: Mastodon. From: @{}. Content: {}", &nickname, &parse_html(&content));
                let html_message = format!(
                    "<h6>Src: Mastodon</h6><ul><li>Id: {}</li><li>From: @{}</li><li>Content:</li></ul>{}",
                    &new_last_id,
                    &nickname,
                    &content
                );
                debug!("Response: {:?}", matrix.post_message(&room_id, &mm_message, &html_message).await);
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
                let html_message = format!(
                    "<h6>Src: Mastodon</h6><ul><li>Id: {}</li><li>From: @{}</li><li>Content:</li></ul>{}",
                    &new_last_id,
                    &nickname,
                    &content
                );
                debug!("Response: {:?}", matrix.post_message(&room_id, &mm_message, &html_message).await);
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
