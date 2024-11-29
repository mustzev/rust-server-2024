use mongodb::bson::{
    oid::ObjectId,
    serde_helpers::{serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string},
    DateTime,
};
use serde::Serializer;

pub fn serialize_bson_datetime_as_rfc3339_string_option<S: Serializer>(
    val: &Option<DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(val) => serialize_bson_datetime_as_rfc3339_string(val, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_object_id_as_hex_string_option<S: Serializer>(
    val: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(val) => serialize_object_id_as_hex_string(val, serializer),
        None => serializer.serialize_none(),
    }
}
