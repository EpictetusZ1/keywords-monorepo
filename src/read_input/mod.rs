use scraper::Html;
use std::fs;


pub fn read_input(path: &str) {
    let contents = fs::read_to_string(path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error reading file");
            return;
        }
    };

    let document = Html::parse_document(&contents);
    let body = document.select(&scraper::Selector::parse("body").unwrap()).next().unwrap();

    println!("Contents: {:?}", body.inner_html());
}