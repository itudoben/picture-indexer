extern crate chrono;

use std::error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::time::SystemTime;

use chrono::{DateTime, Utc};

use crate::config::Config;
use crate::picture_utils::PictureMetadata;

pub mod config;
mod picture_utils;

pub fn run(config: Config) -> std::io::Result<()> {
    let pic_tmp: String = config.picture_path.to_string(); // Make a copy

    // Store picture metadata
    let mut v: Vec<PictureMetadata> = Vec::new();

    let file = File::open(&pic_tmp).unwrap(); // Use the reference to picture_path and does not take ownership
    // file:///Users/jhujol/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch04-02-references-and-borrowing.html
    // We call having references as function parameters borrowing

    let this_time = fs::metadata(&pic_tmp).unwrap().created().unwrap();
    let datetime: DateTime<Utc> = SystemTime::into(this_time);

    let pic_metadata = PictureMetadata::new(config.picture_path.to_string(), datetime).unwrap();
    v.push(pic_metadata);

    match print_bytes(&pic_tmp) {  // Same here, we are borrowing the reference of the object pointed to it
        Ok(()) => print!("Could print the result"),
        Err(error) => print!("Could not print the result {:?}", error),
    }

    // Print the EXIF data if they exist.
    match exif::Reader::new()
        .read_from_container(&mut BufReader::new(&file)) {
        Ok(exif) =>
            for f in exif.fields() {
                println!("  {}/{}: {}",
                         f.ifd_num.index(), f.tag,
                         f.display_value().with_unit(&exif));
                println!("      {:?}", f.value);
            },
        Err(_error) =>
            println!("Could not read EXIF data from file {}", &pic_tmp),
    };

    let file = File::create(format!("/Users/jhujol/Projects/rust/picture-indexer/{}", "pics.csv")).unwrap();

    return save_results_on_file(&v, file);
}

fn save_results_on_file(v: &Vec<PictureMetadata>, mut file: File) -> std::io::Result<()> {
    for item in v {
        let mut ss: String = String::from(item);
        ss.push('\n');
        println!("\n***** {}", &ss);
        match file.write(ss.as_bytes()) {
            Ok(_some_bytes) => print!("Wrote the string to it"),
            Err(_error) => println!("Could not get the bytes off of a serialized PictureMetadata object.")
        };
    }

    Ok(())
}

fn print_bytes(picture_path: &String) -> Result<(), Box<dyn error::Error>> {
    let mut file = File::open(picture_path)?;
    println!("{}", picture_path);

    let mut buffer = [0; 100];
    let n = file.read(&mut buffer[..])?; // borrowing the reference of buffer

    println!("The bytes: {:?}", &buffer[..n]); // borrowing the reference of buffer

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use chrono::{DateTime, Utc};

    use crate::picture_utils::PictureMetadata;

    #[test]
    fn first_test() {
        let a: i8 = 2;
        let b: i8 = 4;
        assert_eq!(6, a + b, "we are testing addition of {} and {}", a, b);
    }

    #[test]
    fn test_picture_metadata() {
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = SystemTime::into(now);
        let a_path = String::from("a_path");
        let pm = PictureMetadata::new(a_path, datetime).unwrap();

        assert_eq!("a_path", pm.file_path, "This tests the path is equal to {}", "a_path");
    }
}
