#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(helloWorld)] mod tests;

#[get("/")]
fn hello() -> &'static str{
    "Goodbye World! \nI am o n rocketttt"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}