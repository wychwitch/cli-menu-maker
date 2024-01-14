use console_menu::{Menu, MenuOption, MenuProps};
use dirs;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CommandItem {
    name: String,
    command: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Configuration {
    title: String,
    message: String,
    fg_color: u8,
    bg_color: u8,
    msg_color: u8,
    commands: Vec<CommandItem>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("You must pass in the path to a file")
    } else {
        let yaml_str = load_file(&args[1]);
        let config: Configuration = serde_yaml::from_str(&yaml_str).unwrap();
        let menu_options: Vec<MenuOption> = config
            .commands
            .into_iter()
            .map(|x| MenuOption::new(&x.name, move || f(&x.command)))
            .collect();

        let menu_options = menu_options;
        let mut menu = Menu::new(
            menu_options,
            MenuProps {
                title: &config.title,
                message: &config.message,
                fg_color: config.fg_color,
                bg_color: config.bg_color,
                msg_color: config.msg_color,
                ..MenuProps::default()
            },
        );
        menu.show();
    }
}

fn f(command: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .status()
            .expect("Error!");
    } else {
        Command::new("sh")
            .args(["-c", command])
            .status()
            .expect("Error!");
    };
}

//taken from https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust
fn load_file(file_path: &str) -> String {
    dbg!("{}", file_path);
    let file = File::open(file_path);

    match file {
        Ok(mut file) => {
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Unable to read file");
            contents
        }
        Err(e) => {
            panic!("{}", e)
        }
    }

    // iterate / process doc[s] ..
}
