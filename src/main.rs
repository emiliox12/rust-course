use diesel::prelude::*;
mod schema;
mod models;
mod repositories;
mod routes;
#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);
#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/rustaceans", rocket::routes![
        routes::rustaceans::get_rustaceans,
        routes::rustaceans::view_rustaceans,
        routes::rustaceans::create_rustacean,
        routes::rustaceans::update_rustacean,
        routes::rustaceans::delete_rustacean,
    ])
    .mount("/crate", rocket::routes![
        routes::crates::get_crates,
        routes::crates::view_crates,
        routes::crates::create_crate,
        routes::crates::update_crate,
        routes::crates::delete_crate,
    ])
    .attach(DbConn::fairing())
    .launch()
    .await;
}
