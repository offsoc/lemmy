use crate::newtypes::{LocalUserId, OAuthProviderId};
use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use lemmy_db_schema_file::schema::oauth_account;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[cfg(feature = "full")]
use ts_rs::TS;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Selectable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = oauth_account))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", ts(export))]
/// An auth account method.
pub struct OAuthAccount {
  pub local_user_id: LocalUserId,
  pub oauth_provider_id: OAuthProviderId,
  pub oauth_user_id: String,
  pub published_at: DateTime<Utc>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, derive_new::new)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = oauth_account))]
pub struct OAuthAccountInsertForm {
  pub local_user_id: LocalUserId,
  pub oauth_provider_id: OAuthProviderId,
  pub oauth_user_id: String,
}
