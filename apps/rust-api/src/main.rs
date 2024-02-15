use keywords::kw_list::KWList;
use keywords::read_csv::run;
use keywords::read_input::read_input;
use keywords::replace::Replace;
extern crate regex;
use regex::Regex;
use std::fs;

// TODO: Add 3 Sample job descriptions in.
fn main() {
    println!("RUST FILE RUNNING");
    // let input =  read_input("original/Jack_Heaton_CV.html");
    // let test = run();
    // let data = test.unwrap();
    // let one = &data[0];
    //
    // let mut keywords = KWList::new();
    // keywords.find_and_count_keywords(&one);
    // keywords.prepare_keywords_for_replacement();
    // keywords.print_keywords();
    //
    // let cleaned = clean_text(&input);
    // let resume = Replace::new(cleaned, keywords);
    // // remove html tags
    // println!("{}", resume.replace_keywords());
}

fn clean_text(input: &str) -> String {
    // Remove HTML tags
    let re_tags = Regex::new(r"(?s)<[^>]*>").unwrap();
    let without_tags = re_tags.replace_all(input, "").to_string();

    // Replace HTML entities like &nbsp; with a space
    let re_nbsp = Regex::new(r"&nbsp;").unwrap();
    let without_nbsp = re_nbsp.replace_all(&without_tags, " ").to_string();

    // Collapse multiple whitespaces into one, and trim leading/trailing spaces
    let re_space = Regex::new(r"\s+").unwrap();
    let collapsed_spaces = re_space.replace_all(&without_nbsp, " ").to_string();

    // Trim the text to remove leading and trailing whitespaces
    collapsed_spaces.trim().to_string()
}