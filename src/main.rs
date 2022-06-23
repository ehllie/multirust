use enum_iterator::{all, Sequence};
use multirust::{compress, grep_tool};
use std::{env, fmt::Debug, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide the name of a tool you want to run!");
        process::exit(1)
    }
    let tool = parse_tool(&args[1]).unwrap_or_else(|_| {
        eprintln!("Unknown tool: {}", &args[1]);
        process::exit(1)
    });
    match tool {
        Tool::Grep => grep(&args),
        Tool::Compress => compress(&args),
    }
}

#[derive(Debug, PartialEq, Sequence)]
enum Tool {
    Grep,
    Compress,
}

fn parse_tool(arg: &str) -> Result<Tool, ()> {
    for tool in all::<Tool>() {
        if arg == format!("{:?}", tool).to_lowercase() {
            return Ok(tool);
        }
    }
    return Err(());
}

fn compress(args: &[String]) {
    let config = compress::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing config: {}", err);
        process::exit(1)
    });

    if let Err(e) = compress::compress(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

fn grep(args: &[String]) {
    let config = grep_tool::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing config: {}", err);
        process::exit(1)
    });

    if let Err(e) = grep_tool::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
