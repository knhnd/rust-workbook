// Q2 解答例
// 実行：cargo run -p ch01 --example q2_mutability

fn main() {
    // カウンタは同じ変数の値を更新するので、mut が適切
    let mut count = 0;
    count += 1;

    // 文字列（&str）から長さ（usize）へ型が変わるので、シャドーイングを使う
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{count} {spaces}"); // 1 3
}
