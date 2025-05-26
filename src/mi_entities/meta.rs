use serde::{Deserialize, Serialize};

use crate::{common::Int, Real};

use super::RolePolicies;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaLite {
    pub maintainer_name: Option<String>,
    pub maintainer_email: Option<String>,
    pub version: String,
    pub provides_tarball: Option<bool>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub uri: String,
    pub description: Option<String>,
    pub langs: Vec<String>,
    pub tos_url: Option<String>,
    pub repository_url: Option<String>,
    pub feedback_url: Option<String>,
    pub default_dark_theme: Option<String>,
    pub default_light_theme: Option<String>,
    pub disable_registration: bool,
    pub email_required_for_signup: bool,
    pub enable_hcaptcha: bool,
    pub hcaptcha_site_key: Option<String>,
    pub enable_mcaptcha: bool,
    pub mcaptcha_site_key: Option<String>,
    pub mcaptcha_instance_url: Option<String>,
    pub enable_recaptcha: bool,
    pub recaptcha_site_key: Option<String>,
    pub enable_turnstile: bool,
    pub turnstile_site_key: Option<String>,
    pub enable_testcaptcha: Option<bool>,
    pub google_analytics_measurement_id: Option<String>,
    pub sw_publickey: Option<String>,
    pub mascot_image_url: String,
    pub banner_url: Option<String>,
    pub server_error_image_url: Option<String>,
    pub info_image_url: Option<String>,
    pub not_found_image_url: Option<String>,
    pub icon_url: Option<String>,
    pub max_note_text_length: Int,
    pub ads: Vec<Ad>,
    pub notes_per_one_ad: Int, // FIXME: 確認
    pub enable_email: bool,
    pub enable_service_worker: bool,
    pub translator_available: bool,
    pub sentry_for_frontend: Option<SentryForFrontend>,
    pub media_proxy: String,
    pub enable_url_preview: bool,
    pub background_image_url: Option<String>,
    pub impressum_url: Option<String>,
    pub logo_image_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub inquiry_url: Option<String>,
    pub server_rules: Vec<String>,
    pub theme_color: Option<String>,
    pub policies: RolePolicies,
    pub note_searchable_scope: Option<NoteSearchableScope>,
    pub max_file_size: Option<Int>,
    pub federation: Option<Federation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ad {
    pub id: String,
    pub url: String,
    pub place: String,
    pub ratio: Real, // FIXME: 確認
    pub image_url: String,
    pub day_of_week: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryForFrontend {
    pub options: SentryOptions,
    pub vue_integration: Option<serde_json::Value>,
    pub browser_tracing_integration: Option<serde_json::Value>,
    pub replay_integration: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryOptions {
    pub dsn: String,

    #[serde(flatten)]
    pub other: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteSearchableScope {
    Local,
    Global,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Federation {
    All,
    Specified,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedOnly {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Features>,
    pub proxy_account_name: Option<String>,
    pub require_setup: bool,
    pub cache_remote_files: bool,
    pub cache_remote_sensitive_files: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    pub registration: bool,
    pub email_required_for_signup: bool,
    pub local_timeline: bool,
    pub global_timeline: bool,
    pub hcaptcha: bool,
    pub turnstile: bool,
    pub recaptcha: bool,
    pub object_storage: bool,
    pub service_worker: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub miauth: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDetailed {
    #[serde(flatten)]
    pub lite: MetaLite,

    #[serde(flatten)]
    pub detailed_only: DetailedOnly,
}
