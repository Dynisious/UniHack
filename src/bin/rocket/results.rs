use rocket::response::NamedFile;
use std::path::Path;


#[get("/results/<a>/<b>/<c>/<d>/<e>")]
pub fn get_neural_results(a:usize,b:usize,c:usize,d:usize,e:usize) -> String {
    let input = [a,b,c,d,e];
    let output = super::UniHack::neural_net(&input);
    format!("{},{},{}",output[0],output[1],output[2])
}

#[get("/itinerary")]
pub fn return_results() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/itinerary.html")).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gnr_test() {
        assert_eq!(get_neural_results(3, 5, 7, 9, 11), "0,0,0");
    }
}