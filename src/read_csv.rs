use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = "job_desc_list/1.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}