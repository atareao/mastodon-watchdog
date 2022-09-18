use reqwest;
use oauth::Token;
use std::{format, collections::HashMap};

static BASE_URI: &str = "https://api.twitter.com";

#[derive(oauth::Request)]
struct SearchTweets<'a>{
    q: &'a str,
    include_entities: bool,
    result_type: &'a str,
}

#[derive(oauth::Request)]
struct Tweet{
    status: String,
}

impl Tweet{
    pub fn new(message: &str)->Self{
        Tweet{status: message.to_string()}
    }
    pub fn get_params(self) -> HashMap<String, String>{
        let mut result = HashMap::new();
        result.insert("status".to_string(), self.status.to_string());
        result
    }
}

pub struct Twitter{
    token: Token,
}

impl Twitter{
    pub fn new(client_identifier: &str, client_secret: &str, token: &str, token_secret: &str)->Self{
        Twitter{
            token: oauth::Token::from_parts(
                client_identifier.to_string(),
                client_secret.to_string(),
                token.to_string(),
                token_secret.to_string()),
        }
    }
    pub async fn tweet(self, message: &str){
        let uri = format!("{}/1.1/statuses/update.json", BASE_URI);
        let request = Tweet::new(message);
        let authorization_header = oauth::post(&uri, &request, &self.token,
            oauth::HMAC_SHA1);
        let client = reqwest::Client::new();
        let res = client
            .post(&uri)
            .header("Authorization", authorization_header)
            .form(&request.get_params())
            .send()
        .await;
    }
}
