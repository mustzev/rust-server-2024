use axum::http::StatusCode;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Document},
    results::{InsertOneResult, UpdateResult},
    Database,
};
use serde::{Deserialize, Serialize};

use super::schemas::{
    merchants::MERCHANTS_COLLECTION_NAME, products::PRODUCTS_COLLECTION_NAME,
    users::USERS_COLLECTION_NAME,
};
use crate::{routes::auth::User, utilities::error::internal_error};

#[derive(Debug, Clone)]
pub struct Actions {
    database: Database,
    collection_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Generic {
    created_at: DateTime,
    created_by: ObjectId,
    updated_at: DateTime,
    updated_by: ObjectId,
    is_deleted: bool,
    deleted_at: DateTime,
    deleted_by: ObjectId,
}

impl Actions {
    pub async fn find_one(
        self,
        mut doc: Document,
    ) -> Result<Option<Generic>, (StatusCode, String)> {
        doc.insert(
            "isDeleted",
            doc! { "$or": [
                doc! { "$exists": false },
                doc! { "$eq": false },
            ] },
        );
        self.database
            .collection::<Generic>(&self.collection_name)
            .find_one(doc)
            .await
            .map_err(internal_error)
    }

    pub fn find() {}

    pub async fn insert_one(
        self,
        mut doc: Document,
        user: User,
    ) -> Result<InsertOneResult, (StatusCode, String)> {
        doc.insert("createdAt", DateTime::now());
        doc.insert("createdBy", user.id);
        self.database
            .collection(&self.collection_name)
            .insert_one(doc)
            .await
            .map_err(internal_error)
    }

    pub async fn update_one(
        self,
        mut query: Document,
        mut update: Document,
        user: User,
    ) -> Result<UpdateResult, (StatusCode, String)> {
        query.insert(
            "isDeleted",
            doc! { "$or": [
                doc! { "$exists": false },
                doc! { "$eq": false },
            ] },
        );
        update.insert("updatedAt", DateTime::now());
        update.insert("updatedBy", user.id);
        self.database
            .collection::<Generic>(&self.collection_name)
            .update_one(query, update)
            .await
            .map_err(internal_error)
    }

    pub async fn delete_one(
        self,
        mut query: Document,
        user: User,
    ) -> Result<UpdateResult, (StatusCode, String)> {
        query.insert(
            "isDeleted",
            doc! { "$or": [
                doc! { "$exists": false },
                doc! { "$eq": false },
            ] },
        );
        let update = doc! {
            "isDeleted": true,
            "deletedAt": DateTime::now(),
            "deletedBy": user.id
        };
        self.database
            .collection::<Generic>(&self.collection_name)
            .update_one(query, update)
            .await
            .map_err(internal_error)
    }
}

#[derive(Debug, Clone)]
pub struct Cdb {
    pub users: Actions,
    pub products: Actions,
    pub merchants: Actions,
}

pub fn make_cdb(db: Database) -> Cdb {
    let cdb = Cdb {
        users: Actions {
            database: db.clone(),
            collection_name: USERS_COLLECTION_NAME.to_string(),
        },
        products: Actions {
            database: db.clone(),
            collection_name: PRODUCTS_COLLECTION_NAME.to_string(),
        },
        merchants: Actions {
            database: db,
            collection_name: MERCHANTS_COLLECTION_NAME.to_string(),
        },
    };
    cdb
}
