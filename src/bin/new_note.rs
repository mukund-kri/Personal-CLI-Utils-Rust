use std::fs::create_dir;

use chrono::prelude::*;
use clap::Parser;
use convert_case::{Case, Casing};

// Command line arguments
#[derive(Parser)]
struct Cli {
    // The only thing I need is the title of the note
    title: String,
}

fn main() {
    // get today's date and format so that I can prefix to the note directory
    let today = Local::now();
    let today_str = today.format("%Y-%m-%d").to_string();

    // Parse the command line arguments
    let args = Cli::parse();

    // Create the note directory
    let note_dir = format!("{}-{}", today_str, args.title);
    create_dir(&note_dir).expect("Failed to create note directory");

    // The notes in markdown format
    let note_md = format!("{}/{}.md", &note_dir, args.title,);
    // Touch the note file
    std::fs::File::create(&note_md).expect("Failed to create note file");

    // Next, create scala code file
    let code_file = format!("{}/{}.scala", &note_dir, args.title.to_case(Case::Pascal));
    std::fs::File::create(&code_file).expect("Failed to create code file");
}
