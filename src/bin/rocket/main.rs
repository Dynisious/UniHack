#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate UniHack;
extern crate rocket;

mod index;
mod quiz;
mod assets;
mod results;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index::index,assets::get_file,quiz::quiz_page,results::get_neural_results, results::return_results])
}

fn main() {
    rocket().launch();
}
