// Q1【予想問題】実行する前に出力を予想してください。

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}");
    }
    println!("outer: {x}");
}
