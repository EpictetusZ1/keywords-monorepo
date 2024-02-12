

// This will take in a struct of keywords (KWInstance), and replace the placeholder in the resume input with the keywords
// The keywords are already sorted via their occurrence and index of appearance in the job description

use crate::kw_list::KWList;

pub struct Replace {
    pub resume: String,
    pub keywords: KWList,
}

impl Replace {
    pub fn new(resume: String, keywords: KWList) -> Self {
        Self {
            resume,
            keywords,
        }
    }

    pub fn replace_keywords(&self) -> String {
        let mut output = self.resume.clone();
        for (canonical, instance) in self.keywords.keywords.iter() {
            if let Some(output_version) = &instance.output_version {
                output = output.replace("{{placeholder}}", output_version);
            }
        }
        output
    }
}