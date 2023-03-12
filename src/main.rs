use std::collections::HashMap;
use std::env;
use std::path::Path;
use walkdir::WalkDir;
mod config;

#[derive(Hash, Eq, PartialEq, Debug)]
struct FSItem {
    // A File System Item structure
    file_extension: String,
    file_name: String,
    file_path: String,
    file_size: u64,
    key_words: Option<Vec<String>>,
    project: String,
}

impl FSItem {
    // Creates a new FSItem.
    fn new(
        file_extension: &str,
        file_name: &str,
        file_path: &str,
        file_size: u64,
        key_words: Option<Vec<String>>,
        project: Option<&str>,
    ) -> FSItem {
        FSItem {
            file_name: file_name.to_string(),
            file_path: file_path.to_string(),
            file_extension: file_extension.to_string(),
            file_size: file_size,
            key_words: key_words,
            project: project.expect("REASON").to_string(),
        }
    }
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

fn set_key_words(path: &String) -> Option<Vec<String>> {
    // Tag an item with keywords
    let mut key_words: Vec<String> = vec![];
    for key_word in config::KEY_WORDS {
        if path.to_lowercase().contains(key_word) {
            key_words.push(key_word.to_string());
        }
    }
    if !key_words.is_empty() {
        return Some(key_words);
    }
    None
}

fn index_directory(path: String, ignore_patterns: &[&str]) -> HashMap<String, FSItem> {
    // indexing a given directory path
    let mut fs_map = HashMap::new();
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
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let file_path = entry.path().to_str().unwrap().to_string();
        let file_extension = entry
            .path()
            .extension()
            .unwrap_or_default()
            .to_string_lossy();
        let file_size = entry.metadata().unwrap().len();
        let project = set_project(&file_path);
        let key_words = set_key_words(&file_path);
        fs_map.insert(
            file_path.clone(),
            FSItem::new(
                &file_extension,
                &file_name,
                &file_path,
                file_size,
                key_words,
                Some(project),
            ),
        );
    }
    fs_map
}

fn main() {
    // main function
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[ERROR] No path provided.");
        eprintln!("Help: cargo run <path>");
        std::process::exit(1);
    } else {
        let path = Path::new(&args[1]).to_string_lossy();
        let fs_map = index_directory(path.to_string(), &config::IGNORE_PATTERNS);
        for (_file_name, _file_path) in &fs_map {
            println!("{}: {:#?}", _file_name, _file_path);
        }
        println!("{}", fs_map.len());
    }
}
