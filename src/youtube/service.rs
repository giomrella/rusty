use super::Response;

fn get_google_api_key() -> String {
    match std::env::var("GOOGLE_API_KEY") {
        Ok(key) => key,
        Err(_r) => {
            eprintln!("Error: Set environment variable GOOGLE_API_KEY");
            std::process::exit(1);
        }
    }
}

pub async fn search_youtube(term: &str) -> Result<String, reqwest::Error> {
        let key = get_google_api_key();
        let client = reqwest::Client::new();
        let response = client
                .get("https://youtube.googleapis.com/youtube/v3/search")
                .query(&[
                    ("q",term),
                    ("maxResults","1"),
                    ("key",&key)])
                .header("Accept", "application/json")
                .send().await?.text().await?;
        println!("response {}", response); 
        let res: Response = serde_json::from_str(&response).unwrap();
        let items = res.items;
        let response = match items.get(0) {
                Some(item) => format!("https://www.youtube.com/watch?v={}", item.id.videoId),
                None => "Never heard of it.".to_string()
        };
        Ok(response)
}

