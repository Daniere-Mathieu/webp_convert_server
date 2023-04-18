#[macro_use]
extern crate rocket;

mod path_helper;
mod webp_convert;

use rocket::{
    form::{Form, FromForm},
    fs::TempFile,
};

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/upload", data = "<upload>")]
async fn upload_form(mut upload: Form<Upload<'_>>) {
    let save_path = "/home/riri/dev/rust/webpConvert/input/".to_owned();

    let file_name = upload.file.name().unwrap();
    let file_name_input = file_name.clone();
    let file_name_output = file_name_input.clone().to_owned();

    let content_type = upload.file.content_type().unwrap();

    let extension = path_helper::get_extension(&content_type);

    let file_path = save_path + file_name + extension.as_str();

    let extension = path_helper::get_extension(&content_type);
    let file_path_buf = path_helper::get_path(file_name_input.to_owned() + extension.as_str());

    let result = upload.file.persist_to(file_path).await;

    let result = webp_convert::convert(file_path_buf, file_name_output);
    println!("result: {:?}", result);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![upload_form])
}
