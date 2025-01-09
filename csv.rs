use csv::{self, Result};
use csv::Writer;
use std::fs::File;
use std::path::Path;
use std::fs;


pub fn read_csv(file_path: &str, chunk_size: usize, document_name: &str) -> Result<()>{

 //   let cwd = std::env::current_dir().unwrap().expect("Couldn't get current working directory.");

    fs::create_dir_all(document_name).expect("Couldn't create a directory.");
    
    let file = File::open(file_path).expect("Problem with reading the file.");

    let mut rdr = csv::Reader::from_reader(file);

    // Initialize chunk tracking
    let mut chunk_index = 0;
    let mut current_size = 0usize;
    let mut chunk_writer: Option<Writer<File>>;

    // Create headers for the first chunk
    let headers = rdr.headers()?.clone(); // the headers of the file
    let chunk_path = format!("chunk_{}.csv", chunk_index); // name of the new chunk
    let chunk_path = Path::new("").join(document_name).join(&chunk_path); // the full path of the chunk
    println!("{:?}", chunk_path);
    chunk_writer = Some(Writer::from_path(&chunk_path)?);

    if let Some(ref mut writer) = chunk_writer {
        writer.write_record(&headers)?;
    }

    // Process records
    for result in rdr.records() {
        let record = result?;
        let record_size = record.iter().map(|field| field.len()).sum::<usize>() + 1; // +1 for newline

        // Check if the current record exceeds the chunk size
        if current_size + record_size > chunk_size {
            // Finalize the current chunk
            if let Some(ref mut writer) = chunk_writer {
                writer.flush()?;
            }

            // Start a new chunk
            // redo previous steps
            chunk_index += 1;

            let chunk_path = format!("chunk_{}.csv", chunk_index);
            let chunk_path = Path::new("").join(document_name).join(&chunk_path);

            chunk_writer = Some(Writer::from_path(&chunk_path)?);

            if let Some(ref mut writer) = chunk_writer {
                writer.write_record(&headers)?;
            }

            current_size = 0;
        }

        // Write the record to the current chunk
        if let Some(ref mut writer) = chunk_writer {
            writer.write_record(&record)?;
        }
        current_size += record_size;
    }

    // Finalize the last chunk
    if let Some(ref mut writer) = chunk_writer {
        writer.flush()?;
    }

    println!("Splitting completed. Created {} chunks.", chunk_index + 1);
    Ok(())
}

pub fn split_csv_to_half(file_path: &str) -> Result<()> {

    // function splits a csv file into two halves if the user specified.
    // similar to the read_csv function

    fs::create_dir_all("splitted_dataset").expect("Couldn't create a directory.");

    let file = File::open(file_path).expect("Couldn't open file.");

    let mut rdr = csv::Reader::from_reader(file);

    // the top headers of the csv file
    let headers = rdr.headers()?.clone();

    // all of the row data
    let data: Vec<_> = rdr.records().collect::<Result<_>>()?;

    let midpoint = data.len() / 2; // the point we split the data

    let first_half = &data[..midpoint];
    let second_half = &data[midpoint..];

    let path = Path::new("").join("splitted_dataset").join("fist_split.csv");

    let mut writer1 = Writer::from_path(path)?;

    // write the headers at the top first and then write each record in the first half
    writer1.write_record(&headers).expect("Couldn't write the headers of the CSV file.");
    
    for record in first_half {
        writer1.write_record(record).expect("Couldn't write data.");
    }

    writer1.flush().expect("Couldn't write data.");

    let path = Path::new("").join("splitted_dataset").join("second_split.csv");

    let mut writer2 = Writer::from_path(path).unwrap();

    // write the headers at the top first and then write each record in the second half
    writer2.write_record(&headers).expect("Couldn't write the headers of the CSV file.");
    for record in second_half {
        writer2.write_record(record).expect("Couldn't write data.");
    }

    writer2.flush().expect("Couldn't write data.");


    println!("Splitted the dataset successfully.");

    Ok(())
}
