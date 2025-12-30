use std::fs;

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
                if let Some(path) = trimmed.strip_prefix("data=") {
                    ConfigLine::Data(path.trim().trim_matches('"').trim().to_string())
                } else if let Some(name) = trimmed.strip_prefix("content=") {
                    ConfigLine::Content(name.trim().to_string())
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

    pub fn get_data_paths(&self) -> Vec<String> {
        self.lines
            .iter()
            .filter_map(|line| {
                if let ConfigLine::Data(path) = line {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_content_files(&self) -> Vec<String> {
        self.lines
            .iter()
            .filter_map(|line| {
                if let ConfigLine::Content(name) = line {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn update_content_files(&mut self, new_content: Vec<String>) {
        self.lines.retain(|line| !matches!(line, ConfigLine::Content(_)));
        
        let mut last_data_idx = 0;
        let mut found_data = false;
        for (i, line) in self.lines.iter().enumerate() {
            if matches!(line, ConfigLine::Data(_)) {
                last_data_idx = i;
                found_data = true;
            }
        }
        
        let insert_at = if found_data { last_data_idx + 1 } else { 0 };
        
        let mut new_lines = new_content.into_iter().map(ConfigLine::Content).collect::<Vec<_>>();
        
        let mut tail = self.lines.split_off(insert_at);
        self.lines.append(&mut new_lines);
        self.lines.append(&mut tail);
    }

    pub fn update_data_paths(&mut self, new_paths: Vec<String>) {
        self.lines.retain(|line| !matches!(line, ConfigLine::Data(_)));
        let mut new_lines = new_paths.into_iter().map(ConfigLine::Data).collect::<Vec<_>>();
        let mut old_lines = self.lines.split_off(0);
        self.lines.append(&mut new_lines);
        self.lines.append(&mut old_lines);
    }

    pub fn backup(path: &str) -> Result<(), String> {
        let backup_path = format!("{}.bak", path);
        fs::copy(path, backup_path).map(|_| ()).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_openmw_cfg() {
        let content = "data=\" /path/to/mod1\"
data=\" /path/to/mod2\"
content=Morrowind.esm
content=Tribunal.esm
fallback=foo";
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

    #[test]
    fn test_update_content_files() {
        let mut config = OpenMWConfig::parse("data=\"path\"
content=old.esm
other=val");
        config.update_content_files(vec!["new1.esm".into(), "new2.esm".into()]);
        
        let expected = "data=\"path\"
content=new1.esm
content=new2.esm
other=val\n";
        assert_eq!(config.serialize(), expected);
    }

    #[test]
    fn test_update_data_paths() {
        let mut config = OpenMWConfig::parse("data=\"old\"
content=file.esm");
        config.update_data_paths(vec!["new1".into(), "new2".into()]);
        
        let expected = "data=\"new1\"
data=\"new2\"
content=file.esm\n";
        assert_eq!(config.serialize(), expected);
    }

    #[test]
    fn test_backup() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("openmw.cfg");
        fs::write(&file_path, "content").unwrap();
        
        let result = OpenMWConfig::backup(file_path.to_str().unwrap());
        assert!(result.is_ok());
        
        let backup_path = dir.path().join("openmw.cfg.bak");
        assert!(backup_path.exists());
        assert_eq!(fs::read_to_string(backup_path).unwrap(), "content");
    }
}
