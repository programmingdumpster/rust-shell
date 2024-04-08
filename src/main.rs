#![windows_subsystem = "console"]

mod commands;
use std::io::{ self, Write };
use std::path::PathBuf;
use std::process::Command;
use std::env::{ self };
use commands::*;
use sysinfo::System;

fn main() {
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
                        match args.len() {
                            1 => if let Err(error) = echo(args) {
                                eprintln!("Directory reading error: {}", error);
                            }
                            _ => println!("Usage: echo [text]"),
                        }
                    }

                    "cat" => {
                        // Merges files
                        match args.len() {
                            3 => if let Err(error) = cat(args[0], args[1], args[2]) {
                                eprintln!("Merging error: {}", error);
                            }
                            _ => println!("Usage: cat [file1] [file2] [output_file]"),
                        }
                    }

                    "cd" => {
                        // Change directory
                        match args.len() {
                            1 => if let Err(error) = cd(args[0]) {
                                eprintln!("Dir changing error: {}", error);
                            }
                            _ => println!("Usage: cd [directory] "),
                        }
                    }

                    "grep" => {
                        // Search for text in a file
                        match args.len() {
                            2 => if let Err(error) = grep(args[0], args[1]) {
                                eprintln!("Grep error: {}", error);
                            }
                            _ => println!("Usage: grep [text] [file.txt]"),
                        }
                    }

                    "find" => {
                        // Find a file
                        match args.len() {
                            2 => if let Err(error) = find(args[0], PathBuf::from(args[1])) {
                                eprintln!("finding error: {}", error);
                            }
                            _ => println!("Usage: find [item]"),
                        }
                    }

                    "ls" => {
                        // List files in a directory

                        match args.len() {
                            1 => if let Err(error) = ls(PathBuf::from(args[0])) {
                                eprintln!("File reading error: {}", error);
                            }

                            0 => if let Err(error) = ls(PathBuf::from(env::current_dir().unwrap())) {
                                eprintln!("File reading error: {}", error);
                            }

                            _ => println!("Usage: grep [text] [file.txt]"),
                        }
                    }

                    "cmds" => {
                        // Display available commands
                        println!(
                            "
                    
                           CMDS: 
                                quit / exit
                                echo [input]
                                clear - clears console
                                cat [file.text] [file.txt] [output file name]
                                grep [text] [file]
                                find [file] [directory]
                                ls [directory]
                                cd [directory]
                                cmds - this output
                    
                        "
                        );
                    }

                    "clear" => {
                        // Clear the console
                        match std::env::consts::OS {
                            "windows" => {
                                // WIN
                                let _ = Command::new("CLS").status();
                            }
                            "linux" | "macos" => {
                                // UNIX
                                let _ = Command::new("clear").status();
                            }
                            _ => {
                                // Forbidden OS
                                println!(
                                    "ur system: {}",
                                    System::long_os_version().unwrap().to_string()
                                );
                            }
                        }
                    }

                    _ => {
                        println!("Unknown command: {}", command);
                    }
                }
            }
            None => println!(),
        }
    }
}
