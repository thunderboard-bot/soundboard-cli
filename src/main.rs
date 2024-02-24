mod config;
mod sounds;
mod api;
mod custom_error;

use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => {
            match config_matches.subcommand() {
                Some(("token", token_matches)) => {
                    let token = token_matches.get_one::<String>("token").unwrap();
                    println!("Setting token to {}", token);
                    let result = config::set_token(token.clone());
                    if result.is_err() {
                        println!("Failed to set token: {}", result.err().unwrap());
                    }
                },
                Some(("server", server_matches)) => {
                    let api_url = server_matches.get_one::<String>("server-url").unwrap();
                    println!("Setting server URL to {}", api_url);
                    // verify URL does not end in /
                    if api_url.ends_with("/") {
                        println!("server URL should not end in /");
                        return;
                    }
                    let result = config::set_server(api_url.clone());
                    if result.is_err() {
                        println!("Failed to set server URL: {}", result.err().unwrap());
                    }
                }
                _ => unreachable!()
            }
        },
        Some(("list", _)) => {
            println!("Available Sounds are: ");
            let res = sounds::list_sounds().await;
            if res.is_err() {
                println!("Failed to list sounds: {}", res.err().unwrap());
            }
        },
        Some(("play", play_matches)) => {
            let sound_id = play_matches.get_one::<String>("sound-id").unwrap();
            println!("Playing sound clip {}", sound_id);
            let res = sounds::play_sound(sound_id.clone()).await;
            if res.is_err() {
                println!("Failed to play sound: {}", res.err().unwrap());
            }
        },
        Some(("add", add_matches)) => {
            let sound_name = add_matches.get_one::<String>("sound-name").unwrap();
            let sound_file = add_matches.get_one::<String>("path/to/sound-file").unwrap();
            println!("Uploading sound clip {} from {}", sound_name, sound_file);
            let res = sounds::add_sound(sound_name.clone(), sound_file.clone()).await;
            if res.is_err() {
                println!("Failed to upload sound: {}", res.err().unwrap());
            }
        },
        _ => unreachable!()
    }
}

fn cli() -> Command {
    Command::new("thunderboard-cli")
        .version("0.2.2")
        .author("Max Pursian")
        .about("CLI to interact with the Thunderboard Discord bot")
        .subcommand_required(true)
        .subcommand(
            Command::new("config")
                .about("Configure settings")
                .subcommand(
                    Command::new("token")
                        .about("Set the access token")
                        .arg(
                            Arg::new("token")
                                .required(true)
                        ),
                )
                .subcommand(
                    Command::new("server")
                        .about("Set the server URL")
                        .arg(
                            Arg::new("server-url")
                                .required(true)
                        ),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all sound clips")
        )
        .subcommand(
            Command::new("play")
                .about("Play a sound clip")
                .arg(
                    Arg::new("sound-id")
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Upload a sound clip")
                .arg(
                    Arg::new("sound-name")
                        .required(true)
                )
                .arg(
                    Arg::new("path/to/sound-file")
                        .required(true)
                ),
        )
}