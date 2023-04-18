use std::path::{Path, PathBuf};

use rocket::http::ContentType;

pub fn get_path(file_name: String) -> PathBuf {
    let path = Path::new(&file_name);
    let mut path = path.join("/home/riri/dev/rust/webpConvert/input/");
    path.push(file_name);
    return path.to_path_buf();
}

pub fn get_extension(content_type: &ContentType) -> String {
    ".".to_owned() + content_type.extension().unwrap().as_str()
}
