#![allow(dead_code)]
#![allow(unused_variables)]
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use serde::Deserialize;
use std::collections::HashMap;
use std::process;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Error> {
        let func = service_fn(func);
        lambda_runtime::run(func).await?;
        Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
        get_slack_api_key();
        get_spotify_api_key();
        let (event, _context) = event.into_parts();
        println!("event: {}\n", event);
        //let body = event.as_str().unwrap();
        let body = event["body"].as_str().unwrap();
        let body: HashMap<String, Value> = serde_json::from_str(body).unwrap();
        if body.contains_key("challenge") && body["challenge"] != json!(null) {
            return Ok(json!({ "challenge": body["challenge"] }));
        }
        response_to_slack_event(&body).await;
        return Ok(json!({ "message": "Success!" }));
}
fn get_slack_api_key() -> String {
    match std::env::var("SLACK_API_KEY") {
        Ok(key) => key,
        Err(_r) => {
            eprintln!("Error: Set environment variable SLACK_API_KEY");
            process::exit(1);
        }
    }
}
fn get_spotify_api_key() -> String {
    match std::env::var("SPOTIFY_CLIENT_CREDS_B64") {
        Ok(key) => key,
        Err(_r) => {
            eprintln!("Error: Set environment variable SPOTIFY_CLIENT_CREDS_B64");
            process::exit(1);
        }
    }
}
async fn response_to_slack_event(body: &HashMap<String, Value>)  -> Result<(), reqwest::Error> {
    //println!("body: {:?}\n", body);
    let event = &body["event"];
    //println!("event: {:?}\n", event);
    let channel = &event["channel"].as_str().unwrap();
    let text = &event["text"].as_str().unwrap();
    let user_id = &event["user"].as_str().unwrap();
    let rusty_user_id = &body["authorizations"][0]["user_id"]
        .as_str()
        .unwrap_or_else(|| "U01UTH2J666");//rusty user id
    if event["bot_id"] == json!(null) {
        let mut message = String::new();
        let re = Regex::new(r".*, spin that track ").unwrap();
        if re.is_match(text) {
            let track = re.replace_all(text, "").into_owned();
            message = search_spotify(&track).await?;
        }
        if message != String::new() {
            post_message(channel, message.as_str()).await?
        } 
    } else {
        println!("Bot message! bot_id: {}\n", event["bot_id"]);
    }
    Ok(())
}
async fn auth_spotify() -> Result<String, reqwest::Error> {
    let basic_creds = get_spotify_api_key();
    //println!("basic_creds {}", basic_creds); 
    let client = reqwest::Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", basic_creds))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[("grant_type","client_credentials")])
        .send().await?.text().await?;
    //println!("response {}", response); 
    let json: HashMap<String, Value> = serde_json::from_str(&response).unwrap();
    let token = &json["access_token"];
    let token = token.as_str().unwrap();
    //println!("token {}", token); 
    Ok(token.to_string())
}
#[derive(Deserialize)] struct SpotifyResponse { tracks: Items }
#[derive(Deserialize)] struct Items { items: Vec<ExternalUrls> }
#[derive(Deserialize)] struct ExternalUrls { external_urls: Spotify }
#[derive(Deserialize)] struct Spotify { spotify: String }

async fn search_spotify(term: &str) -> Result<String, reqwest::Error> {
    let token = auth_spotify().await?;
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/search")
        .query(&[
            ("q",term),
            ("type","track"),
            ("market","US"),
            ("limit","1")])
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send().await?.text().await?;
    println!("response {}", response); 
    let spotify_response: SpotifyResponse = serde_json::from_str(&response).unwrap();
    let url = &spotify_response.tracks.items[0].external_urls.spotify;
    Ok(url.to_string())
}
async fn post_message(channel: &str, message: &str) -> Result<(), reqwest::Error> {
    let token = get_slack_api_key();
    let mut body = HashMap::new();
    body.insert("channel", channel);
    body.insert("text", message);
    let client = reqwest::Client::new();
    client
        .post("https://slack.com/api/chat.postMessage")
        .json(&body)
        .header("Authorization", format!("Bearer {}", token))
        .send().await?;
    Ok(())
}
