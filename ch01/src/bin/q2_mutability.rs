// Q2【エラー修正】このコードにはコンパイルエラーの原因が2つあります。
// エラーの原因を説明してから修正してください。

fn main() {
    let count = 0;
    count += 1;

    let mut spaces = "   ";
    spaces = spaces.len();
    println!("{count} {spaces}");
}
