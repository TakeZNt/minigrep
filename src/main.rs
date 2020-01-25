use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 3 {
      panic!("usage: minigrep query filename");
    }

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.filename)
      .expect("ファイルを開くのに失敗しました。");
    println!("テキストは : \n{}", contents);
}

struct Config {
  query : String,
  filename : String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    Config{query : args[1].clone(), filename : args[2].clone()}
  }
}
