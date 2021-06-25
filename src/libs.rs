use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use exif::{Error, Exif};

pub struct Config {
    pub picture_path: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let picture_path = args[1].clone();

        Ok(Config { picture_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let pic_tmp: String = config.picture_path.to_string(); // Make a copy

    let file = File::open(&pic_tmp)?; // Use the reference to picture_path and does not take ownership
    // file:///Users/jhujol/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch04-02-references-and-borrowing.html
    // We call having references as function parameters borrowing

    print_bytes(&pic_tmp); // Same here, we are borrowing the reference of the object pointed to it

    let exif = exif::Reader::new()
        .read_from_container(&mut BufReader::new(&file))?;

    for f in exif.fields() {
        println!("  {}/{}: {}",
                 f.ifd_num.index(), f.tag,
                 f.display_value().with_unit(&exif));
        println!("      {:?}", f.value);
    }

    Ok(())

    //
    // let pic_path = "/Users/jhujol/Desktop/ford_taunus_tc_1972.jpeg";
    // toto(pic_path);
    //
    // let pic_path2: &str = "/Users/jhujol/Desktop/vaccin-arn.jpeg";
    // toto(pic_path2);
    //
    // Ok(())
}

fn print_bytes(picture_path: &String) -> Result<(), Box<dyn error::Error>> {
    let mut file = File::open(picture_path)?;

    let mut buffer = [0; 100];
    let n = file.read(&mut buffer[..])?;

    println!("The bytes: {:?}", &buffer[..n]);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn internal() {
        let a: i8 = 2;
        let b: i8 = 4;
        assert_eq!(6, a + b, "we are testing addition with {} and {}", a, b);
    }
}

// impl From<exif::Error> for exif::Error{
//     fn from(_: Error) -> Self {
//         todo!()
//     }
// }



/*fn toto(pic_path: &str) -> std::io::Result<()> {
    println!("{:?}", pic_path);

    // Use EXIF library
    let mut file = File::open(pic_path).unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let mut buffer = [0; 100];
    let n = file.read(&mut buffer[..])?;

    println!("The bytes: {:?}", &buffer[..n]);


    let mut line = String::new();
    let mut bufreader = std::io::BufReader::new(&file);
    let len = bufreader.read_line(&mut line).unwrap_or_else(|error| {
        panic!("Problem reading lines from file: {:?}", error)
    });
    println!("First line is {} bytes long", len);


    let exifreader = exif::Reader::new();
    let exifresult: Result<Exif, Error> = exifreader.read_from_container(&mut bufreader);
    let exif: Exif = exifresult.unwrap_or_else(|error| {
        panic!("Problem reading EXIF from file: {:?}", error)
    });

    for f in exif.fields() {
        println!("{} {} {}",
                 f.tag, f.ifd_num, f.display_value().with_unit(&exif));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn internal() {
        let a: i8 = 2;
        let b: i8 = 4;
        assert_eq!(6, a + b, "we are testing addition with {} and {}", a, b);
    }
}
*/
