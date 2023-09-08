# CLI Bookmarks

Rust program to open a `sites.db` file.

It can add, remove and list entries in the `sites.db` file. 

Everything added to the database is encrypted by (a very weak) XOR encryption. 

## Usage
- Open a command line and navigate to the directory with the executable and the database file.
- Run ./clibookmarks and this will display the help menu.

## Building
- Clone this repo.
- Run `cargo build`. (Do not do `cargo run`. This will throw an error.)
- Move the compiled file (usually in `target/debug`) to a folder with the (`sites.db`)[https://github.com/tyropro/CLIBookmarks/releases/download/1.0.0/sites.db] file.

### Disclaimer
This is not meant to be secure. It's just meant to work :)