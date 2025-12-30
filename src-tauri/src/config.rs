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
    pub fn parse(_content: &str) -> Self {
        // Placeholder
        Self { lines: vec![] }
    }

    pub fn serialize(&self) -> String {
        // Placeholder
        String::new()
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