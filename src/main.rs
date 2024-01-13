// This example uses the following crates:
// serde_json = "1.0"
// reqwest = "0.11.8"
// tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
// main.rs

mod requests;
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

#[tokio::main]
async fn main() {
    let animes = vec![
        "Re:Zero: Kara Hajimeru Isekai Seikatsu".to_string(),
        "Mushoku Tensei: Jobless Reincarnation".to_string(),
        "Tensei Shitara Slime Datta Ken".to_string(),
    ];
    let title = fzf(animes);
    let mut chosen_id = 0;
    if let Some(title) = title {
        match requests::make_graphql_request(&title).await {
            Ok(titles_and_ids) => {
                // Print the HashMap
                for (title, id) in titles_and_ids {
                    println!("ID: {}, Title: {}", &id, title);
                    chosen_id = id;
                }
            }
            Err(e) => eprintln!("Error making GraphQL request: {:?}", e),
        }
    } else {
        println!("No title selected");
    }
    println!("ID is: {}", &chosen_id);
    requests::print_info(chosen_id).await;
}
