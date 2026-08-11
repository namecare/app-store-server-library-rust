//! Runtime configuration, read from environment variables.
//!
//! Every setting has a default that matches the bundled `testNotification`
//! fixture, so `cargo run` works with no setup.

use crate::common::certs::RootSource;
use app_store_server_library::models::app_store_environment::Environment;
use std::path::PathBuf;

/// The bundled demo signing key, used when `PROMO_KEY_PATH` is unset.
const DEMO_SIGNING_KEY_PEM: &str = include_str!("../../assets/testSigningKey.p8");

#[derive(Debug)]
pub enum ConfigError {
    Invalid(String),
    Io(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Invalid(message) => write!(f, "invalid configuration: {}", message),
            ConfigError::Io(error) => write!(f, "configuration I/O error: {}", error),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::Io(error)
    }
}

/// Everything the example servers need to start.
#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub bundle_id: String,
    pub app_apple_id: Option<i64>,
    pub environment: Environment,
    pub root_source: RootSource,
    pub promo_key_pem: String,
    pub promo_key_id: String,
}

fn var_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Builds a [`Config`] from the process environment.
pub fn from_env() -> Result<Config, ConfigError> {
    let port = var_or("PORT", "8080")
        .parse::<u16>()
        .map_err(|_| ConfigError::Invalid("PORT must be a number between 1 and 65535".into()))?;

    let bundle_id = var_or("BUNDLE_ID", "com.example");

    let app_apple_id = match std::env::var("APP_APPLE_ID") {
        Ok(raw) => Some(raw.parse::<i64>().map_err(|_| {
            ConfigError::Invalid("APP_APPLE_ID must be an integer".into())
        })?),
        Err(_) => {
            println!(
                "[config] WARNING: APP_APPLE_ID unset - verifying notifications against the \
                 bundled fixture's app Apple id 1234. Real deployments must set APP_APPLE_ID to \
                 their own app's id, or every genuine notification will be rejected with 401."
            );
            Some(1234)
        }
    };

    let environment = match var_or("ENVIRONMENT", "sandbox").to_lowercase().as_str() {
        "sandbox" => Environment::Sandbox,
        "production" => Environment::Production,
        other => {
            return Err(ConfigError::Invalid(format!(
                "ENVIRONMENT must be 'sandbox' or 'production', got '{}'",
                other
            )))
        }
    };

    // An explicit certificate directory always wins; DEMO_MODE is the opt-in
    // fallback; Apple's production roots are the default.
    let root_source = if let Ok(dir) = std::env::var("APPLE_ROOT_CERTS_DIR") {
        RootSource::Directory(PathBuf::from(dir))
    } else if std::env::var("DEMO_MODE").is_ok() {
        RootSource::Demo
    } else {
        RootSource::AppleProduction
    };

    let promo_key_pem = match std::env::var("PROMO_KEY_PATH") {
        Ok(path) => std::fs::read_to_string(path)?,
        Err(_) => {
            println!(
                "[config] WARNING: PROMO_KEY_PATH unset - signing promotional offers with the \
                 bundled demo key. Signatures will not be accepted by the App Store."
            );
            DEMO_SIGNING_KEY_PEM.to_string()
        }
    };

    let promo_key_id = var_or("PROMO_KEY_ID", "DEMOKEYID");

    Ok(Config {
        port,
        bundle_id,
        app_apple_id,
        environment,
        root_source,
        promo_key_pem,
        promo_key_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn clear_env() {
        for key in [
            "PORT",
            "BUNDLE_ID",
            "APP_APPLE_ID",
            "ENVIRONMENT",
            "APPLE_ROOT_CERTS_DIR",
            "DEMO_MODE",
            "PROMO_KEY_PATH",
            "PROMO_KEY_ID",
        ] {
            std::env::remove_var(key);
        }
    }

    #[test]
    fn defaults_are_sandbox_and_the_fixture_bundle_id() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();

        let config = from_env().unwrap();

        assert_eq!(config.port, 8080);
        assert_eq!(config.bundle_id, "com.example");
        assert_eq!(config.app_apple_id, Some(1234));
        assert_eq!(config.environment, Environment::Sandbox);
        assert!(matches!(config.root_source, RootSource::AppleProduction));
    }

    #[test]
    fn demo_mode_selects_the_demo_root_source() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();
        std::env::set_var("DEMO_MODE", "1");

        let config = from_env().unwrap();

        assert!(matches!(config.root_source, RootSource::Demo));
        clear_env();
    }

    #[test]
    fn certs_dir_takes_precedence_over_demo_mode() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();
        std::env::set_var("DEMO_MODE", "1");
        std::env::set_var("APPLE_ROOT_CERTS_DIR", "/tmp/some-certs");

        let config = from_env().unwrap();

        assert!(matches!(config.root_source, RootSource::Directory(_)));
        clear_env();
    }

    #[test]
    fn production_environment_is_parsed() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();
        std::env::set_var("ENVIRONMENT", "production");

        let config = from_env().unwrap();

        assert_eq!(config.environment, Environment::Production);
        clear_env();
    }

    #[test]
    fn unknown_environment_is_rejected() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();
        std::env::set_var("ENVIRONMENT", "staging");

        let error = from_env().unwrap_err();

        assert!(matches!(error, ConfigError::Invalid(_)));
        clear_env();
    }
}
