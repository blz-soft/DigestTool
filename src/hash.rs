//! ハッシュを行う関数のモジュール
//! 返り値は以下のライブラリを使っています
//! generic-array = "0.14.4"
//! typenum = "1.14.0"

use sha2::{Sha256, Sha512, Digest};
use sha3::{Sha3_512, Sha3_256};
use generic_array::GenericArray;
use typenum::{uint::{UInt,UTerm}, bit::{B0, B1}};

pub type HashValue256 = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;
pub type HashValue512 = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>>;

/// #sha256関数
/// readerを受け取って、読み取りつハッシュ値を求めていき、readerが空になったらハッシュ値を返却します。
/// ## 引数 
/// - reader
/// ## 返り値
/// Result型 std::ioのError あるいは、(ファイルサイズ, ハッシュ値の配列)
pub fn sha2_256(input_reader: &mut impl std::io::Read) -> Result<(u64, HashValue256), std::io::Error> {
    let hasher = Sha256::new();
    let mut hasher = std::io::BufWriter::new(hasher);

    // リーダーを読み込んでハッシュ値を計算する
    let data_size = std::io::copy(&mut *input_reader, &mut hasher)?;

    // 結果を取り出す
    let hasher = hasher.into_inner()?;
    let hash_value = hasher.finalize();
    Ok((data_size, hash_value))
}

/// #sha512関数
/// readerを受け取って、読み取りつハッシュ値を求めていき、readerが空になったらハッシュ値を返却します。
/// ## 引数 
/// - reader
/// ## 返り値
/// Result型 std::ioのError あるいは、(ファイルサイズ, ハッシュ値の配列)
pub fn sha2_512(input_reader: &mut impl std::io::Read) -> Result<(u64, HashValue512), std::io::Error> {
    let hasher = Sha512::new();
    let mut hasher = std::io::BufWriter::new(hasher);

    // リーダーを読み込んでハッシュ値を計算する
    let hash_read_result = std::io::copy(&mut *input_reader, &mut hasher);

    // 結果を取り出す
    let data_size = hash_read_result?;
    let hasher = hasher.into_inner()?;
    let hash_value = hasher.finalize();
    Ok((data_size, hash_value))
}

/// #sha3_256関数
/// readerを受け取って、読み取りつハッシュ値を求めていき、readerが空になったらハッシュ値を返却します。
/// ## 引数 
/// - reader
/// ## 返り値
/// Result型 std::ioのError あるいは、(ファイルサイズ, ハッシュ値の配列)
pub fn sha3_256(input_reader: &mut impl std::io::Read) -> Result<(u64, HashValue256), std::io::Error> {
    let hasher = Sha3_256::new();
    let mut hasher = std::io::BufWriter::new(hasher);

    // リーダーを読み込んでハッシュ値を計算する
    let hash_read_result = std::io::copy(&mut *input_reader, &mut hasher);

    // 結果を取り出す
    let data_size = hash_read_result?;
    let hasher = hasher.into_inner()?;
    let hash_value = hasher.finalize();
    Ok((data_size, hash_value))
}

/// #sha3_512関数
/// readerを受け取って、読み取りつハッシュ値を求めていき、readerが空になったらハッシュ値を返却します。
/// ## 引数 
/// - reader
/// ## 返り値
/// Result型 std::ioのError あるいは、(ファイルサイズ, ハッシュ値の配列)
pub fn sha3_512(input_reader: &mut impl std::io::Read) -> Result<(u64, HashValue512), std::io::Error> {
    let hasher = Sha3_512::new();
    let mut hasher = std::io::BufWriter::new(hasher);

    // リーダーを読み込んでハッシュ値を計算する
    let hash_read_result = std::io::copy(&mut *input_reader, &mut hasher);

    // 結果を取り出す
    let data_size = hash_read_result?;
    let hasher = hasher.into_inner()?;
    let hash_value = hasher.finalize();
    Ok((data_size, hash_value))
}

#[cfg(test)]
mod test{
    use super::*;

    /// 成功パターン
    #[test]
    fn ok_sha256() {
        let expected_hash = hex_literal::hex!("E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855");
        let expected_size = 0;

        let mut input_reader: &[u8] = &[];
        let (data_size, hash_value) = sha2_256(&mut input_reader).unwrap();

        assert_eq!(expected_size, data_size);
        assert_eq!(expected_hash, hash_value[..]);
    }

    /// 失敗パターン
    #[test]
    fn ng_sha256() {
        let expected_hash = hex_literal::hex!("E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B854");
        let expected_size = 0;

        let mut input_reader: &[u8] = &[];
        let (data_size, hash_value) = sha2_256(&mut input_reader).unwrap();

        assert_eq!(expected_size, data_size);
        assert_ne!(expected_hash, hash_value[..]);
    }
}