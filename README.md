# fs-index-rs

A file system indexing tool written in rust.

# Configurations

`cp config/config_template.toml config/config.toml`

In `config/config.toml` edit the following static environment variable:

- `debug`: A boolean to set debugging mode

- `ignore_patterns`: A list of ignore subpath to exclude from file system indexing

- `key_words`: A list of key words to populate the `key_words` indexing field.

- `output_filename`: A string to define the output path and file name.

- `projects`: A list of projects to populate the `project` indexing field.

# Install

`cargo install --path .`

# Run

## Indexing

`fs-index /path/to/indexing`

or

`cargo run /path/to/indexing`

## Search

`fs-index search "my key words"`

or

`cargo run search "my key words"`

# Benchmarks

Execution time:

```
$ time fs-index /home/trotto
[INFO] 314381 items saved into fs_index.json

real    0m3,811s
user    0m2,264s
sys     0m1,397s
```
