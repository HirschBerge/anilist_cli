use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use chrono::{DateTime, Local, TimeZone};

const QUERY_PAGES: &str = "
query ($id: Int, $page: Int, $perPage: Int, $search: String) {
    Page (page: $page, perPage: $perPage) {
        pageInfo {
            total
            currentPage
            lastPage
            hasNextPage
            perPage
        }
        media (id: $id, search: $search, type: ANIME) {
            id
            title {
                romaji
                english
            }
        }
    }
}
";
const QUERY_INFO: &str = "
query ($id: Int) { # Define which variables will be used in the query (id)
  Media (id: $id, type: ANIME) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    id
    title {
      romaji
      english
      native
    }
    status
    genres
    description
    averageScore
    seasonYear
    episodes
    nextAiringEpisode {
        airingAt
    }
  }
}
";

/// # Description
/// Used to make make a search API query to Anilist
///
/// Returns a `Result<HashMap<String, u64>, reqwest::Error>` containing the search results from AniList
///
/// # Usage
/// ```
/// let results = anilist_api_search("Darling in the FranXX");
///```
/// # Errors
///
/// This function will return an error if TBD
pub async fn anilist_api_search(title: &str) -> Result<HashMap<String, u64>, reqwest::Error> {
    let client = Client::new();

    // Define query and variables
    let json = json!(
        {
            "query": QUERY_PAGES,
            "variables": {
                "search": title,
                "page": 1,
                "perPage": 10
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
        .await?
        .text()
        .await?;

    let parsed_json: serde_json::Value = serde_json::from_str(&resp).unwrap();

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

    Ok(titles_and_ids)
}

pub async fn anilist_metadata_lookup(id: u64) {
    let client = Client::new();
    // Define query and variables
    let json = json!({"query": QUERY_INFO, "variables": {"id": id}});
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
    // Get json
    let mut result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    if let Some(airing_at_value) = result.pointer("/data/Media/nextAiringEpisode/airingAt") {
        if airing_at_value.is_number() {
            let timestamp = airing_at_value.as_i64().unwrap();
            if let Some(naive_datetime) =  Local.timestamp_opt(timestamp, 0).single() {
                let formatted_date = naive_datetime.format("%m-%d-%Y %H:%M").to_string();
                if let Some(next_airing_episode) = result.pointer_mut("/data/Media/nextAiringEpisode") {
                    next_airing_episode["airingAt"] = json!(formatted_date);
                }
            }
        } 
    }
    println!("{:#}", result);
}
