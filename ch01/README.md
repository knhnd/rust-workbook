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

### Q2以降

準備中。

---

解き終わったら [ANSWERS.md](./ANSWERS.md) で解説を確認。
