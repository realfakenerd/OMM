use tauri::command;

#[tauri::command]
pub async fn request_saf_permission(path: String) -> Result<(), String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub async fn list_mods(path: String) -> Result<Vec<String>, String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub async fn read_config(path: String) -> Result<String, String> {
    Err("Not implemented".into())
}

#[tauri::command]
pub async fn write_config(path: String, content: String) -> Result<(), String> {
    Err("Not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_saf_permission() {
        let result = request_saf_permission("test".into()).await;
        assert!(result.is_ok(), "Should succeed once implemented");
    }

    #[tokio::test]
    async fn test_list_mods() {
        let result = list_mods("test".into()).await;
        assert!(result.is_ok(), "Should succeed once implemented");
    }

    #[tokio::test]
    async fn test_read_config() {
        let result = read_config("test".into()).await;
        assert!(result.is_ok(), "Should succeed once implemented");
    }

    #[tokio::test]
    async fn test_write_config() {
        let result = write_config("test".into(), "content".into()).await;
        assert!(result.is_ok(), "Should succeed once implemented");
    }
}
