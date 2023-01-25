use std::collections::HashMap;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

use crate::file_content_as_hash_map::file_content_to_hash_map;
use crate::file_content_as_hash_map::hash_map_to_file_content;
use crate::host_os::STATE_DIR_PATH;

pub struct Properties {
    content: HashMap<String, String>,
    path: PathBuf,
}

impl Properties {
    pub fn load(filename: &str, must_exist: bool) -> io::Result<Properties> {
        let path = Self::path(filename);

        match fs::read_to_string(&path) {
            Ok(file_content) => {
                let content = Self::hash_map_from_file_content(&file_content);
                Ok(Properties { content, path })
            }
            Err(e) if !must_exist && e.kind() == ErrorKind::NotFound => {
                eprintln!(
                    "Arquivo {:?} não encontrado. Prosseguindo com conteúdo vazio.",
                    path
                );
                Ok(Properties {
                    content: HashMap::new(),
                    path,
                })
            }
            Err(e) => Err(e),
        }
    }

    fn hash_map_from_file_content(file_content: &str) -> HashMap<String, String> {
        let mut hash_map = file_content_to_hash_map(file_content);
        Self::unescape_inline(&mut hash_map);
        hash_map
    }

    fn unescape_inline(hash_map: &mut HashMap<String, String>) {
        for value in hash_map.values_mut() {
            *value = value.replace(r"\\", r"\");
        }
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
        fs::write(&self.path, file_content)
    }

    fn to_file_content(&self) -> String {
        let hash_map = Self::escape(&self.content);
        hash_map_to_file_content(&hash_map)
    }

    fn escape(hash_map: &HashMap<String, String>) -> HashMap<String, String> {
        let entries = hash_map
            .iter()
            .map(|(key, value)| (key.clone(), value.replace('\\', r"\\")));
        HashMap::from_iter(entries)
    }

    fn path(filename: &str) -> PathBuf {
        [STATE_DIR_PATH, filename].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_map_from_file_content() {
        let file_content = "abc=xyz\n#ignored line\njjj=12\\\\3";

        let hash_map = Properties::hash_map_from_file_content(&file_content);

        assert_eq!(hash_map.len(), 2);
        assert_eq!(hash_map["abc"], "xyz");
        assert_eq!(hash_map["jjj"], r"12\3");
    }

    #[test]
    fn get() {
        let properties = create_properties();

        assert_eq!(properties.get("abc"), Some(&"xyz".to_string()));
        assert_eq!(properties.get("jjj"), Some(&r"12\3".to_string()));
    }

    #[test]
    fn set() {
        let mut properties = create_properties();

        properties.set("@@@", "###");
        properties.set("jjj", "999");

        assert_eq!(properties.content.len(), 3);
        assert_eq!(properties.content["abc"], "xyz");
        assert_eq!(properties.content["jjj"], "999");
        assert_eq!(properties.content["@@@"], "###");
    }

    #[test]
    fn unset() {
        let mut properties = create_properties();

        properties.unset("jjj");

        assert_eq!(properties.content.len(), 1);
        assert_eq!(properties.content["abc"], "xyz");
    }

    #[test]
    fn to_file_content() {
        const EXPECTED_LINE_1: &str = "abc=xyz\n";
        const EXPECTED_LINE_2: &str = "jjj=12\\\\3\n";
        let expected_prefix_1 = format!("{EXPECTED_LINE_1}{EXPECTED_LINE_2}");
        let expected_prefix_2 = format!("{EXPECTED_LINE_2}{EXPECTED_LINE_1}");
        assert_eq!(expected_prefix_1.len(), expected_prefix_2.len());

        let properties = create_properties();

        let file_content = properties.to_file_content();
        assert!(file_content == expected_prefix_1 || file_content == expected_prefix_2);
    }

    fn create_properties() -> Properties {
        let content = HashMap::from_iter([
            ("abc".to_string(), "xyz".to_string()),
            ("jjj".to_string(), r"12\3".to_string()),
        ]);

        Properties {
            content,
            path: PathBuf::new(),
        }
    }
}
