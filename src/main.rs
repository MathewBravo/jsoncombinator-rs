mod combine_match;
mod find_matches;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    parser::command_parser(args);
}
