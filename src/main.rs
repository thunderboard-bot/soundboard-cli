mod config;
mod sounds;
mod api;

use clap::{Arg, Command};

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => {
            match config_matches.subcommand() {
                Some(("token", token_matches)) => {
                    let token = token_matches.get_one::<String>("token").unwrap();
                    println!("Setting token to {}", token);
                    config::set_token(token.clone()).unwrap();
                },
                Some(("api", api_matches)) => {
                    let api_url = api_matches.get_one::<String>("api-url").unwrap();
                    println!("Setting API URL to {}", api_url);
                    config::set_server(api_url.clone()).unwrap();
                },
                _ => unreachable!()
            }
        },
        Some(("list", _)) => {
            println!("Listing all sound clips");
        },
        Some(("play", play_matches)) => {
            let sound_id = play_matches.get_one::<String>("sound-id").unwrap();
            println!("Playing sound clip {}", sound_id);
        },
        Some(("add", add_matches)) => {
            let sound_name = add_matches.get_one::<String>("sound-name").unwrap();
            let sound_file = add_matches.get_one::<String>("path/to/sound-file").unwrap();
            println!("Uploading sound clip {} from {}", sound_name, sound_file);
            sounds::add_sound(sound_name.clone(), sound_file.clone());
        },
        _ => unreachable!()
    }
}

fn cli() -> Command {
    Command::new("soundboard-cli")
        .version("1.0")
        .author("Max Pursian")
        .about("CLI to interact with the Discord soundboard bot")
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
                    Command::new("api")
                        .about("Set the API URL")
                        .arg(
                            Arg::new("api-url")
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