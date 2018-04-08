use rocket::response::NamedFile;
use std::path::Path;

// quiz_page()
//
// serves the quiz page
#[get("/quiz")]
pub fn quiz_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/quiz.html")).ok()
}
