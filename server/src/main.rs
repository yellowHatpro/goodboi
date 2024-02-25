#[macro_use] extern crate rocket;
mod lib;
mod pg_pool;
#[get("/")]
fn index() -> &'static str {
    "Hello,!"
}



#[launch]
fn rocket() -> _ {
    lib::main();
    rocket::build().mount("/", routes![index])
}