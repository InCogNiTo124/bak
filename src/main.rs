use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, author, long_about=None)]
struct Args {
    file: PathBuf,
}

// append .bak to the file name
// https://stackoverflow.com/a/76378247
fn append_to_path(p: PathBuf, s: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push(s);
    p.into()
}

fn copy_file(from_path: &PathBuf) -> Result<u64, std::io::Error> {
    let to_path = append_to_path(from_path.clone(), ".bak");
    fs::copy(from_path, to_path)
}

fn main() {
    let args = Args::parse();
    if let Err(error) = copy_file(&args.file) {
        println!("Error: {}", error);
    }
}
