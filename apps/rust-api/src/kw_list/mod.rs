pub mod input_list;

use std::collections::{HashMap, HashSet};

// TODO: Make a list of 'hard' and 'soft' keywords, so I can match technical keywords, as well as behavioural ones
// TODO: Make a 'replacer' that keeps track of the freq. of keywords in the job desc. as well as their output usage

#[derive(Debug, Clone)]
pub struct KWInstance {
    pub canonical: String,            // The canonical form of the keyword
    pub variations: HashSet<String>,  // Variations of the keyword found in the text
    pub output_version: Option<String>, // The chosen output version, if any
    pub position: Option<usize>, // The position of the keyword in the job description
}

impl KWInstance {
    pub fn new(canonical: &str) -> Self {
        Self {
            canonical: canonical.to_string(),
            variations: HashSet::new(),
            output_version: None,
            position: None,
        }
    }

    pub fn add_variation(&mut self, variation: &str, pos: usize) {
        self.variations.insert(variation.to_string());
        if self.position.is_none() {
            self.position = Some(pos);
        }
        if self.output_version.is_none() {
            self.output_version = Some(variation.to_string());
        }
    }
}

/// Represents a collection of keywords and their variations relevant to job descriptions,
/// along with their frequency of occurrence.
#[derive(Debug)]
pub struct KWList {
    pub keywords: HashMap<String, KWInstance>, // Maps canonical forms to KWInstance
}

impl KWList {
    /// Creates a new instance of `KWList` with predefined keywords set to 0 occurrences.
    pub fn new() -> Self {
        let mut keywords = HashMap::new();
        let keyword_variations = input_list::keyword_input(); // Assuming this provides canonical forms

        for (canonical, _variations) in keyword_variations.into_iter() {
            let normalized_canonical = Self::normalize(&canonical);
            keywords.insert(normalized_canonical, KWInstance::new(&canonical));
        }

        Self {
            keywords,
        }
    }

    /// Normalizes a string by converting it to lowercase and replacing certain characters.
    pub fn normalize(input: &str) -> String {
        input.to_lowercase()
            .replace("_", " ")
            .replace(".", " ")
    }

    /// Finds and updates the frequency of keywords present in the given text.
    pub fn find_and_count_keywords(&mut self, text: &str) {
        let words = text.split_whitespace().collect::<Vec<&str>>();
        let mut pos = 0;
        for (_i, word) in words.iter().enumerate() {
            let normalized_word = Self::normalize(word);
            if let Some(kw_instance) = self.keywords.get_mut(&normalized_word) {
                if !kw_instance.canonical.contains(word) {
                    pos += 1;
                    kw_instance.add_variation(word, pos);
                }

            }
        }
    }

    pub fn get_sorted_keywords(&mut self) -> Vec<(&String, usize, String, &Option<String>, Option<usize>)>{
        let mut keywords_info: Vec<(&String, usize, String, &Option<String>, Option<usize>)> = self.keywords.iter()
            .map(|(canonical, kw_instance)| {
                let variations_count = kw_instance.variations.len();
                let variations_str = kw_instance.variations.iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ");
                let output_version_str = &kw_instance.output_version;
                let position = kw_instance.position;
                (canonical, variations_count, variations_str, output_version_str, position)
            })
            .collect();

        keywords_info.retain(|(_canonical, count, _variations_str, _output_version, _pos)| *count > 0);
        // Sort by occurrences (descending) and then by position (ascending)
        keywords_info.sort_by(|a, b| {
            let occ_cmp = b.1.cmp(&a.1);
            if occ_cmp == std::cmp::Ordering::Equal {
                a.4.unwrap_or(usize::MAX).cmp(&b.4.unwrap_or(usize::MAX))
            } else {
                occ_cmp
            }
        });

        keywords_info
    }

    /// Prepares a list of keywords for replacement in the resume, based on their frequencies.
    /// This aims to distribute the usage of keywords based on their frequency in the job description.
    pub fn prepare_keywords_for_replacement(&self) -> Vec<(String, usize)> {
        self.keywords.values()
            .filter(|kw_instance| !kw_instance.variations.is_empty())
            .map(|kw_instance| {
                (kw_instance.output_version.clone().unwrap(), kw_instance.variations.len())
            })
            .collect()
    }

    pub fn print_keywords(&mut self) {
        let keywords_info = self.get_sorted_keywords();

        if !keywords_info.is_empty() {
            println!("\n\n{:<20} | {:<12} | {:<30} | {}", "Canonical", "Occurrences", "Variations / Output Version", "First Occurrence Position");
            println!("{:-<1$}", "", 90);

            for (canonical, count, variations_str, _output_version, position) in keywords_info {
                let position_str = position.map_or("N/A".to_string(), |p| p.to_string());
                println!("{:<20} | {:<12} | {:<30} | {}", canonical, count, variations_str, position_str);
            }
        } else {
            println!("No keywords found.");
        }
    }

}
