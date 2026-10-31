// Q3 / Q3' 解答例
// 実行：cargo run -p ch01 --example q3_expressions

/// Q3：最後の式（セミコロンなし）が戻り値になる。
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("{}", plus_one(5)); // 6

    // Q3'：最後の行が文（セミコロンあり）なので、ブロックの値は ()
    // a * 2 の結果を捨てているので、本来はコンパイラが警告を出す（ここでは意図的なので抑制）
    #[allow(unused_must_use)]
    let y = {
        let a = 3;
        a * 2;
    };
    println!("{y:?}"); // ()

    // 参考：ブロックも if も式なので、値を返せる
    let z = {
        let a = 3;
        a * 2
    };
    let grade = if z >= 6 { "合格" } else { "不合格" };
    println!("{z} {grade}"); // 6 合格
}
