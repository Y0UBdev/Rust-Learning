use std::{fmt::format, fs::File, io::{self, BufReader, prelude::*}, pin::Pin};

use clap::{Parser, builder::Str};

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

#[derive(Debug)]
struct DataFile {
    words: Option<usize>,
    lines: Option<usize>,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    
    let data = wc(args.file, args.lines, args.words)?;
    
    display(data, args.lines, args.words);

    Ok(())
}

fn wc(file: String, lines: bool, words: bool) -> io::Result<DataFile> {
    let text = read(file)?;
    
    let mut cw: Option<usize> = None;
    let mut cl: Option<usize> = None;
    
    if words {cw = Some(word_count(&text))}
    if lines {cl = Some(line_count(&text))}

    Ok(DataFile {words: cw, lines: cl})
}

fn read(file: String) -> io::Result<String> {
    let file = File::open(&file)?;
    let mut bufreader = BufReader::new(file);
    let mut contents = String::new();
    bufreader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn word_count(text: &str) -> usize {
    let parts: Vec<&str> = text.split_whitespace().collect();
    return parts.len();
}

fn line_count(text: &str) -> usize {
    let parts: Vec<&str> = text.split('.').collect();
    return parts.len();
}

fn display(data: DataFile, lines: bool, words: bool) {
    let w= match data.words {
        Some(value) => format!("Words={value}"),
        None => String::from("Les mots non calculé"),
    };
    let l = match data.lines {
        Some(value) => format!("Lines={value}"),
        None => String::from("Les lignes non calculé"),
    }; 
    
    if lines & words { println!("{w}\n{l}") }
    else if words { println!("{w}") }
    else { println!("{l}") }
}