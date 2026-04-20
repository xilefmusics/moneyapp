DEFINE TABLE OVERWRITE user TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD OVERWRITE created_at ON user TYPE datetime READONLY VALUE $before ?? time::now() PERMISSIONS FULL;
DEFINE FIELD OVERWRITE default_collection ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD OVERWRITE email ON user TYPE string VALUE string::lowercase(string::trim($value)) ASSERT string::is_email($value) PERMISSIONS FULL;
DEFINE FIELD OVERWRITE last_login_at ON user TYPE option<datetime> VALUE $value ?? $before ?? NONE PERMISSIONS FULL;
DEFINE FIELD OVERWRITE request_count ON user TYPE int VALUE $value ?? $before ?? 0 PERMISSIONS FULL;
DEFINE FIELD OVERWRITE role ON user TYPE string VALUE string::lowercase($value ?? $before ?? 'default') ASSERT $value INSIDE ['default', 'admin'] PERMISSIONS FULL;
DEFINE FIELD OVERWRITE oauth_picture_url ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD OVERWRITE oauth_avatar_blob_id ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD OVERWRITE avatar_blob_id ON user TYPE option<string> PERMISSIONS FULL;

DEFINE INDEX OVERWRITE user_email_unique ON user FIELDS email UNIQUE CONCURRENTLY;

DEFINE EVENT OVERWRITE session_user_cascade ON user WHEN $event = 'DELETE' THEN (DELETE session WHERE user = $before.id);
DEFINE EVENT OVERWRITE http_request_audit_user_clear ON user WHEN $event = 'DELETE' THEN (UPDATE http_request_audit SET user = NONE WHERE user = $before.id);
