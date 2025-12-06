use serde::{Deserialize, Serialize};

/// Configuration commune pour tous les outils
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
}

impl AppConfig {
    pub fn new(app_name: &str, version: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            version: version.to_string(),
        }
    }
}

/// Fonction utilitaire commune
pub fn format_message(prefix: &str, message: &str) -> String {
    format!("[{}] {}", prefix, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_message() {
        let result = format_message("INFO", "Test message");
        assert_eq!(result, "[INFO] Test message");
    }
}
