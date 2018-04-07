use rocket::response::NamedFile;
use rocket::response::content::Html;
use std::path::Path;

//Index.html
//improper error handling for now
#[get("/")]
pub fn index() -> Html<NamedFile> {
    let index_page = NamedFile::open(Path::new("statics/index.html")).expect("404");
    Html(index_page)
}