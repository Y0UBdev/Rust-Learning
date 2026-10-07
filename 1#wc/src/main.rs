use std::{fs::File, io::{self, BufReader, prelude::*}, path::Path};

use clap::{Parser};

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

    let file = args.file;

    let lines = args.lines || (!args.lines && !args.words);
    let words = args.words || (!args.lines && !args.words);

    let data = wc(&file, lines, words)?;
    
    display(data, &file, lines, words);

    Ok(())
}


fn wc(file: &str, lines: bool, words: bool) -> io::Result<DataFile> {
    let text = read(file)?;
    
    let mut cw: Option<usize> = None;
    let mut cl: Option<usize> = None;
    
    if words {cw = Some(word_count(&text))}
    if lines {cl = Some(line_count(&text))}

    Ok(DataFile {words: cw, lines: cl})
}


fn read(file: &str) -> io::Result<String> {
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


fn display(data: DataFile, file: &str, lines: bool, words: bool) {
    let w= match data.words {
        Some(value) => format!("Words={value}"),
        None => String::from("Les mots non calculé"),
    };
    let l = match data.lines {
        Some(value) => format!("Lines={value}"),
        None => String::from("Les lignes non calculé"),
    };
    
    let name = Path::new(file)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();    

    println!("Données du fichier {}", name);
    
    if words & !lines { println!("{w}") }
        else if lines & !words { println!("{l}") }
        else { println!("{w}\n{l}") }
}
