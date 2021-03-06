use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 戻り値がある場合はunwrap_or_elseを使う
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("引数の解析時に問題が発生しました。{}", err);
        process::exit(1);
    });

    // 戻り値がないので if let Err(e)を使う
    if let Err(e) = minigrep::run(config){
      eprintln!("アプリケーションでエラーが発生しました。{}", e);
      process::exit(1);
    }
}
