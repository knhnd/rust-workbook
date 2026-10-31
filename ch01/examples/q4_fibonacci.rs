// Q4 解答例
// 実行：cargo run -p ch01 --example q4_fibonacci

/// 手順2：ループで n 番目のフィボナッチ数を求める。
/// ループ中は常に a = fib(k), b = fib(k+1) の関係が保たれる。
/// 注意：fib(93) 以上では、debug ビルドだとオーバーフローで panic する。
fn fib(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

/// 手順4・5：オーバーフローしたら None を返す。
/// ループを 1 回減らして b を返すことで、答えに不要な fib(n+1) を計算しない。
fn checked_fib(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 1..n {
        // ? は「None ならその場で None を返す、Some なら中身を取り出す」
        let sum = a.checked_add(b)?;
        (a, b) = (b, sum);
    }
    Some(b)
}

fn main() {
    println!("fib(10) = {}", fib(10)); // 55

    for n in [10, 93, 94, 100] {
        println!("checked_fib({n}) = {:?}", checked_fib(n));
    }
    // checked_fib(10)  = Some(55)
    // checked_fib(93)  = Some(12200160415121876738)
    // checked_fib(94)  = None
    // checked_fib(100) = None
}
