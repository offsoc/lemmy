use crate::newtypes::{LocalUserId, PersonId, RegistrationApplicationId};
use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use lemmy_db_schema_file::schema::registration_application;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[cfg(feature = "full")]
use ts_rs::TS;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Selectable, Identifiable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = registration_application))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", ts(export))]
/// A registration application.
pub struct RegistrationApplication {
  pub id: RegistrationApplicationId,
  pub local_user_id: LocalUserId,
  pub answer: String,
  #[cfg_attr(feature = "full", ts(optional))]
  pub admin_id: Option<PersonId>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub deny_reason: Option<String>,
  pub published_at: DateTime<Utc>,
}

#[cfg_attr(feature = "full", derive(Insertable))]
#[cfg_attr(feature = "full", diesel(table_name = registration_application))]
pub struct RegistrationApplicationInsertForm {
  pub local_user_id: LocalUserId,
  pub answer: String,
}

#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = registration_application))]
pub struct RegistrationApplicationUpdateForm {
  pub admin_id: Option<Option<PersonId>>,
  pub deny_reason: Option<Option<String>>,
}
