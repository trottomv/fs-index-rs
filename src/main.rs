use std::env;
use std::path::Path;
use walkdir::WalkDir;
mod config;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufWriter;

#[derive(Hash, Debug, Deserialize, Serialize)]
struct FSItem {
    // A File System Item structure
    file_extension: String,
    file_name: String,
    file_path: String,
    file_size: u64,
    key_words: Option<Vec<String>>,
    project: String,
}

fn set_project(path: &String) -> &'static str {
    // Tag an item with projects
    for project in config::PROJECTS {
        if path.to_lowercase().contains(project) {
            return project;
        }
    }
    ""
}

fn set_key_words(path: &String) -> Vec<String> {
    // Tag an item with keywords
    let mut key_words: Vec<String> = vec![];
    for key_word in config::KEY_WORDS {
        if path.to_lowercase().contains(key_word) {
            key_words.push(key_word.to_string());
        }
    }
    key_words
}

fn index_directory(path: String, ignore_patterns: &[&str]) -> Vec<FSItem> {
    // indexing a given directory path
    let mut fs_items = Vec::new();
    let walker = WalkDir::new(path).into_iter().filter_entry(|entry| {
        let file_path = entry.path().to_str().unwrap().to_string();
        !ignore_patterns
            .iter()
            .any(|pattern| file_path.contains(pattern))
    });
    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let file_path = entry.path().to_str().unwrap().to_string();
        let fs_item = FSItem {
            file_extension: entry
                .path()
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_name: entry.file_name().to_str().unwrap().to_string(),
            file_path: file_path.clone(),
            file_size: entry.metadata().unwrap().len(),
            key_words: Some(set_key_words(&file_path)),
            project: (set_project(&file_path)).to_string(),
        };
        fs_items.push(fs_item)
    }
    fs_items
}

fn main() {
    // build file system index
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[ERROR] No path provided.");
        eprintln!("Help: cargo run <path>");
        std::process::exit(1);
    } else {
        let path = Path::new(&args[1]).to_string_lossy();
        let fs_items = index_directory(path.to_string(), &config::IGNORE_PATTERNS);
        let output_file = File::create(config::OUTPUT_FILENAME).expect("Unable to create file");
        let bw = BufWriter::new(output_file);
        serde_json::to_writer(bw, &fs_items).expect("Failed writing :(");
        println!(
            "[INFO] {} items saved into {}",
            fs_items.len(),
            config::OUTPUT_FILENAME
        );
    }
}
