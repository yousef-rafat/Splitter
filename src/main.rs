use std::env;
mod csv;
mod json;
mod txt;
use std::fs;
// json file path C:\Users\yrafa\Downloads\test.txt
fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Welcome to Splitter! Splitter is a tool for splitting large datasets into chunks.\n");
        println!("To start splitting the dataset, specify the file using -file [FILE_DIR]\n");
        println!("To specify the chunk size, use -size [NUMBER_IN_MEGABYTES]\n");
        println!("To specify the name of splitted dataset folder, use -name [NAME]\n");
        return;
    }

    let file_path = file_argument_handler(&args);

    // check if file exists before we continue proceeding
    if !fs::exists(file_path).expect("Couldn't check if file exists or not.") {
        eprintln!("File path doesn't exist. Please input a correct file path.");
        return;
    }

    let parts: Vec<_> = file_path.split('.').collect(); // Create a longer-lived Vec
    let file_extension = parts.last().expect("Couldn't get file extension.");

    // Get document name and chunk size if they exist.
    let (document_name, chunk_size, half) = check_for_additional_arguments(&args);

    let chunk_size = megabytes_to_bytes(chunk_size);

    handle_different_file_extensions(file_extension, chunk_size, document_name.as_str(), file_path, half);
}

fn file_argument_handler(args: &Vec<String>) -> &str {
    // handle and get the file argument

    if let Some(index) = args.iter().position(|arg| arg == "-file") {
        if let Some(file_path) = args.get(index + 1) {
            return file_path.as_str();
        } else {
            eprintln!("Error: No file path provided after '-file'");
            return "None";
        }
    }
    else {
        return "None";
    }
}

fn megabytes_to_bytes(megabytes: usize) -> usize {
    // convert a decimal number to megabytes for later splitting
    megabytes * 1024 * 1024
}

fn handle_different_file_extensions(
    file_extension: &str,
    chunk_size: usize,
    document_name: &str,
    file_path: &str,
    half: bool,
) {
    match file_extension {

        "csv" => {
            if half {
                if let Err(e) = csv::split_csv_to_half(file_path, document_name) {
                    eprintln!("Error occurred splitting CSV file into half: {e}");
                }
            } else {
                if let Err(e) = csv::read_csv(file_path, chunk_size, document_name) {
                    eprintln!("Error occurred splitting CSV file: {e}");
                }
            }
        }

        "json" => {
            if half {
                if let Err(e) = json::split_json_half(file_path, document_name) {
                    eprintln!("Error occurred splitting JSON file into half: {e}");
                }
            } else {
                if let Err(e) = json::split_json(file_path, document_name, chunk_size) {
                    eprintln!("Error occurred splitting JSON file: {e}");
                }
            }
        }

        "txt" => {
            if half {
                if let Err(e) = txt::split_text_half(file_path, document_name) {
                    eprintln!("Error occurred splitting JSON file into half: {e}");
                }
            } else {
                if let Err(e) = txt::split_text(file_path, document_name, chunk_size) {
                    eprintln!("Error occurred splitting JSON file: {e}");
                }
            }
        }
        _ => {
            eprintln!("Unsupported file extension: {file_extension}");
        }
    }
}

fn check_for_additional_arguments(args: &Vec<String>) -> (String, usize, bool) {
    // Function to get any additional arguments if they were specified by the user

    let mut returned_values: (String, usize, bool) = ("splitted_dataset".to_string(), 100, false);

    if let Some(index) = args.iter().position(|arg| arg == "-name") {
        if let Some(document_name) = args.get(index + 1) {
            returned_values.0 = document_name.clone();
        } else {
            println!("No name specified. Defaulting to splitted_dataset.");
        }    
    }

    if args.iter().any(|arg| arg == "-half") {
        returned_values.2 = true; 
    }

    if let Some(index) = args.iter().position(|arg| arg == "-size") {
        if let Some(chunk_size) = args.get(index + 1) {
            returned_values.1 = chunk_size.parse().unwrap_or(100);
        } 
    } else {
        if returned_values.2 == false {
            println!("No size specified, splitting every 100 Megabytes.\n");
        }
    }

    return returned_values;
}