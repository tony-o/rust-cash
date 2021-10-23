use std::{env, fs};
use std::process::Command;
use std::path::Path;

fn main() {
    let mut dir: String = "/tmp/cash".to_string();
    let mut name: String = "last".to_string();
    let mut config: Vec<String> = env::args().collect();
    let mut name_changed: bool = false;

    config.remove(0);
    while !config.is_empty() {
        let arg = config.remove(0);
        
        if arg == "-d" || arg == "--dir" {
            if config.is_empty() {
                eprintln!("{:?} requires an argument", arg);
                std::process::exit(1);
            }
            dir = config.remove(0);
            if !Path::new(&dir).exists() {
                eprintln!("{:?} does not exist", dir);
                std::process::exit(1);
            }
        } else if arg == "-n" || arg == "--name" {
            if config.is_empty() {
                eprintln!("{:?} requires an argument", arg);
                std::process::exit(1);
            }
            name = config.remove(0);
            name_changed = true;
        } else {
            config.insert(0, arg);
            break;
        }
    }

    let full_dir: String = (dir + "/" + if name_changed { "named/" } else { "" }).clone();
    let full_path: String = full_dir.clone() + &name;
    match std::fs::create_dir_all(full_dir.clone()) {
        Err(_) => {
            eprintln!("error creating cache directory {:?}", full_dir);
            std::process::exit(1);
        },
        Ok(_) => {},
    }

    if config.is_empty() {
        let path = Path::new(&full_path);
        if !path.exists() {
            eprintln!("no $$$ for {:?} found", name);
            std::process::exit(2);
        }
        print!("{}", fs::read_to_string(path).unwrap());

        return;
    }
    

    let cmd = Command::new(config.remove(0)).args(config.clone()).output();
    match cmd {
        Ok(o)  => {
            fs::write(Path::new(&full_path), o.stdout.clone()).unwrap();
            print!("{}", String::from_utf8(o.stdout).unwrap());
        },
        Err(e) => println!("err={:?}", e),
    }
    
}
