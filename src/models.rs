use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};
use crate::schema::{rustaceans, crates};

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Rustacean {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub name: String,
  pub email: String,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime, 
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
  pub name: String,
  pub email: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Crate {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub rustacean_id: i32,
  pub code: String,
  pub name: String,
  pub version: String,
  pub description: Option<String>,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime, 
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name=crates)]
pub struct NewCrate {
  pub rustacean_id: i32,
  pub code: String,
  pub name: String,
  pub version: String,
  pub description: Option<String>,
}