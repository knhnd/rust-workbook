# ch01 基本文法

対応するThe Bookの章：第1〜3章
- [第3章 一般的なプログラミングの概念](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)（[日本語版](https://doc.rust-jp.rs/book-ja/ch03-00-common-programming-concepts.html)）

第1章（インストール）と第2章（数当てゲーム）は、The Bookの手順どおりに一度動かしておけばOKなのでスキップ。

## Python / TypeScript との違い（要点）

| 観点 | Python / TS | Rust |
|---|---|---|
| 変数の変更 | Python：自由に再代入できる<br>TS：`let` は再代入でき、`const` はできない | **デフォルトで変更不可**。変更するなら `let mut` |
| 不変の深さ | TSの `const` は中身（配列の要素など）を変更できる | `mut` なしだと**中身も含めて**変更できない |
| 同名の再宣言 | Python：ただの再代入<br>TS：同じスコープではエラー | **シャドーイング**：新しい変数を作る（型も変えてよい） |
| ブロック | Python：`if` や `for` にスコープがない | `{ }` だけで独立したブロック（スコープ）を作れる |

## 問題

### Q1【予想問題】シャドーイングとスコープ

`src/bin/q1_shadowing.rs` を**実行する前に**出力を予想してください。その後、実行して確認。

```bash
cargo run -p ch01 --bin q1_shadowing
```

<details>
<summary>ヒント</summary>

`{ }` のブロックの中で `let` した変数は、ブロックの外では見えない。
</details>

### Q1'【予想問題】シャドーイングと `mut` の違い

Q1のコードを次のように書き換えると、どうなるでしょうか。予想してから、実際に書き換えて確かめてください（確かめたら元に戻しておくこと）。

**(a)** ブロック内の `let x = x * 2;` を、`let` なしの `x = x * 2;` に変える

**(b)** (a) に加えて、2行目の `let x = x + 1;` を `let mut x = x + 1;` に変える。このとき `outer:` の値はいくつになるでしょうか？

<details>
<summary>ヒント</summary>

`let` があるときは新しい変数が作られる。`let` がないときは、今見えている一番新しい変数への代入になる。
</details>

### Q2【エラー修正】不変性と型

`src/bin/q2_mutability.rs` には、コンパイルエラーの原因が**2つ**ある。

1. 実行して、エラーメッセージを読んでください。最初は1つしか表示されない。
2. 1つ目を直すと、もう1つのエラーが現れる。それぞれの原因を、自分の言葉で説明してください。
3. 両方を修正して、コンパイルが通るようにしてください。
4. （考察）なぜエラーは2つ同時に表示されなかったのでしょうか？

```bash
cargo run -p ch01 --bin q2_mutability
```

<details>
<summary>ヒント</summary>

`spaces` のほうは、`mut` を使わずにシャドーイングを使うと通る。
</details>

### Q3【エラー修正】式と文

`src/bin/q3_expressions.rs` は、Pythonの感覚だと正しく見えるのにエラーになる。

1. なぜエラーになるのかを説明してください。
2. **1文字だけ**削除して直してください。

```bash
cargo run -p ch01 --bin q3_expressions
```

<details>
<summary>ヒント</summary>

Rustでは、セミコロンを付けると「式」が「文」に変わる。文は値を返さない。
</details>

### Q3'【予想問題】ブロックの値

次のコードで、`y` の値は何になるでしょうか？

```rust
let y = {
    let a = 3;
    a * 2;
};
```

<details>
<summary>ヒント</summary>

ブロック `{ }` の値は、最後にあるセミコロンのない式の値になる。
</details>

### Q4【実装＋実験】フィボナッチ数と整数オーバーフロー

`src/bin/q4_fibonacci.rs` の `todo!()` を、ループで実装してください（再帰は使わない）。

1. まずそのまま実行して、`todo!()` が何をするか観察してください。
2. `todo!()` をループを使った実装に置き換えてください。`fib(10)` が `55` になればOK。
3. `fib(100)` を次の**両方**で実行し、挙動の違いを観察してください。
   ```bash
   cargo run -p ch01 --bin q4_fibonacci
   cargo run -p ch01 --bin q4_fibonacci --release
   ```
4. （発展）オーバーフローを安全に扱う関数 `checked_fib(n: u32) -> Option<u64>` を書いてください。`checked_fib(10)` は `Some(55)`、`checked_fib(100)` は `None` を返すこと。表示には `println!("{:?}", checked_fib(10));` を使う。
5. （さらに発展）`checked_fib(93)` を試してください。`fib(93)` は `u64` に収まる値なのに、`None` になっていないでしょうか？

<details>
<summary>ヒント</summary>

- ループは The Book 3.5節。基本は `for`。ループ変数を使わないときは `for _ in 0..n` と書く。
- 手順4は The Book 3.2節の「Integer Overflow」のコラムと、整数型の `checked_add` メソッドがヒント。
- 手順5は、ループの中で計算している `a + b` が「何番目のフィボナッチ数か」を考えるとわかる。
</details>

---

解き終わったら [ANSWERS.md](./ANSWERS.md) で解説を確認。
