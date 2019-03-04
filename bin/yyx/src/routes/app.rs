use rocket::http::{ContentType, Status};
use rocket::Response;
use std::io::Cursor;
use std::path::PathBuf;

use yyx_resources::YyxAsset;

#[get("/")]
pub fn index<'a>() -> Response<'a> {
  let resolve =
    YyxAsset::get("index.html").map(|data| (ContentType::from_extension("html").unwrap(), data));
  if let Some((content_type, data)) = resolve {
    Response::build()
      .header(content_type)
      .sized_body(Cursor::new(data))
      .finalize()
  } else {
    Response::build().status(Status::NotFound).finalize()
  }
}

#[get("/<file..>")]
pub fn static_file<'a>(file: PathBuf) -> Response<'a> {
  if file.extension().is_none() {
    return index();
  }

  let resolve = YyxAsset::get(&file.to_string_lossy()).map(|data| {
    (
      file
        .extension()
        .and_then(|ext| ContentType::from_extension(&ext.to_string_lossy()))
        .unwrap_or(ContentType::Binary),
      data,
    )
  });
  if let Some((content_type, data)) = resolve {
    Response::build()
      .header(content_type)
      .sized_body(Cursor::new(data))
      .finalize()
  } else {
    Response::build().status(Status::NotFound).finalize()
  }
}