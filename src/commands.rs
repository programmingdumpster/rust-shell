use std::{
    env::set_current_dir,
    fs::{ self, File },
    io::{ self, BufRead, BufReader, Read, Write },
    path::PathBuf,
};

#[allow(unused_imports)]
use nu_ansi_term::Color::*;
use walkdir::WalkDir;

pub fn echo(args: &[&str]) -> io::Result<()> {
    // Echo function
    let text = args.join(" ");

    println!("{}", text);

    Ok(())
}

pub fn cat(file1: &str, file2: &str, name: &str) -> io::Result<()> {
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

pub fn ls(path: PathBuf) -> io::Result<()> {
    // List files in a directory
    let mut column_w = 0;
    let mut filenames = Vec::new();
    let entries = fs::read_dir(path)?;
    //Pushing names to vector, getting width for column
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

pub fn find(item: &str, dir: PathBuf) -> io::Result<()> {
    // Find a file

    let dir_entires = WalkDir::new(dir);

    for entry in dir_entires.into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_string_lossy().to_lowercase() == item.to_lowercase() {
            let path = entry.path().to_string_lossy().to_string();
            println!("{:<6}", path);
        }
    }
    Ok(())
}

pub fn grep(text: &str, file: &str) -> io::Result<()> {
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

pub fn cd(dir: &str) -> io::Result<()> {
    // Change directory
    let home = home::home_dir().unwrap().to_string_lossy().to_string();

    match dir {
        ".." => set_current_dir("..")?,
        "" => set_current_dir(home)?,
        _ => set_current_dir(dir)?,
    }
    Ok(())
}
