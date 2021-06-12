use std::env;
use std::thread;
use std::time::Duration;

use reqwest::StatusCode;

use serde::Serialize;

#[derive(Serialize)]
struct DiscordMessage {
    content: String
}

fn main() {
    println!("BUMP v0.1");
    let token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(e) => {
            use std::env::VarError;
            match e {
                VarError::NotPresent => {
                    println!("Fun fact: You can avoid pasting your token every time and just set the DISCORD_TOKEN environment variable!");
                    print!("Discord Token: ");
                    read_line()
                },
                VarError::NotUnicode(_) => {
                    eprintln!("DISCORD_TOKEN var is not unicode");
                    print!("Discord Token: ");
                    read_line()
                }
            }
        }
    };

    print!("Channel ID: ");
    let channel = read_line();

    loop {
        println!("Bumping...");

        let req = reqwest::blocking::Client::new()
            .post(format!("https://discord.com/api/v9/channels/{}/messages", channel))
            .header("Authorization", &token)
            .json(&DiscordMessage {
                content: String::from("!d bump")
            })
            .send()
            .unwrap();

        match req.status() {
            StatusCode::OK => println!("Bumped."),
            StatusCode::NOT_FOUND => panic!("Channel/Endpoint not found"),
            StatusCode::UNAUTHORIZED => panic!("Invalid Token"),
            StatusCode::FORBIDDEN => panic!("You do not have permission to send messages in the given channel"),
            StatusCode::TOO_MANY_REQUESTS => {
                eprintln!("Ratelimited, Retrying in 5 seconds...");
                thread::sleep(Duration::from_secs(5));
                continue;
            },
            status @ _ => eprintln!("Failed to Bump, Status Code: {}", status.as_str())
        }

        thread::sleep(Duration::from_secs(36000));
    }
}

fn read_line() -> String {
    use std::io;
    use std::io::Write;

    let mut text = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut text)
        .unwrap();

    text
}

