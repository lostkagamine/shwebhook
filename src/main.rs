use std::path::Path;
use clap::Parser;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Parser)]
#[clap(author="Rin", version="1.0", about="A program to send a Discord webhook message via CLI", long_about=None)]
struct ProgramArgs {
    #[clap(raw=true, help="The message to be sent")]
    message: String,

    #[clap(short='p', long="profile", help="Profile override (see config)")]
    profile: Option<String>,

    #[clap(short='u', long="username", help="Username override")]
    username: Option<String>
}

#[derive(Deserialize, Debug)]
struct ConfigFile {
    default_url: String,
    profiles: Vec<Profile>,
    default_profile: String
}

#[derive(Deserialize, Debug)]
struct Profile {
    name: String,
    url: Option<String>,
    username: Option<String>,
    avatar: Option<String>
}

#[derive(Serialize)]
struct MessageJsonPayload<'a> {
    content: String,
    username: Option<&'a str>,
    avatar_url: Option<&'a str>
}

async fn send_message(msg: &str, url: &str, username: Option<&str>, avatar_url: Option<&str>) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let body = MessageJsonPayload {
        content: msg.into(),
        username: username,
        avatar_url: avatar_url
    };
    let body_json = serde_json::to_string(&body).unwrap();
    client.post(url)
        .body(body_json)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    Ok(())
}

const DEFAULT_CONFIG_FILE_PATH: &str = "/etc/shwebhook.toml";

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = ProgramArgs::parse();
    println!("{}", cli.message);
    let cfg_file_loc = match Path::new("./shwebhook.toml").exists() {
        true => "./shwebhook.toml",
        false => DEFAULT_CONFIG_FILE_PATH
    };
    let cfg = std::fs::read_to_string(cfg_file_loc).expect("error while reading config");
    let f: ConfigFile = toml::from_str::<ConfigFile>(&cfg).expect("bad config");

    // determine the profile
    let profile_name = match cli.profile {
        Some(s) => s,
        None => f.default_profile
    };

    // does the profile exist?
    let profile_opt = f.profiles.iter().find(|prf| prf.name == profile_name);
    if let None = profile_opt {
        println!("nonexistent profile {}", profile_name);
        return Err(());
    };
    let profile = profile_opt.unwrap();

    // determine the URL
    let url: &str = match &profile.url {
        Some(x) => x,
        None => &f.default_url
    };

    // determine username
    let username: Option<&str> = profile.username.as_deref();

    // determine avatar
    let avatar: Option<&str> = profile.avatar.as_deref();

    // send the message!
    match send_message(&cli.message, url, username, avatar).await {
        Ok(()) => (),
        Err(x) => panic!("{}", x)
    };


    Ok(())
}
