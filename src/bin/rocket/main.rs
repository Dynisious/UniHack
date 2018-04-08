#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate UniHack;
extern crate rocket;

mod assets;
mod index;
mod quiz;
mod results;
mod training;

// rocket()
//
// instantiates a server instance,
// consisting of possible server routes
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/",
        routes![
            index::index,
            index::get_file,
            assets::get_asset,
            quiz::quiz_page,
            results::get_neural_results,
            results::return_results,
            training::training_mode,
            training::neural_training,
            training::neural_training_completion,
        ],
    )
}


// main
//
// launches the app server
fn main() {
    rocket().launch();
}
