use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 3 {
      panic!("usage: minigrep query filename");
    }

    let query = &args[1];
    let filename = &args[2];
    println!("queryは{}です", query);
    println!("filenameは{}です", filename);

    let contents = fs::read_to_string(filename)
      .expect("ファイルを開くのに失敗しました。");
    println!("テキストは : \n{}", contents);
}


