use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Value;


fn json_setup<'a> (file_path: &str, document_name: &str) -> serde_json::StreamDeserializer<'a, serde_json::de::IoRead<BufReader<File>>, Value> {

    if let Err(e) = std::fs::create_dir_all(&document_name) {
        eprintln!("Couldn't create folder for the split dataset: {e}");
    }

    let file = File::open(file_path).expect("Failed to open the JSON file");
    let reader = BufReader::new(file);

    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<Value>();

    stream
}

pub fn split_json(file_path: &str, document_name: &str, chunk_size: usize) -> std::io::Result<()> {
    
    let stream = json_setup(file_path, document_name);

    let mut chunk_index = 0;
    let mut buffer: Vec<Value> = Vec::new();
    let mut current_chunk_size = 0;

    println!("{chunk_size}");
    // Iterate over the JSON stream and split into chunks
    for (index, value) in stream.enumerate() {
        match value {
            Ok(json) => {

                let serialized = serde_json::to_vec(&json).expect("Failed to serialize JSON object");
                let object_size = serialized.len();
                current_chunk_size += object_size;

                buffer.push(json);

                // Once the buffer reaches the chunk size, write it to a file and reset the buffer
                if current_chunk_size >= chunk_size {
                    let chunk_path = format!("chunk_{}.json", chunk_index);
                    let path = Path::new(document_name).join(chunk_path);

                    let mut chunk_file = File::create(path).expect("Couldn't create file for chunk.");

                    if let Err(e) = serde_json::to_writer_pretty(&mut chunk_file, &buffer) {
                        eprintln!("Error occurred while saving chunk {}: {}", chunk_index, e);
                    }

                    // Reset buffer for the next chunk
                    buffer.clear();
                    chunk_index += 1;
                    current_chunk_size = 0;
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON object {}: {}", index + 1, e);
            }
        }
    }

    // Write any remaining data in the buffer to a new chunk (if the last chunk has fewer than chunk_size elements)
    if !buffer.is_empty() {
        let chunk_path = format!("chunk_{}.json", chunk_index);
        let path = Path::new(document_name).join(chunk_path);

        let mut chunk_file = File::create(path).expect("Couldn't create file for last chunk.");

        if let Err(e) = serde_json::to_writer_pretty(&mut chunk_file, &buffer) {
            eprintln!("Error occurred while saving last chunk {}: {}", chunk_index, e);
        }
    }

    println!("Splitting completed. Created {} chunks.", chunk_index + 1);
    Ok(())
}

pub fn split_json_half(file_path: &str, document_name: &str) -> std::io::Result<()> {
    
    let stream = json_setup(file_path, document_name);

    // Prepare output files
    let mut chunk_0 = File::create(format!("{}/chunk_0.json", document_name)).expect("Couldn't create file for the first chunk.");
    let mut chunk_1 = File::create(format!("{}/chunk_1.json", document_name)).expect("Couldn't create file for the second chunk.");

    // Divide the stream into two halves
    let mut buffer_0 = Vec::new();
    let mut buffer_1 = Vec::new();
    let mut total_objects = 0;

    for value in stream {
        match value {
            Ok(json) => {
                if total_objects % 2 == 0 {
                    buffer_0.push(json);
                } else {
                    buffer_1.push(json);
                }
                total_objects += 1;
            }
            Err(e) => {
                eprintln!("Error parsing JSON object: {e}");
            }
        }
    }

    // Write first half
    if let Err(e) = serde_json::to_writer_pretty(&mut chunk_0, &buffer_0) {
        eprintln!("Error occurred while saving the first half to file: {e}");
    }

    // Write second half
    if let Err(e) = serde_json::to_writer_pretty(&mut chunk_1, &buffer_1) {
        eprintln!("Error occurred while saving the second half to file: {e}");
    }

    println!("Successfully split the dataset into two halves.");
    Ok(())
}