# fs-index-rs

A file system indexing tool written in rust.

# Configurations

`cp src/config_template.rs src/config.rs`

In `config.rs` you can edit the following static environment variable:

`IGNORE_PATTERNS`: A list of ignore subpath to exclude from file system indexing

`PROJECTS`: A list of projects to populate the `project` indexing field.

`KEY_WORDS`: A list of key words to populate the `key_words` indexing field.

# Run

Indexing:

`cargo run /path/to/indexing`
