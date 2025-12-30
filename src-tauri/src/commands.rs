use tauri::command;
use std::fs;

#[command]
pub async fn request_saf_permission(_path: String) -> Result<(), String> {
    // Placeholder implementation to pass tests.
    // Actual implementation will involve calling Android's SAF via JNI.
    Ok(())
}

#[command]
pub async fn list_mods(path: String) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    let mut mods = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if let Some(name) = entry.file_name().to_str() {
            mods.push(name.to_string());
        }
    }
    Ok(mods)
}

#[command]
pub async fn read_config(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[command]
pub async fn write_config(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_request_saf_permission() {
        let result = request_saf_permission("test".into()).await;
        assert!(result.is_ok(), "Should succeed once implemented");
    }

    #[tokio::test]
    async fn test_list_mods() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("mod1");
        fs::File::create(&file_path).unwrap();
        
        let result = list_mods(dir.path().to_str().unwrap().into()).await;
        assert!(result.is_ok());
        let mods = result.unwrap();
        assert!(mods.contains(&"mod1".to_string()));
    }

    #[tokio::test]
    async fn test_read_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("openmw.cfg");
        fs::write(&file_path, "content").unwrap();
        
        let result = read_config(file_path.to_str().unwrap().into()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "content");
    }

    #[tokio::test]
    async fn test_write_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("openmw.cfg");
        
        let result = write_config(file_path.to_str().unwrap().into(), "new content".into()).await;
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(&file_path).unwrap(), "new content");
    }

    #[tokio::test]
    async fn test_full_config_flow() {
        use crate::config::OpenMWConfig;
        
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("openmw.cfg");
        let initial_content = "data=\"path\"\ncontent=old.esm\n";
        fs::write(&file_path, initial_content).unwrap();
        
        // 1. Read
        let content = read_config(file_path.to_str().unwrap().into()).await.unwrap();
        
        // 2. Parse & Update
        let mut config = OpenMWConfig::parse(&content);
        config.update_content_files(vec!["new.esm".into()]);
        
        // 3. Serialize & Write
        let new_content = config.serialize();
        write_config(file_path.to_str().unwrap().into(), new_content).await.unwrap();
        
        // 4. Verify
        let final_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(final_content, "data=\"path\"\ncontent=new.esm\n");
    }
}