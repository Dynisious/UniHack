use rocket::response::NamedFile;
use std::path::Path;

//Index.html
#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/index.html")).ok()
}

#[get("/<filename>")]
pub fn get_file(filename: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/assets/").join(filename)).ok()
}