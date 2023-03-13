use std::env;
use std::path::Path;
use walkdir::WalkDir;
mod config;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Hash, Debug, PartialEq, Deserialize, Serialize)]
struct FSItem {
    // A File System Item structure
    file_extension: String,
    file_name: String,
    file_path: String,
    file_size: u64,
    key_words: Vec<String>,
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

fn set_key_words(path: &String, project: &String, file_name: &String) -> Vec<String> {
    // Tag an item with keywords
    let mut key_words: Vec<String> = vec![];
    for key_word in config::KEY_WORDS {
        if path.to_lowercase().contains(key_word) {
            key_words.push(key_word.to_string());
        }
    }
    key_words.push(project.to_string());
    key_words.push(file_name.to_string());
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
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let file_path = entry.path().to_str().unwrap().to_string();
        let project = (set_project(&file_path)).to_string();
        let fs_item = FSItem {
            file_extension: entry
                .path()
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_name: file_name.to_string(),
            file_path: file_path.clone(),
            file_size: entry.metadata().unwrap().len(),
            key_words: set_key_words(&file_path, &project, &file_name),
            project: project,
        };
        fs_items.push(fs_item)
    }
    fs_items
}

fn fts_search(query: &str) {
    // Full Text Search by given query string
    let file = File::open(Path::new(config::OUTPUT_FILENAME)).unwrap();
    let reader = BufReader::new(file);
    let fs_items: Vec<FSItem> = serde_json::from_reader(reader).unwrap();
    let re = Regex::new(&format!("{}", query)).unwrap();
    let mut matching_items: Vec<&FSItem> = vec![];
    let binding = re.to_string().to_lowercase();
    let query_words: Vec<&str> = binding.split_whitespace().collect();
    let _fs_items_response: Vec<&FSItem> = fs_items
        .iter()
        .filter(|&fs_item| {
            query_words
                .iter()
                .all(|&word| fs_item.key_words.contains(&(*word).to_string()))
        })
        .collect();
    matching_items.extend(_fs_items_response);
    for item in matching_items {
        println!("{:?}", item.file_path);
    }
}

fn build_index(path: &str) {
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

fn main() {
    // The main function
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[ERROR] No argument provided.");
        eprintln!("Help to build index: cargo run <path>");
        eprintln!("Help to search: cargo run search 'phrase to search'");
        std::process::exit(1);
    } else if args[1] == "search" {
        // Run search
        fts_search(&args[2]);
    } else {
        // Run build file system index
        let path = Path::new(&args[1]).to_string_lossy();
        build_index(&path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_project() {
        let path = String::from("/path/to/index/file.txt");
        let project = set_project(&path);
        assert_eq!(project, "");
    }

    #[test]
    fn test_set_key_words() {
        let path = String::from("/path/to/myproject/backend/Dockerfile");
        let project = String::from("myproject");
        let file_name = String::from("Dockerfile");
        let key_words = set_key_words(&path, &project, &file_name);
        assert!(key_words.contains(&String::from("backend")));
        assert!(key_words.contains(&String::from("docker")));
        assert!(key_words.contains(&String::from("myproject")));
        assert!(key_words.contains(&String::from("Dockerfile")));
    }

    #[test]
    fn test_index_directory() {
        let path = String::from("./test-data");
        let ignore_patterns = ["ignore.txt"];
        let fs_items = index_directory(path, &ignore_patterns);
        assert_eq!(fs_items.len(), 2);
        let _expected_items = [
            FSItem {
                file_extension: String::from("txt"),
                file_name: String::from("test1.txt"),
                file_path: String::from("./test-data/test1.txt"),
                file_size: 0,
                key_words: vec![],
                project: String::from(""),
            },
            FSItem {
                file_extension: String::from("txt"),
                file_name: String::from("test2.txt"),
                file_path: String::from("./test-data/test2.txt"),
                file_size: 0,
                key_words: vec![],
                project: String::from(""),
            },
        ];
        assert!(matches!(fs_items, _expected_items));
    }
}
