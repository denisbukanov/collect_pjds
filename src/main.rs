extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use clap::Parser;
use html_escape;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use xmltree::Element;

/// Read file/stdin and find all staring with '<pjd>' and ending with '</pjd>'
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Shall we decode HTML entities before search
    #[clap(short = 'h', long)]
    html_decode: bool,

    /// Read data from standard input
    #[clap(short, long)]
    stdin: bool,

    /// Path to directory where results will be stored (will be created if does not exist)
    #[clap(short = 'o', long, default_value = ".")]
    output: String,

    /// Path to file to read data from
    input: Option<String>,
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer = buffer.trim().to_string(),
        Err(_) => buffer = String::new(),
    };
    return buffer;
}

fn read_file(path: Option<String>) -> Option<String> {
    match path {
        None => {
            println!("File path is empty");
            None
        },
        Some(path_) => {
            let mut src = String::new();
            if !Path::new(&path_).exists() {
                return None;
            }
            match File::open(path_.clone()) {
                Ok(mut f) => match f.read_to_string(&mut src) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Error while reading file '{}': {}", path_, err);
                        return None;
                    }
                },
                _ => {
                    println!("Error while reading file '{}'", path_);
                    return None;
                }
            };
            Some(src)
        }
    }
}

fn ensure_dir(dst: &String) -> Result<(), std::io::Error> {
    if !Path::new(dst).exists() {
        std::fs::create_dir_all(dst)?
    }
    Ok(())
}

fn store_to_file(path: &String, pjd: &String) -> Result<(), std::io::Error> {
    let dst_file = File::create(path.clone());
    match dst_file {
        Ok(mut dst) => dst.write_all(pjd.as_bytes()),
        Err(err) => Err(err),
    }
}

fn get_command(pjd: &String) -> String {
    Element::parse(pjd.as_bytes())
        .unwrap()
        .get_child("command")
        .unwrap()
        .get_text()
        .unwrap()
        .into_owned()
}

fn main() {
    let opening_tag: &'static str = "<pjd>";
    let closing_tag: &'static str = "</pjd>";
    let closing_tag_len = "</pjd>".len();

    let args = Args::parse();

    if args.stdin && args.input.is_some() {
        println!("Please provide path OR set stdin flag");
        return;
    }

    let mut source = match args.stdin {
        true => read_stdin(),
        false => match read_file(args.input) {
            None => {
                return;
            }
            Some(src) => {
                src
            }
        }
    };

    if args.html_decode {
        let mut dst = String::new();
        html_escape::decode_html_entities_to_string(source, &mut dst);
        source = dst;
    }

    let opening_tags: Vec<_> = source.match_indices(opening_tag).collect();
    let closing_tags: Vec<_> = source.match_indices(closing_tag).collect();
    assert_eq!(opening_tags.len(), closing_tags.len());

    let pjds: Vec<_> = opening_tags
        .iter()
        .zip(closing_tags.iter())
        .map(|content| {
            source
                .get(content.0 .0..content.1 .0 + closing_tag_len)
                .unwrap()
                .to_string()
        })
        .collect();

    match ensure_dir(&args.output) {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to create directory '{}': {}", args.output, err);
            return;
        }
    };
    let mut number: u16 = 0;
    for pjd in pjds.iter() {
        let command = get_command(&pjd);
        let path = format!("{}/{:#02}_{}", args.output, number, command);
        match store_to_file(&path, &pjd) {
            Ok(_) => println!("Done: {}", path),
            Err(err) => println!("Error while writing '{}': {}", path, err),
        };
        number += 1;
    }
}
