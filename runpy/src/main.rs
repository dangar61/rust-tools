use clap::Parser;
use std::process::Command;

/// Run Python Script with Args
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// PYTHON EXE
    #[clap(short, long)]
    exe: String,

    /// PYTHON SCRIPT
    #[clap(short, long)]
    script: String,

    /// PYTHON ARGS
    #[clap(short, long)]
    arguments: String,
}

fn main() {
    let args = Args::parse();

    let script_path = args.script.clone();

    let output = Command::new(args.exe)
        .arg(args.script)
        .arg(args.arguments)
        .output()
        .expect("Failed to execute command");

    let human_readable = String::from_utf8(output.stdout).unwrap();
    println!("{}", human_readable);
    
    assert_eq!(format!("Number of arguments: 2 arguments.\nArgument List: ['{}', 'Test']\nHello World\n", script_path), human_readable);
}
