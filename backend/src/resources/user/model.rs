use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, Kind, RecordId, SurrealValue, Value, kind};

use super::{Role, User};
use crate::database::record_id_string;
use crate::error::AppError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
struct RoleField(pub Role);

impl SurrealValue for RoleField {
    fn kind_of() -> Kind {
        kind!(any)
    }

    fn is_value(_value: &Value) -> bool {
        true
    }

    fn into_value(self) -> Value {
        let j = serde_json::to_value(self.0).unwrap_or(serde_json::Value::Null);
        j.into_value()
    }

    fn from_value(value: Value) -> surrealdb::Result<Self> {
        let j = serde_json::Value::from_value(value)?;
        serde_json::from_value(j)
            .map(RoleField)
            .map_err(|e| surrealdb::Error::internal(e.to_string()))
    }
}

pub fn user_resource(id: &str) -> Result<(String, String), AppError> {
    if let Ok(rid) = RecordId::parse_simple(id) {
        if rid.table.as_str() == "user" {
            return Ok(("user".to_owned(), record_id_string(&rid)));
        }
        return Err(AppError::invalid_request("invalid user id"));
    }
    Ok(("user".to_owned(), id.to_owned()))
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub struct UserRecord {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RecordId>,
    email: String,
    #[serde(default)]
    role: RoleField,
    #[serde(default)]
    default_collection: Option<String>,
    created_at: Datetime,
    #[serde(default)]
    last_login_at: Option<Datetime>,
    #[serde(default)]
    request_count: u64,
    #[serde(default)]
    oauth_picture_url: Option<String>,
    #[serde(default)]
    oauth_avatar_blob_id: Option<String>,
    #[serde(default)]
    avatar_blob_id: Option<String>,
}

impl UserRecord {
    pub fn into_user(self) -> User {
        User {
            id: self.id.map(|id| record_id_string(&id)).unwrap_or_default(),
            email: self.email,
            role: self.role.0,
            default_collection: self.default_collection,
            created_at: self.created_at.into(),
            last_login_at: self.last_login_at.map(Into::into),
            request_count: self.request_count,
            oauth_picture_url: self.oauth_picture_url,
            oauth_avatar_blob_id: self.oauth_avatar_blob_id,
            avatar_blob_id: self.avatar_blob_id,
        }
    }

    pub fn from_user(user: User) -> Self {
        Self {
            id: if !user.id.is_empty() {
                Some(RecordId::new("user", user.id))
            } else {
                None
            },
            email: user.email,
            role: RoleField(user.role),
            default_collection: user.default_collection,
            created_at: user.created_at.into(),
            last_login_at: user.last_login_at.map(Into::into),
            request_count: user.request_count,
            oauth_picture_url: user.oauth_picture_url,
            oauth_avatar_blob_id: user.oauth_avatar_blob_id,
            avatar_blob_id: user.avatar_blob_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;

    #[test]
    fn user_resource_plain_id_ok() {
        let result = user_resource("some-uuid").unwrap();
        assert_eq!(result, ("user".to_owned(), "some-uuid".to_owned()));
    }

    #[test]
    fn user_resource_thing_string_ok() {
        let result = user_resource("user:someid").unwrap();
        assert_eq!(result.0, "user");
        assert_eq!(result.1, "someid");
    }

    #[test]
    fn blc_http_001_user_resource_wrong_table_err() {
        let err = user_resource("team:abc").unwrap_err();
        assert!(matches!(err, AppError::InvalidRequest(_)));
    }
}
