use std::fs::{create_dir, File, read_to_string, write};
use std::io::{Read, Write};
use std::path::Path;

pub struct AoCInputLoader {
    session: String,
    url: String,
    year: isize,
    day: isize
}

impl AoCInputLoader {
    pub fn load(&self, clean: bool) -> String {
        if !clean {
            let cached = self.cached();
            if cached.is_some() { return cached.unwrap() };
        }
        self.download().expect("Failed to download input!")
    }

    fn cached(&self) -> Option<String> {
        let file = format!("/tmp/aoc/y{}/d{}.txt", self.year, self.day);
        let path = Path::new(&file);
        if path.exists() {
            return Some(read_to_string(&path).unwrap());
        }
        None
    }

    fn download(&self) -> Option<String> {
        let cookie = format!("session={}", "53616c7465645f5f2db288a81f635fcf11dbdd8b601493a8a399af4792f38c78efe9371f148a2bcd90a868ba506d4f7efd4bdde761933ef687d73d793b294465");
        let url = "https://adventofcode.com/".parse::<reqwest::Url>().expect("Invalid URL");
        let jar = reqwest::cookie::Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = reqwest::blocking::Client::builder()
            .cookie_provider(std::sync::Arc::new(jar))
            .user_agent("")
            .build()
            .unwrap();

        let url = reqwest::Url::parse(&format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day)).unwrap();

        let mut body = String::new();
        client.get(url).send().unwrap().read_to_string(&mut body).unwrap();

        self.cache(&body);
        Some(body)
    }

    fn cache(&self, input: &String) {
        self.scaffold_directory();
        let file = format!("/tmp/aoc/y{}/d{}.txt", self.year, self.day);
        let path = Path::new(&file);
        File::create(&path).unwrap();
        write(&path, input.as_bytes());
    }

    fn scaffold_directory(&self) {
        let aoc = Path::new("/tmp/aoc");
        if !aoc.exists() { create_dir(aoc); }
        let p = format!("/tmp/aoc/y{}", &self.year);
        let year = Path::new(&p);
        if !year.exists() { create_dir(year); }
    }
}

pub fn aoc_input_loader(year: isize, day: isize) -> AoCInputLoader {
    AoCInputLoader {
        session: "".to_string(),
        url: "".to_string(),
        year,
        day
    }
}