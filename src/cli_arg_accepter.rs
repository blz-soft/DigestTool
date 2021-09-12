//! CLI引数を受け取るモジュール

// Cli ArgumentParser
use clap::*;

pub enum DigestAlgorithm {
    Sha256,
    Sha512
}

pub enum Mode {
    Digest,
    SetUp,
    CleanUp,
}

/// # CLI引数を受け取る関数
pub fn accept_cli_arg() -> (Option<String>, DigestAlgorithm, Mode) {
    let matches = app_from_crate!()
    .arg(Arg::with_name("input_file")
        .short("i")
        .long("input_file")
        .takes_value(true)
        .value_name("FILE"))
    .arg(Arg::with_name("sha512")
        .long("sha512")
        .takes_value(false))
    .arg(Arg::with_name("setup")
        .long("setup")
        .takes_value(false))
    .arg(Arg::with_name("clean_up")
        .long("clean_up")
        .takes_value(false))
    .get_matches();

    let input_file_path = match matches.value_of_lossy("input_file"){
        None => None,
        Some(file) => Some(file.to_string()),
    }; 

    let digest_algorithm = match matches.occurrences_of("sha512") {
        0 => DigestAlgorithm::Sha256,
        _ => DigestAlgorithm::Sha512,
    };
    
    let mode = match (matches.occurrences_of("setup"), matches.occurrences_of("clean_up")) {
        (0, 0) => Mode::Digest,
        (_, 0) => Mode::SetUp,
        (0, _) => Mode::CleanUp,
        (_, _) => {
            println!("セットアップとクリーンアップが同時に選択されています。");
            println!("Enterキーを押すと終了します");
            let mut word = String::new();
            std::io::stdin().read_line(&mut word).ok();
            std::process::exit(0);
        },
    };

    return (input_file_path, digest_algorithm, mode);
}