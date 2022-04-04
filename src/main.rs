use std::fs::{self, File};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MATCH: &str = "class=\"tgme_page_additional\"";

    let users = fs::read_to_string("users.txt").expect("Error loading file");
    let mut of = File::create("valid_users.txt").unwrap();

    for user in users.lines() {
        if user.len() < 5 {
            continue;
        }
        let resp = reqwest::get(format!("https://t.me/{}/", user))
            .await?
            .text()
            .await?;
        if !resp.contains(MATCH) {
            println!("[ + ] t.me/{} -- FREE / BANNED", user);
            match writeln!(&mut of, "{}", format!("{}", user)) {
                Ok(r) => r,
                Err(_) => continue,
            };
        }
    }

    Ok(())
}
