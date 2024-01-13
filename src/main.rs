// This example uses the following crates:
// serde_json = "1.0"
// reqwest = "0.11.8"
// tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
// Query to use in request
extern crate skim;
use skim::prelude::*;
use std::io::Cursor;

fn fzf(options: Vec<String>) -> Option<String> {
    let stringified_choice = options.join("\n");
    let _options_len = options.len();

    let skim_options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false) // Single selection
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(stringified_choice));

    let selected_items = Skim::run_with(&skim_options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    if !selected_items.is_empty() {
        // Skim returns the selected item(s)
        let selected_item = &selected_items[0];
        let selected_option = selected_item.output().to_string();
        Some(selected_option)
    } else {
        None
    }
}

const QUERY: &str = "
query ($id: Int, $page: Int, $perPage: Int, $search: String) {
    Page (page: $page, perPage: $perPage) {
        pageInfo {
            total
            currentPage
            lastPage
            hasNextPage
            perPage
        }
        media (id: $id, search: $search) {
            id
            title {
                romaji
                english
            }
        }
    }
}
";

#[tokio::main]
async fn main() {
    let client = Client::new();
    let title = "Re:Zero";
    // Define query and variables
    let json = json!(
        {
            "query": QUERY,
            "variables": {
                "search": title,
                "page": 1,
                "perPage": 3
            }
        }
    );
    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await;
    let parsed_json: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    // println!("Raw JSON Response: {:#}", resp.unwrap());

    let mut titles_and_ids: HashMap<String, u64> = HashMap::new();

    // Extract relevant information
    if let Some(media_array) = parsed_json["data"]["Page"]["media"].as_array() {
        for media_item in media_array {
            let id = media_item["id"].as_u64().unwrap_or_default();

            // Check if english_title is non-empty
            if let Some(english_title) = media_item["title"]["english"].as_str() {
                if !english_title.is_empty() {
                    // Insert title and ID into the HashMap
                    titles_and_ids.insert(english_title.to_string(), id);
                }
            }
        }
    } else {
        eprintln!("Invalid JSON format");
    }

    // Print the HashMap
    for (title, id) in titles_and_ids {
        println!("ID: {}, Title: {}", id, title);
    }
}
