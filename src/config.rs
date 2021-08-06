pub struct Config {
    pub picture_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        Ok(Config { picture_path: args[1].clone() })
    }
}
