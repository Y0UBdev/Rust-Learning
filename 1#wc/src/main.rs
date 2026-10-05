use std::{fs::File, io::{self, BufReader, prelude::*}};

use clap::Parser;

pub const ERR_OPEN_FILE: &str = "Erreur ! Echec de l'ouverture du fichier.";

pub const LINES: &str = ".";
pub const WORDS: &str = " ";

#[derive(Parser)]
#[command(name = "wc", about = "Compte le contenu d’un fichier")]
struct Args {
    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,

    file: String,
}

struct DataFile {
    words: usize,
    lines: usize,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    
    let data = wc(args.file, args.lines, args.words);
    print!("Word={}, Lines={}", data.word, data.lines);

    Ok(())
}

pub fn wc(file: String, lines: bool, words: bool) -> io::Result<DataFile> {
    let text = read(file)?;

    let c_words = count(&text, ' ');
    let c_lines = count(&text, '.');

    Ok(DataFile {
        words: c_words,
        lines: c_lines,
    })
}

pub fn read(file: String) -> io::Result<String> {
    let file = File::open(&file)?;
    let mut bufreader = BufReader::new(file);
    let mut contents = String::new();
    bufreader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn count(text: &str, pattern: char) -> usize {
    let parts: Vec<&str> = text.split(pattern).collect();
    return parts.len();
}