use keywords::kw_list::KWList;
use keywords::read_csv::run;
use keywords::read_input::read_input;
use keywords::replace::Replace;
use keywords::database;

fn main() {
    let mut my_conn = database::connect_local().expect("Failed to connect to the database");
    database::create_table(&mut my_conn);
    database::seed_db(&mut my_conn);
    database::read_db(&mut my_conn);

    let input =  read_input("input/1.txt");
    let test = run();
    let data = test.unwrap();
    let one = &data[0];

    let mut keywords = KWList::new();
    keywords.find_and_count_keywords(&one);
    keywords.prepare_keywords_for_replacement();
    keywords.print_keywords();

    let resume = Replace::new(input, keywords);
    println!("{}", resume.replace_keywords());
}
