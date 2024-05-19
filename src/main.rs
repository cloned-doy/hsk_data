// use std::collections::HashMap;
// use serde::{Serialize, Deserialize};
// use std::fs::File;
// use std::io::{BufReader, BufWriter};
// // use std::io::BufReader;
// use bincode::{serialize_into, deserialize_from};



// fn parse_csv(csv_data: BufReader<File>) -> HashMap<String, (u16, String)> {
//     let mut map = HashMap::new();
//     let mut line_number = 1; // Initialize line number counter
    
//     for line in csv_data.lines() {
//         let mut parts = line.split(',');
//         if let (Some(hanzi), Some(hsk_level)) = (parts.next(), parts.next()) {
//             map.insert(hanzi.to_string(), (line_number, hsk_level.to_string())); // Use line number as key
//             line_number += 1; // Increment line number
//         }
//     }
//     map
// }

// // Define your data structure
// #[derive(Serialize, Deserialize)]
// struct HskData {
//     data: HashMap<String, (u16, String)>,
// }

// // Serialize your data structure to bincode
// fn serialize_to_bincode(data: &HashMap<String, (u16, String)>, file_path: &str) -> std::io::Result<()> {
//     let hsk_data = HskData { data: data.clone() };
//     let file = File::create(file_path)?;
//     let writer = BufWriter::new(file);
//     serialize_into(writer, &hsk_data);
//     Ok(())
// }

// // Deserialize bincode data
// fn deserialize_from_bincode(file_path: &str) -> std::io::Result<HashMap<String, (u16, String)>> {
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(file);
//     let hsk_data: HskData = deserialize_from(reader);
//     Ok(hsk_data.data)
// }

// fn main() {
//     let filename_path = "src/data/hsk_simplified.csv";
//     let file = File::open(filename_path);
//     let csv_data = BufReader::new(file);
    
//     // Parse CSV data
//     let data = parse_csv(csv_data);

//     // Serialize data to bincode
//     serialize_to_bincode(&data, "hsk.data").expect("Failed to serialize data");

//     // Deserialize data from bincode
//     let loaded_data = deserialize_from_bincode("hsk.data").expect("Failed to deserialize data");

//     println!("{:?}", loaded_data);
// }


use std::path::Path;
use std::io::{Result, BufWriter};
use bincode::{serialize_into};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use serde::Serialize;

fn atomic_save_bin<T: Serialize>(path: &Path, object: &T) -> Result<()> {
    let af = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);

    af.write(|f| {
        serialize_into(BufWriter::new(f), object)
    })?;

    Ok(())
}

fn main() {
    // Convert the string path to a Path
    let file_path = Path::new("./src/data/hsk_simplified.csv");

    // Example usage:
    let data_to_serialize = vec![1, 2, 3, 4, 5]; // Example data to serialize
    atomic_save_bin(&file_path, &data_to_serialize).expect("Failed to save data.");
}
