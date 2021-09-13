//! # ハッシュ値計算ツール
use std::str::FromStr;
use log::debug;
mod hash;
mod cli_arg_accepter;
#[cfg(target_os= "windows")]
mod context_menu;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("アプリ開始");
    let (input_file_path, digest_algorithm, mode) = cli_arg_accepter::accept_cli_arg();

    match mode {
        cli_arg_accepter::Mode::Digest => digest(input_file_path, digest_algorithm),
        cli_arg_accepter::Mode::SetUp => setup(),
        cli_arg_accepter::Mode::CleanUp => cleanup(),
        cli_arg_accepter::Mode::Gui => gui(),
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

extern crate native_windows_gui as nwg;
use std::rc::Rc;
fn gui() {
    debug!("GUIモードで起動しました。");


    nwg::init().unwrap_or_else(|e| {
        debug!("Failed to init Native Windows GUI");
        debug!("{:?}", e);
        std::process::exit(0);
    });

    nwg::Font::set_global_family("Segoe UI").unwrap_or_else(|e| {
        debug!("Failed to set default font");
        debug!("{:?}", e);
        std::process::exit(0);
    });

    // ウェジットのオブジェクトを作成
    let mut window = Default::default();
    let mut button_set_context_menu = Default::default();
    let mut button_remove_context_menu = Default::default();
    let layout = Default::default();

    // ウェジットのオブジェクトのスタイルを変更する
    nwg::Window::builder()
        .size((600, 115))
        .position((600, 300))
        .title("Digest Tool")
        .build(&mut window)
        .unwrap();

    nwg::Button::builder()
        .text("右クリックメニューにDigtestToolを追加します。")
        .parent(&window)
        .build(&mut button_set_context_menu)
        .unwrap();
    
    nwg::Button::builder()
        .text("右クリックメニューからDigest Toolを削除します。")
        .parent(&window)
        .build(&mut button_remove_context_menu)
        .unwrap();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child_item(nwg::GridLayoutItem::new(&button_set_context_menu, 0, 0, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&button_remove_context_menu, 0, 1, 1, 1))
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    // イベントをバインドさせてる。handlerイベントハンドラー(イベントを受け取ってくれるオブジェクト)
    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        #[allow(clippy::single_match)]
        match evt {
            // ボタンが押されたイベントすべてを受け取る?
            E::OnButtonClick => {
                // コントロールハンドラー? ControlHandleっていうのはウェジットとかの部品っぽい
                if handle == button_set_context_menu {
                    debug!("右クリックメニューに追加します");
                    if context_menu::set_to_context_menu().is_ok() {
                        nwg::modal_info_message(&events_window.handle, "Digest Tool", "右クリックメニューに追加しました。");
                    } else {
                        nwg::modal_info_message(&events_window.handle, "Digest Tool", "右クリックメニューに追加できませんでした。");
                    }
                    // ファイルダイアログ
                } else if handle == button_remove_context_menu{
                    debug!("右クリックメニューにから削除します");
                    if context_menu::remove_from_context_menu().is_ok() {
                        nwg::modal_info_message(&events_window.handle, "Digest Tool", "右クリックメニューから削除しました。");
                    } else {
                        nwg::modal_info_message(&events_window.handle, "Digest Tool", "右クリックメニューから削除できませんでした。");
                    }
                }
            },
            _ => {}
        }
    });

    // これは何をしているかわからない、、
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
