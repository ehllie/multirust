use multirust::grep_tool;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tool = parse_tool(&args).unwrap_or_else(|err| {
        eprintln!("Error while selecting tool: {}", err);
        process::exit(1)
    });

    match tool {
        Tool::Grep => grep(&args),
    }
}

enum Tool {
    Grep,
}

fn parse_tool(args: &[String]) -> Result<Tool, &'static str> {
    if args.len() < 2 {
        return Err("A tool name needs to be provided!");
    }
    match args[1].as_str() {
        "grep" => Ok(Tool::Grep),
        _ => Err("Unknown tool"),
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
