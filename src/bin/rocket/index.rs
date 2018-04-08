use rocket::response::NamedFile;
use std::path::Path;

// index()
//
// serves the landing page
#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/index.html")).ok()
}

// get_file()
//
// return a specified file from the statics folder
// used to return CSS/JS file required by the frontend
#[get("/<filename>")]
pub fn get_file(filename: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("statics").join(filename)).ok()
}
