use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Interested,
    Owned,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Interested => "interested",
            Status::Owned => "owned",
        }
    }
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "interested" => Ok(Status::Interested),
            "owned" => Ok(Status::Owned),
            other => Err(FromSqlError::Other(
                format!("unknown status: {other}").into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Completed,
    Dropped,
}

impl Outcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Completed => "completed",
            Outcome::Dropped => "dropped",
        }
    }
}

impl FromSql for Outcome {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "completed" => Ok(Outcome::Completed),
            "dropped" => Ok(Outcome::Dropped),
            other => Err(FromSqlError::Other(
                format!("unknown outcome: {other}").into(),
            )),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GameFilter {
    pub status: Option<Status>,
    pub on_shelf: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ShelfEntry {
    pub slot: i64,
    pub game: Option<GameSummary>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RemoveResult {
    Finished {
        #[serde(rename = "ratingId")]
        rating_id: Option<i64>,
    },
    Dropped,
    NotFinished,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSummary {
    pub id: i64,
    pub title: String,
    pub release_year: Option<i64>,
    pub status: Status,
    pub outcome: Option<Outcome>,
    pub shelf_slot: Option<i64>,
    pub cover_path: Option<String>,
    pub featured_path: Option<String>,
    pub screenshot_paths: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub id: i64,
    pub label: String,
    pub sort_order: i64,
    pub color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Platform {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot {
    pub id: i64,
    pub path: String,
    pub sort_order: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDetail {
    pub id: i64,
    pub title: String,
    pub release_year: Option<i64>,
    pub status: Status,
    pub outcome: Option<Outcome>,
    pub shelf_slot: Option<i64>,
    pub cover_path: Option<String>,
    pub featured_screenshot_id: Option<i64>,
    pub notes: String,
    pub rawg_id: Option<i64>,
    pub rating: Option<Rating>,
    pub platforms: Vec<GamePlatform>,
    pub genres: Vec<Genre>,
    pub screenshots: Vec<Screenshot>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGame {
    pub title: String,
    pub release_year: Option<i64>,
    pub platforms: Vec<PlatformLink>,
    pub genre_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEdit {
    pub title: String,
    pub release_year: Option<i64>,
    pub platforms: Vec<PlatformLink>,
    pub genre_ids: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlatformState {
    Owned,
    Wanted,
}

impl PlatformState {
    pub fn as_str(self) -> &'static str {
        match self {
            PlatformState::Owned => "owned",
            PlatformState::Wanted => "wanted",
        }
    }
}

impl FromSql for PlatformState {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "owned" => Ok(PlatformState::Owned),
            "wanted" => Ok(PlatformState::Wanted),
            other => Err(FromSqlError::Other(
                format!("unknown platform state: {other}").into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformLink {
    pub platform_id: i64,
    pub state: PlatformState,
}

#[derive(Debug, Serialize)]
pub struct GamePlatform {
    pub id: i64,
    pub name: String,
    pub state: PlatformState,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewGame {
    pub id: i64,
    pub title: String,
    pub release_year: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DeletePreview {
    pub total: i64,
    pub first: Vec<PreviewGame>,
    pub blocked: Vec<String>,
}