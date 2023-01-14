use chrono::{DateTime, Utc};
use typed_builder::TypedBuilder;

use super::{ErasedId, Id};

pub type TaskId = Id<Task>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    pub id: TaskId,
    pub post_id: Option<ErasedId>,
    pub name: String,
    pub description: String,
    pub is_closed: bool,
    pub date_to_publish: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateTask {
    pub name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UpdateTask {
    pub post_id: Option<Option<ErasedId>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_closed: Option<bool>,
    pub date_to_publish: Option<Option<DateTime<Utc>>>,
}
