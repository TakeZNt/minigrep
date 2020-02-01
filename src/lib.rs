use std::fs;
use std::error::Error;

pub struct Config {
    pub query : String,
    pub filename : String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
        return Err("usage: minigrep query filename");
        }
        Ok(Config{query : args[1].clone(), filename : args[2].clone()}) // 所有権の移転を回避するため、やむなくcloneしている
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn = dynamic : implemsnts Errorの意味
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    

    Ok(()) // void?
}

// 戻り値のstrはcontentsとlifetimeが同じですよ
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );
    }
}
