#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate UniHack;
extern crate rocket;
extern crate rand;

mod index;
mod quiz;
mod assets;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index::index,assets::get_file])
}

fn main() {
    rocket().launch();
}
