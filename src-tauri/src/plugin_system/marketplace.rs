use crate::plugin_system::types::*;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub manifest_url: String,
    pub download_url: String,
    pub icon_url: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub tags: Vec<String>,
    pub category: String,
    pub rating: f32,
    pub downloads: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub verified: bool,
    pub featured: bool,
    pub screenshots: Vec<String>,
    pub compatibility: CompatibilityInfo,
    pub pricing: PricingInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    pub min_app_version: String,
    pub max_app_version: Option<String>,
    pub platforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    #[serde(default)]
    pub price: Option<f32>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(rename = "is_free")]
    pub is_free: bool,
    #[serde(default)]
    pub trial_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub plugin_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceReview {
    pub id: String,
    pub plugin_id: String,
    pub author: String,
    pub rating: u8,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub helpful: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSearchQuery {
    pub query: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sort_by: Option<SearchSortBy>,
    pub price_filter: Option<PriceFilter>,
    pub rating_filter: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchSortBy {
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "downloads")]
    Downloads,
    #[serde(rename = "updated")]
    Updated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceFilter {
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSearchResult {
    pub plugins: Vec<MarketplacePlugin>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
}

pub struct MarketplaceClient {
    base_url: String,
    api_key: Option<String>,
}

impl MarketplaceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub async fn search_plugins(
        &self,
        query: MarketplaceSearchQuery,
    ) -> Result<MarketplaceSearchResult> {
        let url = format!("{}/api/plugins/search", self.base_url);
        let client = reqwest::Client::new();

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json");

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .json(&query)
            .send()
            .await
            .context("Failed to send search request")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Search request failed: {}",
                response.status()
            );
        }

        let result: MarketplaceSearchResult = response
            .json()
            .await
            .context("Failed to parse search response")?;

        Ok(result)
    }

    pub async fn get_plugin(&self, plugin_id: &str) -> Result<MarketplacePlugin> {
        let url = format!("{}/api/plugins/{}", self.base_url, plugin_id);
        let client = reqwest::Client::new();

        let mut request = client.get(&url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to get plugin")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Get plugin request failed: {}",
                response.status()
            );
        }

        let plugin: MarketplacePlugin = response
            .json()
            .await
            .context("Failed to parse plugin response")?;

        Ok(plugin)
    }

    pub async fn get_plugin_manifest(&self, plugin_id: &str) -> Result<PluginManifest> {
        let url = format!("{}/api/plugins/{}/manifest", self.base_url, plugin_id);
        let client = reqwest::Client::new();

        let mut request = client.get(&url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to get plugin manifest")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Get manifest request failed: {}",
                response.status()
            );
        }

        let manifest: PluginManifest = response
            .json()
            .await
            .context("Failed to parse manifest response")?;

        Ok(manifest)
    }

    pub async fn download_plugin(&self, plugin_id: &str) -> Result<Vec<u8>> {
        let plugin = self.get_plugin(plugin_id).await?;

        let client = reqwest::Client::new();
        let mut request = client.get(&plugin.download_url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to download plugin")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Download request failed: {}",
                response.status()
            );
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to download plugin bytes")?;

        Ok(bytes.to_vec())
    }

    pub async fn get_categories(&self) -> Result<Vec<MarketplaceCategory>> {
        let url = format!("{}/api/categories", self.base_url);
        let client = reqwest::Client::new();

        let mut request = client.get(&url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to get categories")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Get categories request failed: {}",
                response.status()
            );
        }

        let categories: Vec<MarketplaceCategory> = response
            .json()
            .await
            .context("Failed to parse categories response")?;

        Ok(categories)
    }

    pub async fn get_reviews(&self, plugin_id: &str) -> Result<Vec<MarketplaceReview>> {
        let url = format!("{}/api/plugins/{}/reviews", self.base_url, plugin_id);
        let client = reqwest::Client::new();

        let mut request = client.get(&url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to get reviews")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Get reviews request failed: {}",
                response.status()
            );
        }

        let reviews: Vec<MarketplaceReview> = response
            .json()
            .await
            .context("Failed to parse reviews response")?;

        Ok(reviews)
    }

    pub async fn submit_review(
        &self,
        plugin_id: &str,
        review: MarketplaceReview,
    ) -> Result<()> {
        let url = format!("{}/api/plugins/{}/reviews", self.base_url, plugin_id);
        let client = reqwest::Client::new();

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json");

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .json(&review)
            .send()
            .await
            .context("Failed to submit review")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Submit review request failed: {}",
                response.status()
            );
        }

        Ok(())
    }

    pub async fn get_featured_plugins(&self) -> Result<Vec<MarketplacePlugin>> {
        let url = format!("{}/api/plugins/featured", self.base_url);
        let client = reqwest::Client::new();

        let mut request = client.get(&url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to get featured plugins")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Get featured plugins request failed: {}",
                response.status()
            );
        }

        let plugins: Vec<MarketplacePlugin> = response
            .json()
            .await
            .context("Failed to parse featured plugins response")?;

        Ok(plugins)
    }

    pub async fn report_plugin(&self, plugin_id: &str, reason: String) -> Result<()> {
        let url = format!("{}/api/plugins/{}/report", self.base_url, plugin_id);
        let client = reqwest::Client::new();

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "reason": reason }));

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to report plugin")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Report plugin request failed: {}",
                response.status()
            );
        }

        Ok(())
    }
}

impl Default for MarketplaceClient {
    fn default() -> Self {
        Self::new("https://marketplace.ainovelstudio.com".to_string())
    }
}
