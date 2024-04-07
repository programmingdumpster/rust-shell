#![windows_subsystem = "console"]

use std::io::{ self, Read, Write };
use std::process::Command;
use std::fs::{ self, File };
use std::env;
use walkdir::WalkDir;
use std::io::{ BufRead, BufReader };

fn main() {
    let _ = Command::new("clear").status();
    cmds();
    loop {
        print!("\n {:?} {:?} #  ", whoami::username(), env::current_dir().unwrap());
        io::stdout().flush().expect("flushing err");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input err");

        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "quit" | "exit" => {
                println!("");
                break;
            }

            "echo" => {
                echo(args);
            }

            "clear" => {
                let _ = Command::new("clear").status();
            }

            "cat" => {
                if args.len() == 3 {
                    cat(args[0], args[1], args[2]);
                } else {
                    println!("Usage: grep [text] [file.txt]");
                }
            }

            "grep" => {
                if args.len() == 2 {
                    let _ = grep(args[0], args[1]);
                } else {
                    println!("Usage: grep [text] [file.txt]");
                }
            }

            "cd" => {
                if args.len() == 1 {
                    let _ = cd(args[0]);
                } else {
                    let _ = cd("/");
                }
            }

            "cmds" => {
                cmds();
            }

            "find" => {
                if args.len() == 1 {
                    find(args[0]);
                } else {
                    println!("Usage: find [filename] ");
                }
            }

            "ls" => {
                if args.len() == 1 {
                    ls(args[0]);
                } else {
                    let dir = env::current_dir().unwrap().to_string_lossy().to_string();
                    ls(&dir);
                }
            }

            _ => {
                println!("wtf is this command: {}", command);
            }
        }
    }
}

fn echo(args: &[&str]) {
    let text = args.join(" ");

    println!("{}", text)
}

fn cat(_file1: &str, _file2: &str, _name: &str) {
    let mut file1 = File::open(_file1).expect("file open err");
    let mut contents1 = String::new();
    file1.read_to_string(&mut contents1).expect("reading err");

    let mut file2 = File::open(_file2).expect("file open err");
    let mut contents2 = String::new();
    file2.read_to_string(&mut contents2).expect("reading err");

    let mut fileout = File::create(_name).expect("creating err");
    let data = contents1 + &contents2;
    fileout.write_all(data.as_bytes()).expect("cant write err")
}

fn ls(path: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(file_name) = file_name.to_str() {
                    print!(" {} ", file_name);
                }
            }
        }
    } else {
        println!("Blad odczytu folderu ");
    }
}

fn find(item: &str) {
    let dir_path = "/";
    let dir_entires = WalkDir::new(dir_path);

    for entry in dir_entires.into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_string_lossy().to_lowercase() == item.to_lowercase() {
            let path = entry.path().to_string_lossy().to_string();
            println!("{}", path);
        }
    }
}

fn grep(text: &str, file: &str) -> io::Result<()> {
    // Otwórz plik
    let _file1 = File::open(file)?;
    let reader = BufReader::new(_file1);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains(text) {
            println!("{}:{}, {}", file, line_number, line);
        }
    }

    Ok(())
}

fn cmds() {
    println!(
        "

       CMDS: 
            quit / exit
            echo [input]
            clear - clears console
            cat [file.text] [file.txt] [name]
            grep [text] [file.txt]
            find [file]
            ls [path]
            cd [dir]
            cmds - this output

    "
    );
}

fn cd(dir: &str) -> Result<(), std::io::Error> {
    env::set_current_dir(dir)
}
