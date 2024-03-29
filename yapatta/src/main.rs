///
///     @file       main.rs
///     @breif      rustを使って円周率1億桁
///     @author     yapatta 
///     @date       2021/02/11
///     @details    Rustを使って円周率1億桁計算したお話
///                 https://zenn.dev/uu/articles/48e2d4098b6aca
///
///                 アルゴリズムはChudnovskyの公式で高速化手法としてBinary Splitting Methodを用いました。アルゴリズムの説明は以下の記事を参考にして下さい。
///                 https://qiita.com/peria/items/c02ef9fc18fb0362fb89
///       

use rug::{ops::Pow, Float, Integer};
use quanta::Clock;

// 定数たち
//const N: i64 = 100000000; // 一億桁
const N: i64 = 1000000;     // 百万桁
const SN: i64 = N / 14; // n: small N
const A: i64 = 13591409;
const B: i64 = 545140134;
const C: i64 = 640320;
const CT: i64 = C * C * C;
const CTD24: i64 = CT / 24;

fn calc_x(k: i64) -> Integer {
    if k == 0 {
        return Integer::from(1);
    }
    Integer::from(k).pow(3) * CTD24
}

fn calc_y(k: i64) -> Integer {
    A + Integer::from(B) * k
}

fn calc_z(k: i64) -> Integer {
    if k == SN - 1 {
        return Integer::from(0);
    }
    (-1) * Integer::from((6 * k + 1) * (2 * k + 1)) * (6 * k + 5)
}

fn calc(left: i64, right: i64) -> (Integer, Integer, Integer) {
    if right - left == 1 {
        return (calc_x(left), calc_y(left), calc_z(left));
    }

    let mid = (left + right) >> 1;

    let (lx, ly, lz) = calc(left, mid);
    let (rx, ry, rz) = calc(mid, right);

    (lx * &rx, &rx * ly + ry * &lz, &lz * rz)
}

fn main() {
    let clock = Clock::new();   // 20220320 Add.Nitobe
    let start = clock.now();    // 20220320 Add.Nitobe
    let mut prev;               // 20220320 Add.Nitobe

    let (x, y, _z) = calc(0, SN);
    let mut now = clock.now();
    eprintln!("BS:    {:?}", now.duration_since(start));
    prev = now;

    // with_valのprocはあくまでも有効桁のビット長(N / log_10^2), 10進数の桁数とは違う
    // 1e8 / log10 ^2の演算結果を四捨五入した値
//    let prec: u32 = 332192810;
//    let prec: u32 = 332192809+30;     // 十進一億桁
    let prec: u32 = 3321928+30;         // 十進百万桁

    // precは10進数1e8桁の場合u32の制限に引っかからない(u32のMAXが4294967295)
    let ans = Float::with_val(prec, CT).sqrt() * x / 12 / y;
    now = clock.now();
    eprintln!("end:   {:?}", now.duration_since(prev));
    prev = now;

    println!("{}", ans);
    now = clock.now();
    eprintln!("print: {:?}", now.duration_since(prev));
    eprintln!("total: {:?}", now.duration_since(start));
}
