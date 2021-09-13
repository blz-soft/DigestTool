//! # ハッシュ値計算ツール
use std::str::FromStr;
use log::debug;
mod hash;
mod cli_arg_accepter;
#[cfg(target_os= "windows")]
mod context_menu;

fn main() {
    // std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("アプリ開始");
    let (input_file_path, digest_algorithm, mode) = cli_arg_accepter::accept_cli_arg();

    match mode {
        cli_arg_accepter::Mode::Digest => digest(input_file_path, digest_algorithm),
        cli_arg_accepter::Mode::SetUp => setup(),
        cli_arg_accepter::Mode::CleanUp => cleanup(),
        cli_arg_accepter::Mode::GUI => gui(),
    }

    
    println!("Enterキーを押すと終了します");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    debug!("アプリ終了");
}


fn digest(input_file_path: Option<String>, digest_algorithm: cli_arg_accepter::DigestAlgorithm) {
    

    // 計測開始
    let pre_time = chrono::Local::now();

    // ファイルパス入力を取得する
    let input_file_path = match input_file_path {
        Some(s) => s,
        None => {
            debug!("ファイルパスが入力されていませんでした。");
            println!("ファイルパスが入力されていませんでした。"); 
            return
        }
    };
    // ファイルバッファリーダーの取得
    let input_path = match std::path::PathBuf::from_str(&input_file_path){
        Ok(p) => p,
        Err(_) => {
            debug!("入力されたファイルパスが誤っています。");
            println!("入力されたファイルパスが誤っています。");
            return
        }
    };
    let input_file = match std::fs::File::open(input_path) {
        Ok(f) => f,
        Err(_) => {
            debug!("ファイルにアクセスできませんでした。ここか");
            println!("ファイルにアクセスできませんでした。");
            return
        }
    };
    let input_file_size = match input_file.metadata() {
        Ok(meta) => meta.len(),
        Err(_) => {
            debug!("ファイルサイズを取得できませんでした。");
            println!("ファイルサイズを取得できませんでした。");
            return
        }
    };
    let mut input_file_reader = std::io::BufReader::new(input_file);

    // プログレスバーのセットアップ
    let progress_bar_style = indicatif::ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})");
    let progress_bar = indicatif::ProgressBar::new(input_file_size);
    progress_bar.set_style(progress_bar_style);
    // 1秒に4回プログレスバーを更新すると、少しパフォーマンスに影響出てきそう(2.5GHz 4core)
    progress_bar.set_draw_rate(4);
    
    debug!("ハッシュ値計算開始");
    println!("ハッシュ値を計算しています。");

    let (file_size, hash_value)  = match digest_algorithm {
        cli_arg_accepter::DigestAlgorithm::Sha2_256 => {
            debug!("ハッシュアルゴリズム: Sha2 256");
            println!("ハッシュアルゴリズム: Sha2 256");
            let hash_result = hash::sha2_256(&mut input_file_reader, progress_bar);
            let (file_size, hash_value) = match hash_result {
                Err(e) => {
                    debug!("{:?}", e);
                    println!("ファイルを読み込みできませんでした。");
                    std::process::exit(0)
                },
                Ok(result) => result,
            };
            (file_size, hash_value.to_vec())
        },
        cli_arg_accepter::DigestAlgorithm::Sha2_512 => {
            println!("ハッシュアルゴリズム: Sha2 512");
            let hash_result = hash::sha2_512(&mut input_file_reader, progress_bar);
            let (file_size, hash_value) = match hash_result {
                Err(e) => {
                    debug!("{:?}", e);
                    println!("ファイルを読み込みできませんでした。");
                    std::process::exit(0)
                },
                Ok(result) => result,
            };
            (file_size, hash_value.to_vec())
        },
        cli_arg_accepter::DigestAlgorithm::Sha3_256 => {
            println!("ハッシュアルゴリズム: Sha3 256");
            let hash_result = hash::sha3_256(&mut input_file_reader, progress_bar);
            let (file_size, hash_value) = match hash_result {
                Err(e) => {
                    debug!("{:?}", e);
                    println!("ファイルを読み込みできませんでした。");
                    std::process::exit(0)
                },
                Ok(result) => result,
            };
            (file_size, hash_value.to_vec())
        },
        cli_arg_accepter::DigestAlgorithm::Sha3_512 => {
            println!("ハッシュアルゴリズム: Sha3 512");
            let hash_result = hash::sha3_512(&mut input_file_reader, progress_bar);
            let (file_size, hash_value) = match hash_result {
                Err(e) => {
                    debug!("{:?}", e);
                    println!("ファイルを読み込みできませんでした。");
                    std::process::exit(0)
                },
                Ok(result) => result,
            };
            (file_size, hash_value.to_vec())
        },
    };

    let post_time = chrono::Local::now();
    println!("ファイルサイズ: {}MB", file_size / 1_000_000);
    print!("ハッシュ値: [");
    for i in 0..hash_value.len() { 
        print!("{:x}", hash_value[i]);
        if i != hash_value.len() { print!(", "); }
    }
    println!("]");
    println!("所要時間: {:?}", post_time - pre_time);
}

fn setup() {
    println!("コンテクストメニューにコマンドを追加しています。");
    if context_menu::set_to_context_menu().is_ok() {
        println!("コンテクストメニューに追加出来ました。")
    } else {
        println!("コンテクストメニューに追加できませんでした。");
    }
}

fn cleanup() {
    println!("コンテクストメニューにコマンドを削除しています。");
    if context_menu::remove_from_context_menu().is_ok() {
        println!("コンテクストメニューから削除しました。");
    } else {
        println!("コンテクストメニューから削除できませんでした。");
    }
}

fn gui() {
    debug!("GUIモードで起動しました。");
    let html_content = "<html><body><h1>ハッシュ値計算ツールです。</h1><p>GUIは開発中です。</p></body></html>";
	
    web_view::builder()
        .title("My Project")
        .content(web_view::Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}