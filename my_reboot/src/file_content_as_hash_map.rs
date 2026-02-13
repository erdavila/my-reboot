use std::collections::HashMap;

pub(crate) fn file_content_to_hash_map(file_content: &str) -> HashMap<String, String> {
    file_content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let (key, value) = line.split_once('=').unwrap();
            (key.to_string(), value.to_string())
        })
        .collect()
}

pub(crate) fn hash_map_to_file_content(hash_map: &HashMap<String, String>) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    for (key, value) in hash_map {
        let _ = writeln!(output, "{key}={value}");
    }
    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn file_content_to_hash_map() {
        let file_content = "abc=xyz\n#ignored line\njjj=123";

        let hash_map = super::file_content_to_hash_map(file_content);

        assert_eq!(hash_map.len(), 2);
        assert_eq!(hash_map["abc"], "xyz");
        assert_eq!(hash_map["jjj"], "123");
    }

    #[test]
    fn hash_map_to_file_content() {
        const EXPECTED_LINE_1: &str = "abc=xyz\n";
        const EXPECTED_LINE_2: &str = "jjj=123\n";
        let expected_prefix_1 = format!("{EXPECTED_LINE_1}{EXPECTED_LINE_2}");
        let expected_prefix_2 = format!("{EXPECTED_LINE_2}{EXPECTED_LINE_1}");
        assert_eq!(expected_prefix_1.len(), expected_prefix_2.len());

        let hash_map = HashMap::from_iter([
            ("abc".to_string(), "xyz".to_string()),
            ("jjj".to_string(), "123".to_string()),
        ]);

        let file_content = super::hash_map_to_file_content(&hash_map);

        assert!(file_content == expected_prefix_1 || file_content == expected_prefix_2);
    }
}
