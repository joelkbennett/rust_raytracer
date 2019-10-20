use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Image<'a> {
  pub filename: &'a str,
  pub contents: &'a mut String,
  pub path: Path,
}

// impl Image {
  // create new instance
  // pub fn new(filename: &str) {
  //   Image{
  //     filename,
  //     contents: "",
  //     path: Path::new(filename),
  //   }
  // }

  // add contents...

  // get path.display
// }