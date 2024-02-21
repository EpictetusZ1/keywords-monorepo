use crate::kw_list::KWList;
use crate::read_csv::run;
use crate::replace::Replace;

pub mod read_input;
pub mod kw_list;
pub mod read_csv;
pub mod replace;
pub mod database;
pub mod server;

pub fn start_server() {
    let mut my_conn = database::connect_local().expect("Failed to connect to the database");
    database::create_table(&mut my_conn);
    database::seed_db(&mut my_conn);
    database::read_db(&mut my_conn);
}

pub fn read_file() -> (String, String) {
    let input = read_input::read_input("input/1.txt");
    let test = run();
    let data = test.unwrap();
    let one = data[0].clone();
    (input, one)
}

fn other_stuff(input: String, one: String) {
    let mut keywords = KWList::new();
    keywords.find_and_count_keywords(&one);
    keywords.prepare_keywords_for_replacement();
// keywords.print_keywords();

    let resume = Replace::new(input, keywords);
}
