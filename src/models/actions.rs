use crate::models::ArtifactId;
use url::Url;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Artifact {
    pub id: ArtifactId,
    pub node_id: String,
    pub name: String,
    pub size_in_bytes: usize,
    pub url: Url,
    pub archive_download_url: Url,
    pub expired: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
