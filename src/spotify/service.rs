use serde_json::Value;
use std::collections::HashMap;
use super::SearchType;
use super::{TrackResponse,ArtistResponse};

fn get_spotify_api_key() -> String {
    match std::env::var("SPOTIFY_CLIENT_CREDS_B64") {
        Ok(key) => key,
        Err(_r) => {
            eprintln!("Error: Set environment variable SPOTIFY_CLIENT_CREDS_B64");
            std::process::exit(1);
        }
    }
}

async fn auth_spotify() -> Result<String, reqwest::Error> {
    let basic_creds = get_spotify_api_key();
    //println!("basic_creds {}", basic_creds); 
    let response = reqwest::Client::new()
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

pub async fn search_spotify(term: &str, search_type: SearchType) -> Result<String, reqwest::Error> {
    let token = auth_spotify().await?;
    let response = reqwest::Client::new()
        .get("https://api.spotify.com/v1/search")
        .query(&[
            ("q",term),
            ("type",&search_type.to_string().to_lowercase()),
            ("market","US"),
            ("limit","1")])
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send().await?.text().await?;
    println!("response {}", response); 
    let items = match search_type {
        SearchType::Track => {
            let res: TrackResponse = serde_json::from_str(&response).unwrap();
            //res.tracks.items[0].external_urls.spotify.to_owned()
            res.tracks.items
        },
        SearchType::Artist => {
            let res: ArtistResponse = serde_json::from_str(&response).unwrap();
            //res.artists.items[0].external_urls.spotify.to_owned()
            res.artists.items
        }
    };
    let response = match items.get(0) {
        Some(item) => item.external_urls.spotify.to_owned(),
        None => "Never heard of it.".to_string()
    };
    Ok(response)
}

