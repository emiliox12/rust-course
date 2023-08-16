use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, json, Value};
use rocket::http::Status;

use crate::{models::{Crate, NewCrate}, DbConn};
use crate::repositories::CrateRepository;

#[rocket::get("/")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    CrateRepository::find_multiple(c, 100)
    .map(|crates| json!(crates))
    .map_err(|_: diesel::result::Error| Custom(Status::InternalServerError,json!("Error")))
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_crates(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    CrateRepository::find(c, id)
    .map(|a_crate| json!(a_crate))
    .map_err(|e: diesel::result::Error| {
      println!("{}", e);
      Custom(Status::InternalServerError,json!("ERROR"))
  })
  }).await
}

#[rocket::post("/", format="json", data="<new_crate>")]
pub async fn create_crate(new_crate: Json<NewCrate>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    CrateRepository::create(c, new_crate.into_inner())
    .map(|a_crate| Custom(Status::Created, json!(a_crate)))
    .map_err(|e: diesel::result::Error| {
      println!("{}", e);
      Custom(Status::InternalServerError,json!("ERROR"))
  })
  }).await
}

#[rocket::put("/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(id: i32, a_crate: Json<Crate>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    CrateRepository::update(c, id, a_crate.into_inner())
    .map(|a_crate| Custom(Status::Created, json!(a_crate)))
    .map_err(|_: diesel::result::Error| Custom(Status::InternalServerError,json!("Error")))
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_crate(id: i32, db: DbConn)-> Result<NoContent, Custom<Value>>  {
  db.run(move |c| {
    CrateRepository::delete(c, id)
    .map(|_| NoContent)
    .map_err(|_: diesel::result::Error| Custom(Status::InternalServerError,json!("Error")))
  }).await
}