use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::io::Read;

pub fn split_text(file_path: &str, document_name: &str, chunk_size: usize) -> std::io::Result<()> {
    // Open the file for reading
    let mut file = File::open(file_path).expect("Couldn't open file.");

    // Ensure the output directory exists
    if let Err(e) = fs::create_dir_all(document_name) {
        eprintln!("Error occurred while creating a directory: {e}");
        return Err(e);
    }

    // Read the entire file content
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Calculate the total size and iterations
    let total_size = content.as_bytes().len();
    if total_size == 0 {
        eprintln!("The file is empty; no data to split.");
        return Ok(());
    }

    if chunk_size > total_size {
        println!("Chunk size is larger than the file length. No splitting needed.");
        return Ok(());
    }

    let iterations = (total_size + chunk_size - 1) / chunk_size; // Calculate the number of chunks needed

    // Split the content into chunks and write each chunk to a file
    for chunk_index in 0..iterations {
        let start = chunk_index * chunk_size;
        let end = usize::min(start + chunk_size, total_size); // Ensure end does not exceed the total size

        let chunk_data = &content[start..end];
        let chunk_name = format!("chunk_{chunk_index}.txt");
        let file_path = Path::new(document_name).join(chunk_name);

        File::create(file_path)
            .expect("Couldn't create file for a chunk.")
            .write_all(chunk_data.as_bytes())
            .expect("Error while writing data to chunk file.");
    }

    println!("Splitting completed. Created {} chunks.", iterations);

    Ok(())
}


pub fn split_text_half(file_path: &str, document_name: &str) -> std::io::Result<()> {
    // Open the file for reading
    let mut file = File::open(file_path).expect("Couldn't read file.");

    // Ensure the output directory exists
    if let Err(e) = fs::create_dir_all(document_name) {
        eprintln!("Error occurred while creating a directory: {e}");
        return Err(e);
    }

    // Read the entire file content
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Calculate the midpoint of the content
    let total_length = content.len();
    if total_length == 0 {
        eprintln!("The file is empty; no data to split.");
        return Ok(());
    }

    let midpoint = total_length / 2;

    // Find the nearest line break to split at
    let split_index = content[..midpoint]
        .rfind('\n')
        .unwrap_or(midpoint); // If no line break is found, split at the midpoint

    // Split the content into two halves
    let (first_half, second_half) = content.split_at(split_index);

    // Write the first half to a file
    let first_half_path = Path::new(document_name).join("first_half.txt");
    let mut first_half_file = File::create(&first_half_path).expect("Couldn't create file for the first half.");
    first_half_file.write_all(first_half.as_bytes())?;

    // Write the second half to a file
    let second_half_path = Path::new(document_name).join("second_half.txt");
    let mut second_half_file = File::create(&second_half_path).expect("Couldn't create file for the second half.");
    second_half_file.write_all(second_half.as_bytes())?;

    println!(
        "Splitting completed. Created 'first_half.txt' and 'second_half.txt' in the '{}' directory.",
        document_name
    );

    Ok(())
}