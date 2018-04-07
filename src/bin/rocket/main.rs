<<<<<<< HEAD
=======
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate UniHack;
extern crate rocket;

mod index;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index::index])
}

fn main() {
    rocket().launch();
}
>>>>>>> master
