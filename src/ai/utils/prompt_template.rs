use std::path::Path;

pub struct PromptTemplate {
    text_sections: Vec<String>,
    position_argument: Vec<usize>,
}

impl PromptTemplate {
    pub fn load_file(file: &Path) -> Self {
        let contents = std::fs::read_to_string(file).expect("Failed to read file");
        let mut contents = contents.chars();

        let mut text_sections = Vec::new();
        let mut position_argument = Vec::new();

        let mut current_section = String::new();
        let mut current_argument = String::new();

        let mut in_section = true;

        loop {
            let next_char = contents.next();
            match next_char {
                Some('{') => {
                    if in_section {
                        text_sections.push(current_section);
                        current_section = String::new();
                        in_section = false;
                    } else {
                        current_argument = String::new();
                        in_section = true;
                    }
                }
                Some('}') => {
                    position_argument.push(current_argument.parse().unwrap());
                    current_argument = String::new();
                    in_section = true;
                }
                Some(c) => {
                    if in_section {
                        current_section.push(c);
                    } else {
                        current_argument.push(c);
                    }
                }
                None => {
                    text_sections.push(current_section);
                    break;
                }
            }
        }

        PromptTemplate {
            text_sections,
            position_argument,
        }
    }

    pub fn format(&self, args: Vec<String>) -> String {
        let mut formatted = String::new();
        for i in 0..self.text_sections.len() {
            formatted.push_str(&self.text_sections[i]);
            formatted.push_str(&args[self.position_argument[i]])
        }
        formatted
    }
}