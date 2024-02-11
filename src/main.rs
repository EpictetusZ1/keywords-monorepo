use keywords::kw_list::KWList;
use keywords::read_csv::run;

// TODO: Add 3 Sample job descriptions in.
fn main() {
    // read_input("original/Jack_Heaton_CV.html");

    // let job_description = "Looking for a fullstack developer with knowledge in JavaScript, especially React. Skills in front-end and back-end technologies are a must.";
    let test = run();

    for job_description in test.unwrap() {
        let mut keywords = KWList::new();
        keywords.find_and_count_keywords(&job_description);
        // println!("Found keywords: {:?}", keywords);
        // println!(" ");
        keywords.prepare_keywords_for_replacement();
        // println!("Sorted keywords: {:?}", sorted_keywords);
        keywords.print_keywords();
    }
    // keywords.find_and_count_keywords(job_description);

}