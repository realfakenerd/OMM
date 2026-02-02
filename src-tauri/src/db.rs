use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_db<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Table for caching general API responses (featured, trending, search)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS api_cache (
            endpoint TEXT PRIMARY KEY,
            response_json TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    )?;

    // Table for detailed mod metadata
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mod_metadata (
            mod_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            summary TEXT,
            author TEXT,
            version TEXT,
            downloads INTEGER,
            endorsements INTEGER,
            updated_timestamp INTEGER,
            picture_url TEXT,
            last_synced INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn cache_api_response(conn: &Connection, endpoint: &str, response: &str) -> Result<()> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    conn.execute(
        "INSERT OR REPLACE INTO api_cache (endpoint, response_json, timestamp) VALUES (?1, ?2, ?3)",
        rusqlite::params![endpoint, response, timestamp],
    )?;
    Ok(())
}

pub fn get_cached_api_response(conn: &Connection, endpoint: &str, max_age_secs: u64) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT response_json, timestamp FROM api_cache WHERE endpoint = ?1")?;
    let mut rows = stmt.query([endpoint])?;

    if let Some(row) = rows.next()? {
        let timestamp: u64 = row.get(1)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now - timestamp < max_age_secs {
            return Ok(Some(row.get(0)?));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db() {
        let result = init_db(":memory:");
        assert!(result.is_ok());
    }

    #[test]
    fn test_cache_api_response() {
        let conn = init_db(":memory:").unwrap();
        let endpoint = "test_endpoint";
        let response = "{\"key\": \"value\"}";
        
        let result = cache_api_response(&conn, endpoint, response);
        assert!(result.is_ok());

        let cached = get_cached_api_response(&conn, endpoint, 3600).unwrap();
        assert_eq!(cached, Some(response.to_string()));
    }

    #[test]
    fn test_cache_expiration() {
        let conn = init_db(":memory:").unwrap();
        let endpoint = "test_endpoint";
        let response = "{\"key\": \"value\"}";
        
        cache_api_response(&conn, endpoint, response).unwrap();

        // Expired (max_age = 0)
        let cached = get_cached_api_response(&conn, endpoint, 0).unwrap();
        assert_eq!(cached, None);
    }
}
