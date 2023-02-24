use csv::Error;
use std::time::Instant;

use crate::consolidated_record::ConsolidatedRecord;

mod consolidated_record;
mod csv_data;

fn run(name: String, gender: String) {
    let file_path = format!("./data/{}_baby_names_1996_2021.csv", gender);
    let rdr = csv::ReaderBuilder::new().from_path(file_path);

    match rdr {
        Err(e) => println!("{}", e),
        Ok(mut file) => {
            let start_query_time = Instant::now();

            for record in file.records() {
                let row = record.unwrap();
                if row.iter().any(|field| field == name) {
                    let baby_name_record: Result<csv_data::CsvData, Error> = row.deserialize(None);

                    match baby_name_record {
                        Err(e) => println!("{:?}", e),
                        Ok(csv_data) => {
                            let consolidated_record: ConsolidatedRecord =
                                consolidated_record::calculate(&csv_data);
                            println!("{:?}", consolidated_record);
                        }
                    }
                }
            }

            println!("Found baby name within {:?}", start_query_time.elapsed());
        }
    }
}

fn main() {
    run(String::from("Auri"), String::from("girls"));
}
