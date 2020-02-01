use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    //println!("{:?}", args);

    // 戻り値がある場合はunwrap_or_elseを使う
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("引数の解析時に問題が発生しました。{}", err);
        process::exit(1);
    });

    // 戻り値がないので if let Err(e)を使う
    if let Err(e) = minigrep::run(config){
      println!("アプリケーションでエラーが発生しました。{}", e);
      process::exit(1);
    }
}
