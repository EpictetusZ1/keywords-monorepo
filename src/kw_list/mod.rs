mod input_list;

use std::collections::{HashMap, HashSet};

// TODO: Make a list of 'hard' and 'soft' keywords, so I can match technical keywords, as well as behavioural ones
// TODO: Make a 'replacer' that keeps track of the freq. of keywords in the job desc. as well as their output usage

#[derive(Debug, Clone)]
struct KWInstance {
    canonical: String,            // The canonical form of the keyword
    variations: HashSet<String>,  // Variations of the keyword found in the text
    output_version: Option<String>, // The chosen output version, if any
}

impl KWInstance {
    pub fn new(canonical: &str) -> Self {
        Self {
            canonical: canonical.to_string(),
            variations: HashSet::new(),
            output_version: None,
        }
    }

    pub fn add_variation(&mut self, variation: &str) {
        self.variations.insert(variation.to_string());
        // Optionally, set this variation as the output_version if it's the first encountered or based on other logic
        if self.output_version.is_none() {
            self.output_version = Some(variation.to_string());
        }
    }

}

/// Represents a collection of keywords and their variations relevant to job descriptions,
/// along with their frequency of occurrence.
#[derive(Debug)]
pub struct KWList {
    keywords: HashMap<String, KWInstance>, // Maps canonical forms to KWInstance
    total_replacements: usize,
}

impl KWList {
    /// Creates a new instance of `KWList` with predefined keywords set to 0 occurrences.
    pub fn new() -> Self {
        let mut keywords = HashMap::new();
        let keyword_variations = input_list::keyword_input();

        for (canonical, variations) in keyword_variations.into_iter() {
            for variation in variations {
                let normalized_variation = Self::normalize(&variation);

                // Check if the canonical keyword already exists
                if !keywords.contains_key(&normalized_variation) {
                    keywords.insert(normalized_variation, KWInstance::new(&canonical));
                } else {
                    panic!("Duplicate keyword: {}", normalized_variation);
                    // println!("Duplicate keyword: {}", normalized_variation);
                }
            }
        }

        Self {
            keywords,
            total_replacements: 10,
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
        text.split_whitespace().for_each(|word| {
            let normalized_word = Self::normalize(word);
            // Check if the normalized word is a variation of a canonical keyword
            if let Some(kw_instance) = self.keywords.get_mut(&normalized_word) {
                kw_instance.add_variation(word); // Add the original word as a found variation
            }
        });
    }

    /// Prepares a list of keywords for replacement in the resume, based on their frequencies.
    /// This aims to distribute the usage of keywords based on their frequency in the job description.
    pub fn prepare_keywords_for_replacement(&self) -> Vec<(String, usize)> {
        self.keywords.values()
            .filter(|kw_instance| !kw_instance.variations.is_empty())
            .map(|kw_instance| {
                // Assuming `output_version` is always set when variations are added
                (kw_instance.output_version.clone().unwrap(), kw_instance.variations.len())
            })
            .collect()
    }

    pub fn print_keywords(&self) {
        for (canonical, kw_instance) in &self.keywords {
            if !kw_instance.variations.is_empty() {
                println!("{} ({} occurrences):", canonical, kw_instance.variations.len());
                for variation in &kw_instance.variations {
                    println!("- {}", variation);
                }
                if let Some(output_version) = &kw_instance.output_version {
                    println!("Preferred output version: {}", output_version);
                }
            }
        }
    }
}
