use keywords::kw_list::KWList;

// TODO: Add 3 Sample job descriptions in.
fn main() {
    // read_input("original/Jack_Heaton_CV.html");
    let mut keywords = KWList::new();
    let job_description = "Looking for a fullstack developer with knowledge in JavaScript, especially React. Skills in front-end and back-end technologies are a must.";

    keywords.find_and_count_keywords(job_description);
    println!("Found keywords: {:?}", keywords);

    keywords.prepare_keywords_for_replacement();
    // println!("Sorted keywords: {:?}", sorted_keywords);
    keywords.print_keywords();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(KWList::normalize("typescript"), "typescript");
        assert_eq!(KWList::normalize("TypeScript"), "typescript");
        assert_eq!(KWList::normalize("Type_Script"), "type script");
        assert_eq!(KWList::normalize("Type.Script"), "type script");
    }
}