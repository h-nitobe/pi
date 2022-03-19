///
///     @file       main.rs
///     @brief      atctan による円周率１億桁
///     @author     新渡戸広明
///     @date       2022/03/13
///     @details    π/4 = arctan(1)
///                 π = 4 * arctan(1)
///

/// rug: C言語 GMP 多倍長ライブラリの rust ラッパ
use rug::Float;
use quanta::Clock;

/// 二進数での精度（桁数）
/// 1/log10(2) = 3.32192809488736234789
/// の 100000000 倍
//const PREC: u32 =   332192809 + 33;   // 十進一億桁
const PREC: u32 =   3321928 + 33;       // 十進百万桁

fn main() {
    let clock = Clock::new();
    let start = clock.now();

    println!("{}", Float::with_val(PREC, 1).atan() * 4);

    let stop = clock.now();
    eprintln!("{:?}", stop.duration_since(start));
}
