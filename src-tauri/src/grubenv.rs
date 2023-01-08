use std::collections::HashMap;
use std::fs;
use std::io;
use std::iter;
use std::path::PathBuf;

use crate::host_os::STATE_DIR_PATH;

const GRUBENV_CONTENT_LENGTH: usize = 1024;

pub struct Grubenv {
    content: HashMap<String, String>,
}

impl Grubenv {
    pub fn load() -> io::Result<Grubenv> {
        let file_content = fs::read_to_string(Self::path())?;
        Ok(Self::from_file_content(&file_content))
    }

    fn from_file_content(file_content: &str) -> Grubenv {
        let entries = file_content
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| {
                let (key, value) = line.split_once('=').unwrap();
                (key.to_string(), value.to_string())
            });

        Grubenv {
            content: HashMap::from_iter(entries),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.content.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.content.insert(key.to_string(), value.to_string());
    }

    pub fn save(&self) -> io::Result<()> {
        let file_content = self.to_file_content();
        fs::write(Self::path(), file_content)
    }

    fn to_file_content(&self) -> String {
        let mut content: String = self
            .content
            .iter()
            .map(|(key, value)| format!("{key}={value}\n"))
            .collect();

        if content.len() > GRUBENV_CONTENT_LENGTH {
            panic!("Grubenv content is too large!");
        }

        if content.len() < GRUBENV_CONTENT_LENGTH {
            let padding = iter::repeat('#').take(GRUBENV_CONTENT_LENGTH - content.len());
            content.extend(padding);
            assert_eq!(content.len(), GRUBENV_CONTENT_LENGTH);
        }

        content
    }

    fn path() -> PathBuf {
        [STATE_DIR_PATH, "grubenv"].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file_content() {
        let file_content = "abc=xyz\n#ignored line\njjj=123";

        let grubenv = Grubenv::from_file_content(&file_content);

        assert_eq!(grubenv.content.len(), 2);
        assert_eq!(grubenv.content["abc"], "xyz");
        assert_eq!(grubenv.content["jjj"], "123");
    }

    #[test]
    fn get() {
        let grubenv = create_grubenv();

        assert_eq!(grubenv.get("abc"), Some(&"xyz".to_string()));
        assert_eq!(grubenv.get("jjj"), Some(&"123".to_string()));
    }

    #[test]
    fn set() {
        let mut grubenv = create_grubenv();

        grubenv.set("@@@", "###");
        grubenv.set("jjj", "999");

        assert_eq!(grubenv.content.len(), 3);
        assert_eq!(grubenv.content["abc"], "xyz");
        assert_eq!(grubenv.content["jjj"], "999");
        assert_eq!(grubenv.content["@@@"], "###");
    }

    #[test]
    fn to_file_content() {
        const EXPECTED_LINE_1: &str = "abc=xyz\n";
        const EXPECTED_LINE_2: &str = "jjj=123\n";
        let expected_prefix_1 = format!("{EXPECTED_LINE_1}{EXPECTED_LINE_2}");
        let expected_prefix_2 = format!("{EXPECTED_LINE_2}{EXPECTED_LINE_1}");
        assert_eq!(expected_prefix_1.len(), expected_prefix_2.len());

        let grubenv = create_grubenv();

        let file_content = grubenv.to_file_content();

        assert_eq!(file_content.len(), GRUBENV_CONTENT_LENGTH);
        assert!(
            file_content.starts_with(&expected_prefix_1)
                || file_content.starts_with(&expected_prefix_2)
        );

        let remaining = &file_content[expected_prefix_1.len()..];
        assert!(!remaining.contains(|c| c != '#'));
    }

    fn create_grubenv() -> Grubenv {
        let content = HashMap::from_iter([
            ("abc".to_string(), "xyz".to_string()),
            ("jjj".to_string(), "123".to_string()),
        ]);

        Grubenv { content }
    }
}
