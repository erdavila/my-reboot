use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::{fs, io, iter};

use crate::host_os::state_path;

const GRUBENV_CONTENT_LENGTH: usize = 1024;
const GRUBENV_HEADER_LINE: &str = "# GRUB Environment Block\n";

pub struct Grubenv {
    content: BTreeMap<String, String>,
}

impl Grubenv {
    pub fn load() -> io::Result<Grubenv> {
        let file_content = fs::read_to_string(Self::path())?;
        Ok(Self::from_file_content(&file_content))
    }

    fn from_file_content(file_content: &str) -> Grubenv {
        let content = file_content
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| {
                let (key, value) = line.split_once('=').unwrap();
                (key.to_string(), value.to_string())
            })
            .collect();

        Grubenv { content }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.content.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.content.insert(key.to_string(), value.to_string());
    }

    pub fn unset(&mut self, key: &str) {
        self.content.remove(key);
    }

    pub fn save(&self) -> io::Result<()> {
        let file_content = self.to_file_content();
        fs::write(Self::path(), file_content)
    }

    fn to_file_content(&self) -> String {
        let mut content = String::with_capacity(GRUBENV_CONTENT_LENGTH);
        content.push_str(GRUBENV_HEADER_LINE);

        for (key, value) in &self.content {
            let _ = writeln!(content, "{key}={value}");
        }

        let Some(padding_len) = GRUBENV_CONTENT_LENGTH.checked_sub(content.len()) else {
            panic!("Grubenv content is too large!");
        };

        let padding = iter::repeat_n('#', padding_len);
        content.extend(padding);
        assert_eq!(content.len(), GRUBENV_CONTENT_LENGTH);

        content
    }

    fn path() -> PathBuf {
        state_path("grubenv")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file_content() {
        let file_content = "abc=xyz\n#ignored line\njjj=123";

        let grubenv = Grubenv::from_file_content(file_content);

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
    fn unset() {
        let mut grubenv = create_grubenv();

        grubenv.unset("abc");

        assert_eq!(grubenv.content.len(), 1);
        assert_eq!(grubenv.content["jjj"], "123");
    }

    #[test]
    fn to_file_content() {
        const EXPECTED_LINE_1: &str = "abc=xyz\n";
        const EXPECTED_LINE_2: &str = "jjj=123\n";
        // Entries are sorted because we are using a BTreeMap.
        let expected_lines = format!("{EXPECTED_LINE_1}{EXPECTED_LINE_2}");

        let grubenv = create_grubenv();

        let file_content = grubenv.to_file_content();

        assert_eq!(file_content.len(), GRUBENV_CONTENT_LENGTH);
        assert!(file_content.starts_with(GRUBENV_HEADER_LINE));

        let remaining = &file_content[GRUBENV_HEADER_LINE.len()..];
        assert!(remaining.starts_with(&expected_lines));

        let remaining = &remaining[expected_lines.len()..];
        assert!(!remaining.contains(|c| c != '#'));
    }

    fn create_grubenv() -> Grubenv {
        let content = BTreeMap::from_iter([
            ("abc".to_string(), "xyz".to_string()),
            ("jjj".to_string(), "123".to_string()),
        ]);

        Grubenv { content }
    }
}
