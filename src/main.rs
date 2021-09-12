//! # ハッシュ値計算ツール
use std::str::FromStr;
mod hash;
mod cli_arg_accepter;
#[cfg(target_os= "windows")]
mod context_menu;

fn main() {

    let (input_file_path, digest_algorithm, mode) = cli_arg_accepter::accept_cli_arg();

    match mode {
        cli_arg_accepter::Mode::Digest => digest(input_file_path, digest_algorithm),
        cli_arg_accepter::Mode::SetUp => setup(),
        cli_arg_accepter::Mode::CleanUp => cleanup(),
    }

    
    println!("Enterキーを押すと終了します");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
}


fn digest(input_file_path: Option<String>, digest_algorithm: cli_arg_accepter::DigestAlgorithm) {
    

    // 計測開始
    let pre_time = chrono::Local::now();

    // ファイルパス入力を取得する
    let input_file_path = match input_file_path {
        Some(s) => s,
        None => {println!("ファイルパスが入力されていませんでした。"); return}
    };
    // ファイルバッファリーダーの取得
    let input_path = match std::path::PathBuf::from_str(&input_file_path){
        Ok(p) => p,
        Err(_) => {println!("入力されたファイルパスが誤っています。"); return}
    };
    let input_file = match std::fs::File::open(input_path) {
        Ok(f) => f,
        Err(_) => {println!("ファイルにアクセスできませんでした。"); return}
    };
    let mut input_file_reader = std::io::BufReader::new(input_file);

    println!("ハッシュ値を計算しています。");

    let (file_size, hash_value)  = match digest_algorithm {
        cli_arg_accepter::DigestAlgorithm::Sha256 => {
            let hash_result = hash::sha256(&mut input_file_reader);

            let (file_size, hash_value) = match hash_result {
                Err(_e) => {println!("ファイルを読み込みできませんでした。"); std::process::exit(0)},
                Ok(result) => result,
            };
            (file_size, hash_value.to_vec())
        },
        cli_arg_accepter::DigestAlgorithm::Sha512 => {
            let hash_result = hash::sha512(&mut input_file_reader);
            let (file_size, hash_value) = match hash_result {
                Err(_e) => {println!("ファイルを読み込みできませんでした。"); std::process::exit(0)},
                Ok(result) => result,
            };
            (file_size, hash_value.to_vec())
        }
    };

    let post_time = chrono::Local::now();
    println!("ファイルサイズ: {}MB", file_size / 1_000_000);
    print!("ハッシュ値: ");
    for byte in hash_value { print!("{:x}, ", byte);}
    println!("");
    println!("所要時間: {:?}", post_time - pre_time);
}

fn setup() {
    if context_menu::set_to_context_menu().is_ok() {
        println!("コンテクストメニューに追加出来ました。")
    } else {
        println!("コンテクストメニューに追加できませんでした。");
    }
}

fn cleanup() {
    if context_menu::remove_from_context_menu().is_ok() {
        println!("コンテクストメニューから削除しました。");
    } else {
        println!("コンテクストメニューから削除できませんでした。");
    }
}