extern crate phf;
mod model;
use model::Record;
use std::error::Error;
use std::io;
use std::time::Instant;
mod util;
mod fields;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut results: Vec<Record> = Vec::new();
    let now = Instant::now();
    for result in rdr.deserialize() {
        let record: Record = result?;
        results.push(record);
    }
    let elapsed = now.elapsed();
   //println!("Parsed {} rows in {:.2?}: {}/row",results.len(), elapsed, elapsed.as_micros() as i128 / results.len() as i128);
   println!("{}",  serde_json::to_string(&results).unwrap());

    Ok(())
}
