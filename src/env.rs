pub fn load_env(key: &str) -> Result<String, String> {
    let result = std::env::var(key);
    result.map_err(|_| format!("Failed to load env variable ${key}!"))
}
