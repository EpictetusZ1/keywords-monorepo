use std::fs;

pub fn read_input(path: &str) -> String {
    /// Read in a .txt file
    let contents = fs::read_to_string(path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error reading file");
            return String::new()
        }
    };

    contents
}
