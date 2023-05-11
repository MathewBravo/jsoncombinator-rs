use crate::combine_match::combine_matches;
use crate::find_matches::find_matches;

pub fn command_parser(args: Vec<String>) {
    if let Some(index) = args.iter().position(|arg| arg == "-fm") {
        if index + 2 < args.len() {
            find_matches(args[index + 1].clone(), args[index + 2].clone());
        } else {
            println!("Error: Expected two values after -fm flag");
        }
    }

    if let Some(index) = args.iter().position(|arg| arg == "-cm") {
        if index + 3 < args.len() {
            combine_matches(
                args[index + 1].clone(),
                args[index + 2].clone(),
                args[index + 3].clone(),
            );
        } else {
            println!("Error: Expected two files and key after -cm flag");
        }
    }
    if let Some(index) = args.iter().position(|arg| arg == "-h" || arg == "-help") {
        println!("Usage:");
        println!("\t-fm [file1.json] [file2.json] - Find matches between two JSON files.");
        println!("\t-cm [fileA.json] [fileB.json] [key] - Combine matches from two JSON files based on a common key.");
    }
}
