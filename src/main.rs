mod requests;
// Query to use in request
extern crate skim;
use skim::prelude::*;
use std::{fs, io::Cursor};

fn generate_dirs(dir_path: &str) -> Vec<String> {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let directories: Vec<_> = entries
                .filter_map(|entry| match entry {
                    Ok(entry) => {
                        if entry.path().is_dir() {
                            Some(entry.file_name().to_string_lossy().into_owned())
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
                .collect();

            directories
        }
        Err(err) => {
            eprintln!("Error reading directory {}: {:?}", dir_path, err);
            Vec::new()
        }
    }
}

fn fzf(options: Vec<String>) -> Option<String> {
    let stringified_choice = options.join("\n");
    let _options_len = options.len();

    let skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
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
    let animes = generate_dirs("/mnt/NAS/Anime");
    let title = fzf(animes);
    let mut chosen_id = 0;
    if let Some(title) = title {
        match requests::make_graphql_request(&title).await {
            Ok(titles_and_ids) => {
                let titles_and_ids: Vec<_> = titles_and_ids.into_iter().collect(); // Convert to Vec to allow multiple borrows
                for (_, _id) in &titles_and_ids {
                    let selection = fzf(Vec::from_iter(
                        titles_and_ids.iter().map(|(t, _)| t.clone()),
                    ));
                    if let Some(selected_title) = selection {
                        chosen_id = titles_and_ids
                            .iter()
                            .find(|(t, _)| t == &selected_title)
                            .unwrap()
                            .1;
                        break;
                    }
                }
            }
            Err(e) => eprintln!("Error making GraphQL request: {:?}", e),
        }
    } else {
        println!("No title selected");
    }
    // println!("ID is: {}", chosen_id);
    requests::print_info(chosen_id).await;
}
