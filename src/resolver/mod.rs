use std::{collections::HashMap, path::Path, fs, sync::{RwLock, Arc}, io::{BufReader, Read}};

use once_cell::sync::Lazy;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

mod mineskin;

pub use mineskin::*;

static CACHES: Lazy<Arc<RwLock<HashMap<String, SkinTexture>>>> = Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));
const CACHE_PATH: &str = "cache/";

macro_rules! read_len_prefix {
    ($buffer:expr) => {{
        let mut len = [0u8; 8];
        $buffer.read(&mut len).unwrap();
        let len = usize::from_be_bytes(len);
        
        let mut val = [0u8; 10240];
        $buffer.read(&mut val).unwrap();

        String::from_utf8_lossy(&val[..len]).to_string()
    }};
}

macro_rules! write_len_prefix {
    ($buffer:expr, $val:expr) => {
        $buffer.extend_from_slice(&$val.len().to_be_bytes());
        $buffer.extend_from_slice($val.as_bytes());
    };
}

pub fn restore_cache() {
    let path = Path::new(CACHE_PATH);
    if !path.exists() {
        return;
    }

    fs::read_dir(path).unwrap().for_each(|f| {
        let entry = f.unwrap();
        if !entry.file_type().unwrap().is_file() {
            return;
        }
        if !entry.file_name().to_str().unwrap().ends_with(".bin") {
            return;
        }

        dbg!(&entry);

        let bytes = &fs::read(entry.path()).unwrap()[..];
        let mut buffer = BufReader::new(bytes);

        let username = entry.file_name().to_str().unwrap().to_string();
        let value = read_len_prefix!(&mut buffer);
        let signature = read_len_prefix!(&mut buffer);

        drop(CACHES.write().unwrap().insert(username.split_once(".").unwrap().0.to_string(), SkinTexture { value, signature }));
        println!("Restoring cache");
    });
}

pub fn save_cache() {
    let path = Path::new(CACHE_PATH);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }

    CACHES.read().unwrap().par_iter().for_each(|(username, texture)| {
        let file = Path::new(&username).with_extension("bin");
        let path = path.join(file);

        let mut buffer = Vec::new();

        write_len_prefix!(&mut buffer, texture.value);
        write_len_prefix!(&mut buffer, texture.signature);

        fs::write(path, buffer).unwrap();
    });
}

#[derive(Debug, Clone)]
pub struct SkinTexture {
    pub value: String,
    pub signature: String
}

#[async_trait::async_trait]
#[allow(unused_variables)]
pub trait SkinResolver {
    async fn get_uuid(username: &str) -> Option<String>;
    async fn resolve(username: &str, use_cache: bool) -> Option<SkinTexture>
    {
        let lock = CACHES.read().unwrap();
        let Some(texture) = lock.get(username) else { return None };

        Some(texture.clone())
    }

    fn cached(username: &str) -> bool {
        CACHES.read().unwrap().contains_key(username)
    }
}

pub struct DefaultSkinResolver;

#[async_trait::async_trait]
impl SkinResolver for DefaultSkinResolver {
    async fn get_uuid(_: &str) -> Option<String> {
        None
    }
}