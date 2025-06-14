use crate::CommunityView;
use lemmy_db_schema::{
  newtypes::{CommunityId, LanguageId, PaginationCursor, PersonId, TagId},
  source::site::Site,
  CommunitySortType,
};
use lemmy_db_schema_file::enums::{CommunityVisibility, ListingType};
use lemmy_db_views_community_moderator::CommunityModeratorView;
use lemmy_db_views_person::PersonView;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[cfg(feature = "full")]
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Add a moderator to a community.
pub struct AddModToCommunity {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub added: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The response of adding a moderator to a community.
pub struct AddModToCommunityResponse {
  pub moderators: Vec<CommunityModeratorView>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ApproveCommunityPendingFollower {
  pub community_id: CommunityId,
  pub follower_id: PersonId,
  pub approve: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Ban a user from a community.
pub struct BanFromCommunity {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub ban: bool,
  /// Optionally remove or restore all their data. Useful for new troll accounts.
  /// If ban is true, then this means remove. If ban is false, it means restore.
  #[cfg_attr(feature = "full", ts(optional))]
  pub remove_or_restore_data: Option<bool>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub reason: Option<String>,
  /// A time that the ban will expire, in unix epoch seconds.
  ///
  /// An i64 unix timestamp is used for a simpler API client implementation.
  #[cfg_attr(feature = "full", ts(optional))]
  pub expires_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The response for banning a user from a community.
pub struct BanFromCommunityResponse {
  pub person_view: PersonView,
  pub banned: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Block a community.
pub struct BlockCommunity {
  pub community_id: CommunityId,
  pub block: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The block community response.
pub struct BlockCommunityResponse {
  pub community_view: CommunityView,
  pub blocked: bool,
}

/// Parameter for setting community icon or banner. Can't use POST data here as it already contains
/// the image data.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommunityIdQuery {
  pub id: CommunityId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// A simple community response.
pub struct CommunityResponse {
  pub community_view: CommunityView,
  pub discussion_languages: Vec<LanguageId>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
/// Create a community.
pub struct CreateCommunity {
  /// The unique name.
  pub name: String,
  /// A longer title.
  pub title: String,
  /// A sidebar for the community in markdown.
  #[cfg_attr(feature = "full", ts(optional))]
  pub sidebar: Option<String>,
  /// A shorter, one line description of your community.
  #[cfg_attr(feature = "full", ts(optional))]
  pub description: Option<String>,
  /// An icon URL.
  #[cfg_attr(feature = "full", ts(optional))]
  pub icon: Option<String>,
  /// A banner URL.
  #[cfg_attr(feature = "full", ts(optional))]
  pub banner: Option<String>,
  /// Whether its an NSFW community.
  #[cfg_attr(feature = "full", ts(optional))]
  pub nsfw: Option<bool>,
  /// Whether to restrict posting only to moderators.
  #[cfg_attr(feature = "full", ts(optional))]
  pub posting_restricted_to_mods: Option<bool>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub discussion_languages: Option<Vec<LanguageId>>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub visibility: Option<CommunityVisibility>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Create a tag for a community.
pub struct CreateCommunityTag {
  pub community_id: CommunityId,
  pub display_name: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Delete your own community.
pub struct DeleteCommunity {
  pub community_id: CommunityId,
  pub deleted: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Edit a community.
pub struct EditCommunity {
  pub community_id: CommunityId,
  /// A longer title.
  #[cfg_attr(feature = "full", ts(optional))]
  pub title: Option<String>,
  /// A sidebar for the community in markdown.
  #[cfg_attr(feature = "full", ts(optional))]
  pub sidebar: Option<String>,
  /// A shorter, one line description of your community.
  #[cfg_attr(feature = "full", ts(optional))]
  pub description: Option<String>,
  /// Whether its an NSFW community.
  #[cfg_attr(feature = "full", ts(optional))]
  pub nsfw: Option<bool>,
  /// Whether to restrict posting only to moderators.
  #[cfg_attr(feature = "full", ts(optional))]
  pub posting_restricted_to_mods: Option<bool>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub discussion_languages: Option<Vec<LanguageId>>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub visibility: Option<CommunityVisibility>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Follow / subscribe to a community.
pub struct FollowCommunity {
  pub community_id: CommunityId,
  pub follow: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
// TODO make this into a tagged enum
/// Get a community. Must provide either an id, or a name.
pub struct GetCommunity {
  #[cfg_attr(feature = "full", ts(optional))]
  pub id: Option<CommunityId>,
  /// Example: star_trek , or star_trek@xyz.tld
  #[cfg_attr(feature = "full", ts(optional))]
  pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The community response.
pub struct GetCommunityResponse {
  pub community_view: CommunityView,
  #[cfg_attr(feature = "full", ts(optional))]
  pub site: Option<Site>,
  pub moderators: Vec<CommunityModeratorView>,
  pub discussion_languages: Vec<LanguageId>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Fetches a random community
pub struct GetRandomCommunity {
  #[cfg_attr(feature = "full", ts(optional))]
  pub type_: Option<ListingType>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub show_nsfw: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Hide a community from the main view.
pub struct HideCommunity {
  pub community_id: CommunityId,
  pub hidden: bool,
  #[cfg_attr(feature = "full", ts(optional))]
  pub reason: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Fetches a list of communities.
pub struct ListCommunities {
  #[cfg_attr(feature = "full", ts(optional))]
  pub type_: Option<ListingType>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub sort: Option<CommunitySortType>,
  #[cfg_attr(feature = "full", ts(optional))]
  /// Filter to within a given time range, in seconds.
  /// IE 60 would give results for the past minute.
  pub time_range_seconds: Option<i32>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub show_nsfw: Option<bool>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub page_cursor: Option<PaginationCursor>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub page_back: Option<bool>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The response for listing communities.
pub struct ListCommunitiesResponse {
  pub communities: Vec<CommunityView>,
  /// the pagination cursor to use to fetch the next page
  #[cfg_attr(feature = "full", ts(optional))]
  pub next_page: Option<PaginationCursor>,
  #[cfg_attr(feature = "full", ts(optional))]
  pub prev_page: Option<PaginationCursor>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Purges a community from the database. This will delete all content attached to that community.
pub struct PurgeCommunity {
  pub community_id: CommunityId,
  #[cfg_attr(feature = "full", ts(optional))]
  pub reason: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Remove a community (only doable by moderators).
pub struct RemoveCommunity {
  pub community_id: CommunityId,
  pub removed: bool,
  #[cfg_attr(feature = "full", ts(optional))]
  pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Transfer a community to a new owner.
pub struct TransferCommunity {
  pub community_id: CommunityId,
  pub person_id: PersonId,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Update a community tag.
pub struct UpdateCommunityTag {
  pub tag_id: TagId,
  pub display_name: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "full", derive(TS))]
#[cfg_attr(feature = "full", ts(export))]
/// Delete a community tag.
pub struct DeleteCommunityTag {
  pub tag_id: TagId,
}
