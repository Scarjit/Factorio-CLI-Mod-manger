#![allow(clippy::print_stdout,clippy::missing_docs_in_private_items, clippy::panic, clippy::expect_used)]

#[macro_use]
extern crate prettytable;

use std::path::Path;

pub mod helper;
pub mod install;
pub mod json;
pub mod update;
pub mod verify;

fn main() {

    use clap::{load_yaml, App};

    #[allow(clippy::indexing_slicing)]
    let yml = load_yaml!("options.yml");
    let m = App::from(yml).get_matches();

    let serverfile_path = Path::new(if let Some(c) = m.value_of("serverpath") {
        c
    } else {
        "fctrserver/serverfiles"
    });

    if !Path::new(serverfile_path).exists() {
        panic!("serverpath does not exist !");
    }

    let mp_r = format!("{}\\mods", serverfile_path.to_str().expect("Couldn't parse path to string"));
    let mods_path = Path::new(&mp_r);
    let first_install = !mods_path.exists();
    if first_install {
        std::fs::create_dir(&mods_path).expect("Failed to create mod dir");
    }

    println!("Creating api-token");
    let api_token = helper::login(
        m.value_of("username").expect("Couldn't parse username"),
        m.value_of("password").expect("Couldn't parse password"),
    );

    if !crate::helper::get_all_mods() {
        return;
    }

    if first_install && (m.values_of("update").is_some() || m.values_of("verify").is_some()) {
        println!("First install detected, update & verify are not available");
    } else {
        #[allow(clippy::wildcard_enum_match_arm)]
        match m.subcommand_name() {
            Some("install") => {
                if let Some(matches) = m.subcommand_matches("install") {
                    match matches.values_of("mod") {
                        None => {
                            install::user_interaction(mods_path, &api_token);
                        }
                        Some(m) => {
                            install::install(
                                mods_path,
                                m.map(String::from).collect(),
                                &api_token,false
                            );
                        }
                    }
                }
            }
            Some("update") => {
                update::update(mods_path, &api_token);
            }
            Some("verify") => {
                verify::verify(mods_path, &api_token);
            }
            _ => {
                panic!("No subcommand was used !");
            }
        }
    }
}
