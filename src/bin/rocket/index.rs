use rocket::response::NamedFile;
use std::path::Path;

//Index.html
#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/index.html")).ok()
}