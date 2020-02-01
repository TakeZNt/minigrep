use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::boxed::Box;

fn main() {
    let args : Vec<String> = env::args().collect();
    //println!("{:?}", args);

    // 戻り値がある場合はunwrap_or_elseを使う
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("引数の解析時に問題が発生しました。{}", err);
        process::exit(1);
    });

    // 戻り値がないので if let Err(e)を使う
    if let Err(e) = run(config){
      println!("アプリケーションでエラーが発生しました。{}", e);
      process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn = dynamic : implemsnts Errorの意味
  let contents = fs::read_to_string(config.filename)?;

  println!("テキストは : \n{}", contents);

  Ok(()) // void?
}


struct Config {
  query : String,
  filename : String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("usage: minigrep query filename");
    }
    Ok(Config{query : args[1].clone(), filename : args[2].clone()}) // 所有権の移転を回避するため、やむなくcloneしている
  }
}
