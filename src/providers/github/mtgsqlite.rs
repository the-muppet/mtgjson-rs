use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

use super::{MtgjsonConfig, ProviderError, Result, BaseProvider};

pub struct GitHubMTGSqliteProvider {
    _provider: Arc<BaseProvider>,
    repo_url: String,
    temp_download_path: PathBuf,
    output_path: PathBuf,
}

impl GitHubMTGSqliteProvider {
    const REPO_URL: &'static str = "https://github.com/mtgjson/mtgsqlive/";

    pub fn new(config: MtgjsonConfig, output_path: PathBuf, cache_path: PathBuf) -> Result<Self> {
        let temp_download_path = cache_path.join("GitHub-MTGSQLive");

        Ok(Self {
            _provider: Arc::new(BaseProvider::new(config)?),
            repo_url: Self::REPO_URL.to_string(),
            temp_download_path,
            output_path,
        })
    }

    pub fn download_and_install(&self) -> Result<()> {
        log::info!("Cloning MTGSQLive repository from {}", self.repo_url);

        if self.temp_download_path.exists() {
            std::fs::remove_dir_all(&self.temp_download_path)
                .map_err(|e| ProviderError::ConfigError(format!("Failed to remove temp dir: {}", e)))?;
        }

        let clone_status = Command::new("git")
            .args(&["clone", &self.repo_url, self.temp_download_path.to_str().unwrap(), "--depth", "1"])
            .status()
            .map_err(|e| ProviderError::ConfigError(format!("Git clone failed: {}", e)))?;

        if !clone_status.success() {
            return Err(ProviderError::ConfigError("Git clone command failed".to_string()));
        }

        log::info!("Installing MTGSQLive via pip");

        let pip_status = Command::new("python")
            .args(&["-m", "pip", "install", self.temp_download_path.to_str().unwrap()])
            .status()
            .map_err(|e| ProviderError::ConfigError(format!("Pip install failed: {}", e)))?;

        if !pip_status.success() {
            log::warn!("Pip install failed, but continuing");
        }

        Ok(())
    }

    pub fn build_alternative_formats(&self) -> Result<()> {
        log::info!("Building MTGSQLive alternative formats...");

        let status = Command::new("python")
            .args(&[
                "-m",
                "mtgsqlive",
                "-i",
                self.output_path.to_str().unwrap(),
                "-o",
                self.output_path.to_str().unwrap(),
                "--all",
            ])
            .status()
            .map_err(|e| ProviderError::ConfigError(format!("MTGSQLive build failed: {}", e)))?;

        if !status.success() {
            return Err(ProviderError::ConfigError("MTGSQLive build command failed".to_string()));
        }

        log::info!("MTGSQLive build completed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_mtgsqlite_provider_creation() {
        let config = MtgjsonConfig::default();
        let output_path = PathBuf::from("/tmp/output");
        let cache_path = PathBuf::from("/tmp/cache");

        let provider = GitHubMTGSqliteProvider::new(config, output_path, cache_path);
        assert!(provider.is_ok());
    }
}
