use crate::newtypes::{CommentId, CommentReportId, PersonId};
use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use lemmy_db_schema_file::schema::comment_report;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[cfg(feature = "full")]
use ts_rs::TS;

#[skip_serializing_none]
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
  feature = "full",
  derive(Queryable, Selectable, Associations, Identifiable, TS)
)]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::comment::Comment)))]
#[cfg_attr(feature = "full", diesel(table_name = comment_report))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", ts(export))]
/// A comment report.
pub struct CommentReport {
  pub id: CommentReportId,
  pub creator_id: PersonId,
  pub comment_id: CommentId,
  pub original_comment_text: String,
  pub reason: String,
  pub resolved: bool,
  #[cfg_attr(feature = "full", ts(optional))]
  pub resolver_id: Option<PersonId>,
  pub published_at: DateTime<Utc>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub updated_at: Option<DateTime<Utc>>,
  pub violates_instance_rules: bool,
}

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = comment_report))]
pub struct CommentReportForm {
  pub creator_id: PersonId,
  pub comment_id: CommentId,
  pub original_comment_text: String,
  pub reason: String,
  pub violates_instance_rules: bool,
}
