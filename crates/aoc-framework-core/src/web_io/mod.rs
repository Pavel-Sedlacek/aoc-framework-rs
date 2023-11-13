use std::fs::{create_dir, File, read_to_string, write};
use std::io::{Read, Write};
use std::path::Path;

pub struct AoCInputLoader {
  session: String
}

impl AoCInputLoader {
  pub fn new(session: String) -> AoCInputLoader {
    AoCInputLoader {
      session
    }
  }

  pub fn load(&self, clean: bool, day: u8, year: u16) -> String {
    if !clean {
      let cached = self.cached(day, year);
      if cached.is_some() { return cached.unwrap() };
    }
    self.download(day, year).expect("Failed to download input!")
  }
  
  pub fn submit(&self) -> Option<()> {
    None
  }

  fn download(&self, day: u8, year: u16) -> Option<String> {
    let cookie = format!("session={}", self.session);
    let url = "https://adventofcode.com/".parse::<reqwest::Url>().expect("Invalid URL");
    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = reqwest::blocking::Client::builder()
            .cookie_provider(std::sync::Arc::new(jar))
            .user_agent("")
            .build()
            .unwrap();

    let url = reqwest::Url::parse(&format!("https://adventofcode.com/{}/day/{}/input", year, day)).unwrap();

    let mut body = String::new();
    client.get(url).send().unwrap().read_to_string(&mut body).unwrap();

    self.cache(&body, day, year);
    Some(body)
  }

  fn cache(&self, input: &String, day: u8, year: u16) {
    self.scaffold_directory(year);
    let file = format!("/tmp/aoc/y{}/d{}.txt", year, day);
    let path = Path::new(&file);
    File::create(&path).unwrap();
    write(&path, input.as_bytes()).unwrap();
  }

  fn cached(&self, day: u8, year: u16) -> Option<String> {
    let file = format!("/tmp/aoc/y{}/d{}.txt", year, day);
    let path = Path::new(&file);
    if path.exists() {
      return Some(read_to_string(&path).unwrap());
    }
    None
  }

  fn scaffold_directory(&self, year: u16) {
    let aoc = Path::new("/tmp/aoc");
    if !aoc.exists() { create_dir(aoc).unwrap(); }
    let p = format!("/tmp/aoc/y{}", year);
    let year = Path::new(&p);
    if !year.exists() { create_dir(year).unwrap(); }
  }
}