#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize)] 
pub struct Response {
        pub kind: String,
        pub etag: String,
        pub nextPageToken: String,
        pub regionCode: String,
        pub pageInfo: PageInfo,
        pub items: Vec<Item>
}
#[derive(Deserialize)] pub struct PageInfo { pub totalResults: u32, pub resultsPerPage: u32 }
#[derive(Deserialize)] pub struct Item { pub kind: String, pub etag: String, pub id: Id }
#[derive(Deserialize)] pub struct Id { pub kind: String, pub videoId: String }
