#[derive(Debug, PartialEq, Clone)]
pub enum ConfigLine {
    Data(String),
    Content(String),
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OpenMWConfig {
    pub lines: Vec<ConfigLine>,
}

impl OpenMWConfig {
    pub fn parse(content: &str) -> Self {
        let lines = content
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("data=") {
                    let path = trimmed["data=".len()..].trim_matches('"');
                    ConfigLine::Data(path.to_string())
                } else if trimmed.starts_with("content=") {
                    let name = trimmed["content=".len()..].trim();
                    ConfigLine::Content(name.to_string())
                } else {
                    ConfigLine::Other(line.to_string())
                }
            })
            .collect();
        Self { lines }
    }

    pub fn serialize(&self) -> String {
        let mut output = String::new();
        for line in &self.lines {
            match line {
                ConfigLine::Data(path) => {
                    output.push_str(&format!("data=\"{}\"\n", path));
                }
                ConfigLine::Content(name) => {
                    output.push_str(&format!("content={}\n", name));
                }
                ConfigLine::Other(other) => {
                    output.push_str(other);
                    output.push('\n');
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_openmw_cfg() {
        let content = "data=\"/path/to/mod1\"\ndata=\"/path/to/mod2\"\ncontent=Morrowind.esm\ncontent=Tribunal.esm\nfallback=foo";
        let config = OpenMWConfig::parse(content);
        
        let expected = OpenMWConfig {
            lines: vec![
                ConfigLine::Data("/path/to/mod1".into()),
                ConfigLine::Data("/path/to/mod2".into()),
                ConfigLine::Content("Morrowind.esm".into()),
                ConfigLine::Content("Tribunal.esm".into()),
                ConfigLine::Other("fallback=foo".into()),
            ],
        };
        assert_eq!(config, expected);
    }

    #[test]
    fn test_serialize_openmw_cfg() {
        let config = OpenMWConfig {
            lines: vec![
                ConfigLine::Data("/path/to/mod1".into()),
                ConfigLine::Content("Morrowind.esm".into()),
                ConfigLine::Other("fallback=foo".into()),
            ],
        };
        let serialized = config.serialize();
        let expected = "data=\"/path/to/mod1\"\ncontent=Morrowind.esm\nfallback=foo\n";
        assert_eq!(serialized, expected);
    }
}
