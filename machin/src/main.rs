///
///     @file       main.rs
///     @brief      machin's formula による円周率１億桁
///     @author     新渡戸広明
///     @date       2022/03/19
///     @details    π/4 = 4arctan(1/5) - arctan(1/239)

/// rug: C言語 GMP、MPFR、MPC ライブラリの rust ラッパ
/// quanta: 高速タイミングライブラリ
use rug::Float;
use quanta::Clock;

/// ２進数での精度（桁数）
/// 1/log(2) = 3.32192809488736234789
/// の100000000倍
//const PREC:u32 = 332192809+33;
const PREC:u32 = 3321928+33;

fn main() {
    let clock = Clock::new();
    let start = clock.now();

    let mut  t5 = Float::with_val(PREC, 1);
    let mut now = clock.now();
    eprintln!("set 1:        {:?}", now.duration_since(start));
    let mut lap = now;

    t5 = Float::with_val(PREC, &t5 / 5).atan() * 4; 
    now = clock.now();
    eprintln!("4atan(1/5):   {:?}", now.duration_since(lap));
    lap = now;

    let mut t239 = Float::with_val(PREC, 1);
    now = clock.now();
    eprintln!("set 1:        {:?}", now.duration_since(lap));
    lap = now;

    t239 = Float::with_val(PREC, &t239 / 239).atan();
    now = clock.now();
    eprintln!("atan(1/239):  {:?}", now.duration_since(lap));
    lap = now;

    let   pi = Float::with_val(PREC, t5 - t239) * 4;
    now = clock.now();
    eprintln!("4(T5 - T239): {:?}", now.duration_since(lap));
    lap = now;

    println!("{}", pi);
    now = clock.now();
    eprintln!("print:        {:?}", now.duration_since(lap));
    eprintln!("total:        {:?}", now.duration_since(start));
}
