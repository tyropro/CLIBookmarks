use colored::Colorize;
use rusqlite::{Connection, Result};
use std::env;

#[derive(Debug)]
struct Site {
    name: String,
    url: String,
}

fn xor_encrypt(key: &[u8], plaintext: &String) -> String {
    let mut ciphertext: String = String::new();
    let mut key_index: usize = 0;

    for byte in plaintext.bytes() {
        let key_byte: u8 = key[key_index];
        let ciphertext_byte: u8 = byte ^ key_byte;
        ciphertext.push(ciphertext_byte as char);

        key_index += 1;
        if key_index == key.len() {
            key_index = 0;
        }
    }

    ciphertext
}

fn xor_decrypt(key: &[u8], ciphertext: String) -> String {
    let mut plaintext: String = String::new();
    let mut key_index: usize = 0;

    for byte in ciphertext.bytes() {
        let key_byte: u8 = key[key_index];
        let output_byte: u8 = byte ^ key_byte;
        plaintext.push(output_byte as char);

        key_index += 1;
        if key_index == key.len() {
            key_index = 0;
        }
    }

    plaintext
}

fn main() -> Result<()> {
    let key: &[u8] = b"vKFRvlS3oN";

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!(
            "{}",
            "No arguments given. Valid use: clibookmarks [add/rm/list] [[name]] [[url]]".red()
        );
        return Ok(());
    }

    let raw_path = env::current_exe();
    let mut path: String = format!("{:?}", raw_path);
    path = path.replace("Ok(\"", "");
    path = path.replace("clibookmarks.exe\")", "");
    let db: String = format!("{}/sites.db", path);
    let connection: Connection = Connection::open(db)?;

    if args[1] == "add" {
        let plainname: &String = &args[2].to_lowercase();
        let ciphername: String = xor_encrypt(key, plainname);

        let plainurl: &String = &args[3];
        let cipherurl: String = xor_encrypt(key, plainurl);

        connection.execute(
            "INSERT INTO sites (name, url) values (?, ?)",
            [ciphername, cipherurl],
        )?;
    } else if args[1] == "list" {
        let mut stmt = connection.prepare("SELECT * FROM sites")?;

        let sites = stmt.query_map([], |row| {
            Ok(Site {
                name: xor_decrypt(key, row.get(0)?),
                url: xor_decrypt(key, row.get(1)?),
            })
        })?;

        let mut num: u8 = 100;
        let max: u8 = 255;

        for site in sites {
            let mut text: String = format!("{:?}", site);
            text = text.replace("Ok(Site { name: \"", "");
            text = text.replace("\", url: \"", "  ->  ");
            text = text.replace("\" })", "");
            if num > u8::MAX {
                num = 100;
            }
            println!("{}", text.truecolor(160, num, 255));
            if num + 25 > max {
                num = 100;
            } else {
                num += 30;
            }
        }
    } else if args[1] == "rm" {
        let param: String = xor_encrypt(key, &args[2].to_lowercase());
        connection.execute("DELETE FROM sites WHERE name == ?", [param])?;
    } else {
        println!("{}", "No valid arguments given!".red());
    }

    Ok(())
}
