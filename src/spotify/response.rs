#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize)] pub struct TrackResponse { pub tracks: Items }
#[derive(Deserialize)] pub struct ArtistResponse { pub artists: Items }
#[derive(Deserialize)] pub struct Items { pub items: Vec<ExternalUrls> }
#[derive(Deserialize)] pub struct ExternalUrls { pub external_urls: Spotify }
#[derive(Deserialize)] pub struct Spotify { pub spotify: String }
