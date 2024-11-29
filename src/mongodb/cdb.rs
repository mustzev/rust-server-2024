use axum::http::StatusCode;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Document},
    results::{InsertOneResult, UpdateResult},
    Database,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;

use super::schemas::{
    merchants::{Merchant, MERCHANTS_COLLECTION_NAME},
    products::{Product, PRODUCTS_COLLECTION_NAME},
    users::{User, USERS_COLLECTION_NAME},
};
use crate::utilities::error::internal_error;

#[derive(Debug)]
pub struct Actions<T> {
    database: Database,
    collection_name: String,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for Actions<T>
where
    T: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            collection_name: self.collection_name.clone(),
            _phantom: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Generic {
    created_at: DateTime,
    created_by: ObjectId,
    updated_at: DateTime,
    updated_by: ObjectId,
    is_deleted: bool,
    deleted_at: DateTime,
    deleted_by: ObjectId,
}

impl<T: DeserializeOwned + Send + Sync> Actions<T> {
    pub async fn find_one(self, mut doc: Document) -> Result<Option<T>, (StatusCode, String)> {
        doc.insert("isDeleted", doc! { "$ne": true });
        self.database
            .collection::<T>(&self.collection_name)
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
        query.insert("isDeleted", doc! { "$ne": true });
        update.insert("$set.updatedAt", DateTime::now());
        update.insert("$set.updatedBy", user.id);
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
        query.insert("isDeleted", doc! { "$ne": true });
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
    pub users: Actions<User>,
    pub products: Actions<Product>,
    pub merchants: Actions<Merchant>,
}

pub fn make_cdb(db: Database) -> Cdb {
    let cdb = Cdb {
        users: Actions::<User> {
            database: db.clone(),
            collection_name: USERS_COLLECTION_NAME.to_string(),
            _phantom: PhantomData,
        },
        products: Actions::<Product> {
            database: db.clone(),
            collection_name: PRODUCTS_COLLECTION_NAME.to_string(),
            _phantom: PhantomData,
        },
        merchants: Actions::<Merchant> {
            database: db,
            collection_name: MERCHANTS_COLLECTION_NAME.to_string(),
            _phantom: PhantomData,
        },
    };
    cdb
}
