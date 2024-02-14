use std::fs::{self, File};
use std::io::{Write, Result};

// TODO: Convert this to a struct and implement methods for it

struct ModifiedHtml {
    html_content: String,
    replacements: Vec<(String, usize)>,
}

impl ModifiedHtml {
    fn new(html_content: String, replacements: Vec<(String, usize)>) -> Self {
        Self {
            html_content,
            replacements,
        }
    }

    fn replace_keyword_placeholder(&self) -> String {
        let mut modified_html = self.html_content.to_owned();

        for (keyword, count) in &self.replacements {
            let placeholder = "{KEYWORD}"; // Placeholder text to replace
            for _ in 0..*count {
                modified_html = modified_html.replacen(placeholder, keyword, 1);
            }
        }

        modified_html
    }

    fn modify_html_file(&self, file_path: &str) -> Result<()> {
        // Read the HTML file
        let html_content = fs::read_to_string(file_path)?;

        // Replace placeholders with actual keywords
        let modified_html = self.replace_keyword_placeholder();

        // Write the modified HTML content back to the file or a new file
        let mut file = File::create(file_path)?;
        file.write_all(modified_html.as_bytes())?;

        Ok(())
    }

}

// TODO: Put these in the impl.


