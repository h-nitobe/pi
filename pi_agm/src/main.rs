///
///     @file       main.rs
///     @brief      Gausss–Legendre による円周率一億桁
///     @author     新渡戸広明
///     @date       2022/03/14
///     @details
///

/// rug: C言語 GMP 多倍長ライブラリの rust ラッパ
use rug::Float;

/// 1/log(2)=3.3219280948873623478703194294894
/// の一億倍
//const PREC: u32 = 332192809 + 33;
const PREC: u32 = 332 + 33;
const MAX:  u32 = 28;

fn main() {
    let mut a0 = Float::with_val(PREC, 1.0);
    let mut b  = Float::with_val(PREC, 2.0).recip_sqrt();
    let mut t0 = Float::with_val(PREC, 0.25);
    let mut p  = Float::with_val(PREC, 1.0);

    for _i in 1..=MAX {
        let a1 = Float::with_val(PREC, &a0 + &b) / 2;
        b  = Float::with_val(PREC, &a0 * &b).sqrt();
        let mut t1  = Float::with_val(PREC, &a0 - &a1);
        t1 = Float::with_val(PREC, &t1 * &t1) * &p;
        t0 = Float::with_val(PREC, &t0 - &t1);
        p  = Float::with_val(PREC, &p) * 2;
        a0 = Float::with_val(PREC, &a1);
    }
    a0 = Float::with_val(PREC, &a0 + &b);
    a0 = Float::with_val(PREC, &a0 * &a0);
    t0 = Float::with_val(PREC, &t0) * 4;
    a0 = Float::with_val(PREC, &a0 / &t0);

    println!("{}", a0);

}
