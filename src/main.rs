#![allow(dead_code)]
#![allow(unused_variables)]
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use serde::Deserialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
        let func = service_fn(func);
            lambda_runtime::run(func).await?;
                Ok(())
}
#[derive(Deserialize)]
struct SlackChallenge {
    token: String,
    challenge: String,
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
        let (event, _context) = event.into_parts();
        println!("event: {}", event);
        let body = event["body"].as_str().unwrap();
        let body: HashMap<String, Value> = serde_json::from_str(body).unwrap();
        println!("body: {:?}", body);
        if body["challenge"] != json!(null) {
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
            "".to_string()
        }
    }
}
async fn response_to_slack_event(event: &HashMap<String, Value>) {
    let event = &event["event"];
    let channel = &event["channel"].as_str().unwrap();
    let text = &event["text"].as_str().unwrap();
    let user_id = &event["user"].as_str().unwrap();
    let rusty_user_id = &event["authorizations"][0]["user_id"]
        .as_str()
        .unwrap_or_else(|| "U01UTH2J666");//rusty user id
    if event["bot_id"] == json!(null) {
        let mut message = String::new();
        if text.starts_with("Hey, Rusty") {
            message = format!("Hey, <@{}>!", user_id);
        }
        if message != String::new() {
            match async_post_message(channel, message.as_str()).await {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Error: Failed to send message");
                    ()
                }
            }
        } 
    } else {
        println!("Bot message! bot_id: {}\n", event["bot_id"]);
    }
}
async fn async_post_message(channel: &str, message: &str) -> Result<(), reqwest::Error> {
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
