use std::thread;

use serde::{Deserialize, Serialize};

use super::*;

const ENDPOINT: &str = "https://api.minetools.eu";

#[derive(Serialize, Deserialize)]
struct MineskinUUID {
    id: Option<String>,
    status: String
}

#[derive(Serialize, Deserialize)]
struct MineskinProfileProperties {
    name: String,
    signature: String,
    value: String
}

#[derive(Serialize, Deserialize)]
struct MineskinProfile {
    properties: Vec<MineskinProfileProperties>,
    status: String
}

pub struct MineskinResolver;

#[async_trait::async_trait]
impl SkinResolver for MineskinResolver {
    async fn get_uuid(username: &str) -> Option<String> {
        let result = reqwest::get(format!("{ENDPOINT}/uuid/{username}")).await.unwrap().text().await.unwrap();
        let Ok(result): Result<MineskinUUID, _> = serde_json::from_str(&result) else { return None };

        if result.status != "OK" { return None }

        result.id
    }

    async fn resolve(username: &str, use_cache: bool) -> Option<SkinTexture> {
        if use_cache && MineskinResolver::cached(username) {
            return DefaultSkinResolver::resolve(username, use_cache).await
        }

        let Some(uuid) = MineskinResolver::get_uuid(username).await else { return None };

        let result = reqwest::get(format!("{ENDPOINT}/profile/{uuid}")).await.unwrap().text().await.unwrap();

        let Ok(value) = serde_json::from_str::<serde_json::Value>(&result) else { return None };
        let Some(value) = value.as_object() else { return None };
        let Some(raw) = value.get("raw") else { return None };
        let Ok(result) = serde_json::from_value::<MineskinProfile>(raw.to_owned()) else { return None };

        if result.status != "OK" { return None }

        let Some(properties) = result.properties.iter().filter(|v| v.name == "textures").nth(0) else { return None };

        let texture = SkinTexture { value: properties.value.to_owned(), signature: properties.signature.to_owned() };

        drop(CACHES.write().unwrap().insert(username.to_string(), texture.clone()));
        thread::spawn(save_cache);

        Some(texture)
    }
}