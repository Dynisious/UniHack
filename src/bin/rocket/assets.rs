use rocket::response::NamedFile;
use std::path::Path;

#[get("/assets/<filename>")]
pub fn get_file(filename: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/assets/").join(filename)).ok()
}
