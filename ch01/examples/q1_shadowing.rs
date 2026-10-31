// Q1 / Q1' 解答例
// 実行：cargo run -p ch01 --example q1_shadowing

/// Q1：シャドーイング。ブロック内の x は別の変数なので、外側の x は 6 のまま。
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}"); // 12
    }
    println!("outer: {x}"); // 6
}

/// Q1' (b)：mut で同じ変数を書き換える。ブロックを抜けても 12 のまま。
/// （Q1' (a) は mut がないのに再代入するので、コンパイルエラーになる）
fn mutation() {
    let x = 5;
    let mut x = x + 1;
    {
        x = x * 2;
        println!("inner: {x}"); // 12
    }
    println!("outer: {x}"); // 12
}

fn main() {
    println!("--- Q1: シャドーイング ---");
    shadowing();
    println!("--- Q1' (b): mut で再代入 ---");
    mutation();
}
