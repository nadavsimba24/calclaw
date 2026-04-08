//! Version management for CalcLaw Complete
//! Provides version checking and update notification functionality

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub commit_hash: Option<String>,
    pub build_date: Option<String>,
    pub rust_version: Option<String>,
}

/// Update check state
#[derive(Debug)]
pub struct UpdateChecker {
    last_check: RwLock<SystemTime>,
    check_interval: Duration,
    current_version: String,
}

impl UpdateChecker {
    /// Create a new update checker
    pub fn new(current_version: &str, check_interval_hours: u64) -> Self {
        Self {
            last_check: RwLock::new(SystemTime::now() - Duration::from_secs(25 * 3600)), // Force first check
            check_interval: Duration::from_secs(check_interval_hours * 3600),
            current_version: current_version.to_string(),
        }
    }

    /// Get current version info
    pub fn get_version_info(&self) -> VersionInfo {
        VersionInfo {
            version: self.current_version.clone(),
            commit_hash: option_env!("GIT_COMMIT_HASH").map(|s| s.to_string()),
            build_date: option_env!("BUILD_DATE").map(|s| s.to_string()),
            rust_version: option_env!("RUST_VERSION").map(|s| s.to_string()),
        }
    }

    /// Check for updates (non-blocking, runs in background)
    pub async fn check_for_updates(&self) -> Option<UpdateAvailable> {
        // Check if enough time has passed since last check
        let now = SystemTime::now();
        let last_check = *self.last_check.read().await;
        
        if now.duration_since(last_check).unwrap_or_default() < self.check_interval {
            return None;
        }

        // Update last check time
        *self.last_check.write().await = now;

        // Perform update check in background
        let current_version = self.current_version.clone();
        tokio::spawn(async move {
            match Self::fetch_latest_version().await {
                Ok(latest) => {
                    if latest.version != current_version {
                        info!(
                            "Update available: {} -> {}",
                            current_version, latest.version
                        );
                        info!("Release notes: {}", latest.release_url);
                    }
                }
                Err(e) => {
                    warn!("Failed to check for updates: {}", e);
                }
            }
        });

        None
    }

    /// Fetch latest version from GitHub
    async fn fetch_latest_version() -> Result<LatestRelease, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/repos/yourusername/calclaw/releases/latest")
            .header("User-Agent", "CalcLaw-Update-Checker")
            .send()
            .await?;

        let release: GitHubRelease = response.json().await?;
        
        Ok(LatestRelease {
            version: release.tag_name.trim_start_matches('v').to_string(),
            release_url: release.html_url,
            published_at: release.published_at,
            body: release.body,
        })
    }

    /// Force an immediate update check
    pub async fn force_update_check(&self) -> Result<Option<UpdateAvailable>, reqwest::Error> {
        let latest = Self::fetch_latest_version().await?;
        
        if latest.version != self.current_version {
            Ok(Some(UpdateAvailable {
                current_version: self.current_version.clone(),
                latest_version: latest.version,
                release_url: latest.release_url,
                published_at: latest.published_at,
                release_notes: latest.body.unwrap_or_default(),
            }))
        } else {
            Ok(None)
        }
    }
}

/// Information about an available update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAvailable {
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub published_at: String,
    pub release_notes: String,
}

/// Latest release information from GitHub
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    published_at: String,
    body: Option<String>,
}

/// Information about the latest release
#[derive(Debug)]
struct LatestRelease {
    version: String,
    release_url: String,
    published_at: String,
    body: Option<String>,
}

/// API endpoints for version management
pub mod api {
    use super::*;
    use axum::{
        extract::State,
        response::{IntoResponse, Json},
        routing::get,
        Router,
    };
    use std::sync::Arc;

    /// Get current version information
    pub async fn get_version(
        State(checker): State<Arc<UpdateChecker>>,
    ) -> impl IntoResponse {
        Json(checker.get_version_info())
    }

    /// Check for updates
    pub async fn check_updates(
        State(checker): State<Arc<UpdateChecker>>,
    ) -> impl IntoResponse {
        match checker.force_update_check().await {
            Ok(Some(update)) => Json(serde_json::json!({
                "update_available": true,
                "update": update
            })),
            Ok(None) => Json(serde_json::json!({
                "update_available": false,
                "message": "You're running the latest version"
            })),
            Err(e) => Json(serde_json::json!({
                "update_available": false,
                "error": e.to_string(),
                "message": "Failed to check for updates"
            })),
        }
    }

    /// Setup version API routes
    pub fn routes() -> Router<Arc<UpdateChecker>> {
        Router::new()
            .route("/api/version", get(get_version))
            .route("/api/version/check-updates", get(check_updates))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let checker = UpdateChecker::new("1.0.0", 24);
        let info = checker.get_version_info();
        
        assert_eq!(info.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_update_checker_initialization() {
        let checker = UpdateChecker::new("1.0.0", 24);
        
        // Should not check immediately
        let update = checker.check_for_updates().await;
        assert!(update.is_none());
        
        // Get version info
        let info = checker.get_version_info();
        assert_eq!(info.version, "1.0.0");
    }
}