use std::collections::HashSet;
use std::collections::HashMap;

// TODO: Make a list of 'hard' and 'soft' keywords, so I can match technical keywords, as well as behavioural ones

// TODO: Make a 'replacer' that keeps track of the freq. of keywords in the job desc. as well as their output usage

/// Represents a collection of keywords and their variations relevant to job descriptions,
/// along with their frequency of occurrence.
#[derive(Debug)]
pub struct KWList {
    // Stores keywords and their variations in a HashSet for quick lookup
    keywords: HashMap<String, usize>,
    total_replacements: usize, // Placeholder for total keyword replacements in the resume
}



impl KWList {
    /// Creates a new instance of `KWList` with predefined keywords set to 0 occurrences.
    pub fn new() -> Self {
        // Predefined keywords and their variations
        let keyword_variations = vec![
            "typescript", "javascript", "js", "react", "node.js", "node", "html", "css",
            "front-end", "frontend", "front end", "back-end", "backend", "back end",
            "fullstack", "full stack", "web development", "web dev",
            // Add more keywords and variations as necessary
        ];

        // Initialize keywords HashMap with a count of 0 for each keyword
        let keywords = keyword_variations.into_iter()
            .map(|keyword| (Self::normalize(&keyword), 0))
            .collect();

        Self { keywords, total_replacements: 10 } // Example: set total replacements to 100
    }

    /// Normalizes a string by converting it to lowercase and replacing certain characters.
    pub fn normalize(input: &str) -> String {
        input.to_lowercase()
            .replace("_", " ")
            .replace(".", " ")
    }

    /// Finds and updates the frequency of keywords present in the given text.
    pub fn find_and_count_keywords(&mut self, text: &str) {
        text.split_whitespace()
            .for_each(|word| {
                let normalized_word = Self::normalize(word); // Normalize for case-insensitivity
                // Only update the count if the word is a predefined keyword
                if let Some(count) = self.keywords.get_mut(&normalized_word) {
                    *count += 1;
                }
            });
    }

    /// Prepares a list of keywords for replacement in the resume, based on their frequencies.
    /// This aims to distribute the usage of keywords based on their frequency in the job description.
    pub fn prepare_keywords_for_replacement(&self) -> Vec<(String, usize)> {
        let mut keywords_for_replacement: Vec<(String, usize)> = self.keywords.iter()
            .filter(|(_keyword, count)| **count > 0)
            .map(|(keyword, &count)| (keyword.clone(), count))
            .collect();

        keywords_for_replacement.sort_by(|a, b| b.1.cmp(&a.1));

        let mut distributed_keywords = Vec::new();
        let mut total_distributed = 0;

        while distributed_keywords.len() < self.total_replacements && !keywords_for_replacement.is_empty() {
            let len = keywords_for_replacement.len(); // Get the length once, outside the mutable access
            let idx = total_distributed % len; // Calculate index based on how many keywords have been distributed

            // Directly work with the keyword and its count without additional mutable borrow
            if let Some((keyword, count)) = keywords_for_replacement.get_mut(idx) {
                distributed_keywords.push((keyword.clone(), *count)); // Clone and push the keyword and its count
                total_distributed += 1;

                // If a keyword's count reaches zero, remove it from consideration
                if *count == 1 {
                    keywords_for_replacement.remove(idx);
                } else {
                    *count -= 1;
                }
            }
        }

        distributed_keywords
    }

    pub fn print_keywords(&self) {
        for (keyword, &count) in &self.keywords {
            if count > 0 {
                println!("{}: {}", keyword, count);
            }
        }
    }
}