use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NexusMod {
    pub name: String,
    pub summary: String,
    pub author: String,
    pub version: String,
    pub downloads: u64,
    pub endorsements: u32,
    pub updated_timestamp: u64,
    pub picture_url: String,
    pub mod_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NexusModList {
    pub mods: Vec<NexusMod>,
}

pub struct NexusClient {
    api_key: String,
    client: reqwest::Client,
}

impl NexusClient {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok(); // Load .env if present
        let api_key = std::env::var("NEXUS_API_KEY")
            .map_err(|_| "NEXUS_API_KEY not found in environment".to_string())?;
        
        Ok(Self {
            api_key,
            client: reqwest::Client::new(),
        })
    }

    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert(USER_AGENT, HeaderValue::from_static("OpenMW-Android-Modloader/0.1.0"));
        headers
    }

    pub async fn get_mod_details(&self, game: &str, mod_id: u32) -> Result<NexusMod, String> {
        let url = format!("https://api.nexusmods.com/v1/games/{}/mods/{}.json", game, mod_id);
        let response = self.client.get(&url)
            .headers(self.headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<NexusMod>().await.map_err(|e| e.to_string())
        } else {
            Err(format!("API error: {}", response.status()))
        }
    }

    pub async fn get_featured_mods(&self, game: &str) -> Result<Vec<NexusMod>, String> {
        // Note: Nexus API doesn't have a single "featured" endpoint in v1
        // Usually, we fetch by certain criteria or trending.
        // For now, let's implement trending.
        let url = format!("https://api.nexusmods.com/v1/games/{}/mods/trending.json", game);
        let response = self.client.get(&url)
            .headers(self.headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<Vec<NexusMod>>().await.map_err(|e| e.to_string())
        } else {
            Err(format!("API error: {}", response.status()))
        }
    }

    pub async fn search_mods(&self, game: &str, query: &str) -> Result<Vec<NexusMod>, String> {
        // ... (previous implementation)
        let url = format!("https://api.nexusmods.com/v1/games/{}/mods/search.json", game);
        let response = self.client.get(&url)
            .headers(self.headers())
            .query(&[("terms", query)])
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            response.json::<Vec<NexusMod>>().await.map_err(|e| e.to_string())
        } else {
            Err(format!("Search API error: {}", response.status()))
        }
    }

    pub async fn get_download_link(&self, game: &str, mod_id: u32, file_id: u32) -> Result<String, String> {
        let url = format!("https://api.nexusmods.com/v1/games/{}/mods/{}/files/{}/download_link.json", game, mod_id, file_id);
        let response = self.client.get(&url)
            .headers(self.headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        #[derive(Deserialize)]
        struct DownloadLinkResponse {
            #[serde(rename = "URI")]
            uri: String,
        }

        if response.status().is_success() {
            let res = response.json::<Vec<DownloadLinkResponse>>().await.map_err(|e| e.to_string())?;
            res.first().map(|r| r.uri.clone()).ok_or("No download link found".to_string())
        } else {
            Err(format!("Download link API error: {}", response.status()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nexus_client_headers() {
        let client = NexusClient::new("test_key".to_string());
        let headers = client.headers();
        assert_eq!(headers.get("apikey").unwrap(), "test_key");
        assert!(headers.get(USER_AGENT).is_some());
    }

    // We can't easily test the actual API calls without mocking or a real key.
    // In Phase 1 Task 5, we'll write more comprehensive tests possibly using wiremock.
}
