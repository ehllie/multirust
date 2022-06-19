use std::env;

fn grep(query: &String, filename: &String) {
    println!("Searching for {}", query);
    println!("In file {}", filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let tool = &args[1];
    match tool.as_str() {
        "grep" => grep(&args[2], &args[3]),
        _ => println!("Unknown tool: {}", tool),
    }
}
