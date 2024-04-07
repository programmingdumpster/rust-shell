#![windows_subsystem = "console"]

#[warn(unused_imports)]
use std::io::{ self, Read, Write };
use std::process::Command;
use std::fs::{ self, File };
use std::env::{ self, set_current_dir };
use walkdir::WalkDir;
use std::io::{ BufRead, BufReader, Result };
use home::home_dir;

fn main() {
    cmds(); // Display available commands
    loop {
        print!("\n {:?} {:?} #  ", whoami::username(), env::current_dir().unwrap()); // Display username and current directory
        io::stdout().flush().expect("flushing err"); // Flush output buffer

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input err"); // Read user input

        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect(); // Split input into parts

        match parts.get(0) {
            Some(command) => {
                let args = &parts[1..];
                match *command {
                    "quit" | "exit" => {
                        // Quit the program
                        println!();
                        break;
                    }

                    "echo" => {
                        // Echo the input
                        echo(args);
                    }

                    "clear" => {
                        // Clear the console
                        let _ = Command::new("clear").status();
                        let _ = Command::new("cls").status();
                    }

                    "cat" => {
                        // Concatenate two files
                        if args.len() == 3 {
                            match cat(args[0], args[1], args[2]) {
                                Ok(_) => {}
                                Err(error) => eprintln!("Merging error: {}", error),
                            }
                        } else {
                            println!("Usage: grep [text] [file.txt]");
                        }
                    }

                    "cd" => {
                        // Change directory
                        if args.len() == 1 {
                            match cd(args[0]) {
                                Ok(_) => {}
                                Err(error) => eprintln!("{}", error),
                            }
                        } else {
                            cd("").expect("dir not exists");
                        }
                    }

                    "grep" => {
                        // Search for text in a file
                        if args.len() == 2 {
                            let _ = grep(args[0], args[1]);
                        } else {
                            println!("Usage: grep [text] [file.txt]");
                        }
                    }

                    "cmds" => {
                        // Display available commands
                        cmds();
                    }

                    "run" => {
                        // Run an executable
                        if args.len() == 1 {
                            run(args[0]);
                        } else {
                            println!("Usage: run [executable] ");
                        }
                    }

                    "find" => {
                        // Find a file
                        if args.len() == 1 {
                            find(args[0]);
                        } else {
                            println!("Usage: find [filename] ");
                        }
                    }

                    "ls" => {
                        // List files in a directory
                        if args.len() == 1 {
                            match ls(args[0]) {
                                Ok(()) => {}
                                Err(error) => eprintln!("something fucked up: {}", error),
                            }
                        } else {
                            let dir = env::current_dir();
                            match ls(&dir.unwrap().to_string_lossy().to_string()) {
                                Ok(()) => {}
                                Err(error) => eprintln!("something fucked up: {}", error),
                            }
                        }
                    }

                    _ => {
                        println!("wtf is this command: {}", command);
                    }
                }
            }
            None => println!(),
        }
    }
}

fn echo(args: &[&str]) {
    // Echo function
    let text = args.join(" ");

    println!("{}", text)
}

fn cat(file1: &str, file2: &str, name: &str) -> io::Result<()> {
    // Concatenate two files
    let mut file1 = File::open(file1)?;
    let mut contents1 = String::new();
    file1.read_to_string(&mut contents1)?;

    let mut file2 = File::open(file2)?;
    let mut contents2 = String::new();
    file2.read_to_string(&mut contents2)?;

    let mut fileout = File::create(name)?;

    let data = contents1 + &contents2;

    fileout.write_all(data.as_bytes())?;

    Ok(())
}

fn ls(path: &str) -> Result<()> {
    // List files in a directory
    let mut column_w = 0;
    let mut filenames = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let filename_str = entry.file_name().to_string_lossy().to_string();
        column_w = column_w.max(filename_str.len());
        filenames.push(filename_str);
    }

    let terminal_w = term_size::dimensions().map_or(80, |(w, _)| w);
    let n_colums = terminal_w / (column_w + 2);

    for (i, filename) in filenames.into_iter().enumerate() {
        print!("{:width$}", filename, width = column_w);
        if (i + 1) % n_colums == 0 {
            println!();
        } else {
            print!("   ");
        }
    }

    Ok(())
}

fn find(item: &str) {
    // Find a file
    let dir_path = "/";
    let dir_entires = WalkDir::new(dir_path);

    for entry in dir_entires.into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_string_lossy().to_lowercase() == item.to_lowercase() {
            let path = entry.path().to_string_lossy().to_string();
            println!("{:<6}", path);
        }
    }
}

fn grep(text: &str, file: &str) -> io::Result<()> {
    // Search for text in a file
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
    // Display available commands
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

fn cd(dir: &str) -> Result<()> {
    // Change directory
    let home = home_dir().unwrap().to_string_lossy().to_string();

    match dir {
        ".." => set_current_dir("..")?,
        "" => set_current_dir(home)?,
        _ => set_current_dir(dir)?,
    }
    Ok(())
}

fn run(executable: &str) {
    // Run an executable
    let _ = Command::new("./".to_owned() + executable).status();
}
