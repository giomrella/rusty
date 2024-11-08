#![allow(dead_code)]
#![allow(unused_variables)]
use lambda_runtime::{service_fn, LambdaEvent};
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
mod spotify;
mod youtube;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(service_fn(func)).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    get_slack_api_key();
    let (event, _context) = event.into_parts();
    println!("event: {}\n", event);
    //let body = event.as_str().unwrap();
    let body = event["body"].as_str().unwrap();
    let body: HashMap<String, Value> = serde_json::from_str(body).unwrap();
    if body.contains_key("challenge") && body["challenge"] != json!(null) {
        return Ok(json!({ "challenge": body["challenge"] }));
    }
    respond_to_slack_event(&body).await?;
    return Ok(json!({ "message": "Success!" }));
}

fn get_slack_api_key() -> String {
    match std::env::var("SLACK_API_KEY") {
        Ok(key) => key,
        Err(_r) => {
            eprintln!("Error: Set environment variable SLACK_API_KEY");
            std::process::exit(1);
        }
    }
}
async fn respond_to_slack_event(body: &HashMap<String, Value>) -> Result<(), reqwest::Error> {
    //println!("body: {:?}\n", body);
    let event = &body["event"];
    //println!("event: {:?}\n", event);
    let channel = &event["channel"].as_str().unwrap();
    let text = &event["text"].as_str().unwrap();
    let user_id = &event["user"].as_str().unwrap();
    let rusty_user_id = &body["authorizations"][0]["user_id"]
        .as_str()
        .unwrap_or_else(|| "U01UTH2J666"); //rusty user id
    if event["bot_id"] == json!(null) {
        let mut message = String::new();
        let regex_video = Regex::new(r".*, play that video ").unwrap();
        if regex_video.is_match(text) {
            let video = regex_video.replace_all(text, "").into_owned();
            message = youtube::search_youtube(&video).await?;
        }
        let regex_track = Regex::new(r".*, spin that track ").unwrap();
        if regex_track.is_match(text) {
            let track = regex_track.replace_all(text, "").into_owned();
            message = spotify::search_spotify(&track, spotify::SearchType::Track).await?;
        }
        let regex_song = Regex::new(r".*, play that song ").unwrap();
        if regex_song.is_match(text) {
            let track = regex_song.replace_all(text, "").into_owned();
            message = spotify::search_spotify(&track, spotify::SearchType::Track).await?;
        }
        let regex_artist = Regex::new(r".*, play something by ").unwrap();
        if regex_artist.is_match(text) {
            let artist = regex_artist.replace_all(text, "").into_owned();
            message = spotify::search_spotify(&artist, spotify::SearchType::Artist).await?;
        }
        let regex_tiktok = Regex::new(r"https?://(?:www\.)?tiktok\.com/[^ \n]+").unwrap();
        if regex_tiktok.is_match(text) {
            message = fix_tiktok_link(regex_tiktok.find(text).unwrap().as_str()).await?;
        }
        if message != String::new() {
            post_message(channel, message.as_str(), event["thread_ts"].as_str()).await?
        }
    } else {
        println!("Bot message! bot_id: {}\n", event["bot_id"]);
    }
    Ok(())
}

async fn fix_tiktok_link(url: &str) -> Result<String, reqwest::Error> {
    let url = "https://www.tiktok.com/t/ZP8LjaWQK/";
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let res = client.get(url).send().await?;
    let url = if let Some(location) = res.headers().get("location") {
        location.to_str().unwrap_or_default()
    } else {
        url
    }
    ;
    let url = url.split("?").into_iter().next().unwrap_or_default();
    Ok(url.to_string())
}

async fn post_message(
    channel: &str,
    message: &str,
    thread_ts: Option<&str>,
) -> Result<(), reqwest::Error> {
    let token = get_slack_api_key();
    let mut body = HashMap::new();
    body.insert("channel", channel);
    body.insert("text", message);
    if let Some(thread_ts) = thread_ts {
        body.insert("thread_ts", thread_ts);
    }
    let client = reqwest::Client::new();
    client
        .post("https://slack.com/api/chat.postMessage")
        .json(&body)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn get_tiktok_url() -> Result<(), reqwest::Error> {
        let url = "https://www.tiktok.com/t/ZP8LjaWQK/";
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let res = client.get(url).send()?;
        println!("status: {:?}", res.status());
        println!("headers: {:?}", res.headers());
        let url = if let Some(location) = res.headers().get("location") {
            location.to_str().unwrap_or_default()
        } else {
            url
        }
        ;
        println!("Response: {url}");
        // println!("text: {}", res.text().await?);// this was a mess of text
        Ok(())
    }
}
