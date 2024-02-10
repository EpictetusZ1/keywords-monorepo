use scraper::Html;
use std::env;
use std::fs;


pub fn read_input(path: &str) {
    let contents = fs::read_to_string(path)?;

    let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

    let document = Html::parse_document(html);
}