#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate UniHack;
extern crate rocket;

mod index;
mod quiz;
mod assets;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index::index,assets::get_file,quiz::quiz_page])
}

fn main() {
    rocket().launch();
}
