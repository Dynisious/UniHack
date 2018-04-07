use rocket::response::NamedFile;
use std::path::Path;

#[get("/assets/<filename>")]
pub fn get_file(filename: String) -> NamedFile {
    NamedFile::open(Path::new("assets/").join(filename)).expect("404")
}