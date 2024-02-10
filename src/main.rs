use keywords::kw_list::KWList;

fn main() {
    // read_input("original/Jack_Heaton_CV.html");
    let mut keywords = KWList::new();
    let job_description = "Looking for a fullstack developer with knowledge in JavaScript, especially React. Skills in front-end and back-end technologies are a must.";

    let found_keywords = keywords.find_and_count_keywords(job_description);
    println!("Found keywords: {:?}", keywords);

    let sorted_keywords = keywords.prepare_keywords_for_replacement();
    println!("Sorted keywords: {:?}", sorted_keywords);
}
