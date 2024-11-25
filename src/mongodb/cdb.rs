use axum::http::StatusCode;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    results::InsertOneResult,
    Database,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::schemas::{
    merchants::MERCHANTS_COLLECTION_NAME, products::PRODUCTS_COLLECTION_NAME,
    users::USERS_COLLECTION_NAME,
};
use crate::{routes::auth::User, utilities::error::internal_error};

#[derive(Debug, Clone)]
struct Actions<'a> {
    database: Database,
    collection_name: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Generic {
    created_at: SystemTime,
    created_by: ObjectId,
    updated_at: SystemTime,
    updated_by: ObjectId,
    deleted_at: SystemTime,
    deleted_by: ObjectId,
}

impl Actions<'_> {
    async fn find_one(self, mut doc: Document) -> Result<Option<Generic>, (StatusCode, String)> {
        doc.insert("deletedAt", doc! { "$ne": null });
        self.database
            .collection::<Generic>(self.collection_name)
            .find_one(doc)
            .await
            .map_err(internal_error)
    }

    fn find() {}

    async fn insert_one(
        self,
        mut input: Generic,
        user: User,
    ) -> Result<InsertOneResult, (StatusCode, String)> {
        input.created_at = SystemTime::now();
        input.created_by = user.id;
        self.database
            .collection::<Generic>(self.collection_name)
            .insert_one(input)
            .await
            .map_err(internal_error)
    }

    fn update_one() {}

    fn delete_one() {}
}

#[derive(Debug, Clone)]
pub struct Cdb<'a> {
    users: Actions<'a>,
    products: Actions<'a>,
    merchants: Actions<'a>,
}

pub fn make_cdb(db: Database) -> Cdb<'static> {
    let cdb = Cdb {
        users: Actions {
            database: db.clone(),
            collection_name: USERS_COLLECTION_NAME,
        },
        products: Actions {
            database: db.clone(),
            collection_name: PRODUCTS_COLLECTION_NAME,
        },
        merchants: Actions {
            database: db,
            collection_name: MERCHANTS_COLLECTION_NAME,
        },
    };
    cdb
}
