use std::{
    error::Error,
    fs::File,
};

pub fn run() -> Result<Vec<String>, Box<dyn Error>> {
    let mut output: Vec<String> = Vec::new();

    let file_path = "job_desc_list/1.csv";
    let file = File::open(file_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        output.push(record[1].to_string());
    }

    Ok(output)
}