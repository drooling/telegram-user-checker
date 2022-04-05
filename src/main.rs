use std::any::{Any, TypeId};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path;
use std::process;
use nanoid::nanoid;

fn get_outfile() -> std::fs::File {
    if !path::Path::new("./results").exists() {
        fs::create_dir("results").expect("Coudlnt create `results` dir");
    }
    if path::Path::new("./results/free-users.txt").exists() {
        fs::rename(
            "./results/free-users.txt",
            format!(
                "./results/free-users-{}.txt",
                nanoid!(5, &nanoid::alphabet::SAFE)
            ),
        )
        .expect("Couldnt rename existing `free-users.txt` file");
        return File::create("./results/free-users.txt").unwrap();
    } else {
        return File::create("./results/free-users.txt").unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MATCH: &str = "class=\"tgme_page_additional\"";
    const INVALID_CHARS: [char; 32] = [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];

    ctrlc::set_handler(move || {
        println!("[ X ] -> Exit signal received");
        process::exit(0);
    }).expect("Couldn't register exit signal handler");

    let args: Vec<String> = env::args().collect();
    let users_file;
    if args.len() > 1 {
        users_file = String::from(&args[1]);
    } else {
        users_file = String::from("users.txt");
    }

    let users = fs::read_to_string(&users_file).expect("Error loading input file");
    let mut of = get_outfile();

    println!("[ ! ] -> Loaded ( {} ) Usernames", &users.lines().count());

    for user in users.lines() {
        if user.len() < 5 {
            continue;
        } else if !user.is_ascii() {
            continue;
        } else if user.chars().any(|c| INVALID_CHARS.contains(&c)) {
            continue;
        }
        let resp = reqwest::get(format!("https://t.me/{}", user))
            .await?
            .text()
            .await?;
        if resp.type_id() == TypeId::of::<reqwest::Error>() {
            println!("[ + ] -> t.me/{} -- CONNECTION ERROR", user);
            continue;
        }
        if !resp.contains(MATCH) {
            println!("[ + ] -> t.me/{} -- FREE / BANNED", user);
            match writeln!(of, "{}", format!("{}", user)) {
                Ok(r) => r,
                Err(_) => continue,
            };
        }
    }

    println!("[ ! ] -> Finished checking {}", users_file);

    Ok(())
}
