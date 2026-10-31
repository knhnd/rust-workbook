# Rust Workbook

#### The Book 準拠の学習ノート兼問題集

Rust公式チュートリアル「[The Rust Programming Language](https://doc.rust-lang.org/book/)」（通称 The Book）に準拠した、手を動かして理解を確かめるための問題集。Python / TypeScript を普段使いしている人が Rust でつまずきやすそうなポイントをまとめて問題を作成。問題や解説は Claude を活用して作成。

> [!NOTE]
> これは作者自身がRustを学びながら作っている「学習ノート兼問題集」。誤りを見つけたら Issue で教えてください。

## 特徴

- **The Bookに準拠、ただし同じではない構成**：複数の章をまとめたり、他の教材の演習を加えたりした構成。章番号はThe Bookと一致しない（下の対応表を参照）
- **Python / TS との比較つき**：「Pythonならこう、TSならこう、Rustでは…」という観点の解説
- **実際につまずいた記録**：解説の「よくある間違い」は、初心者がハマりやすいと思ったポイント

## 使い方

1. 下の対応表で、該当するThe Bookの章（[日本語版](https://doc.rust-jp.rs/book-ja/)）を読む
2. 各章フォルダの `README.md` の問題を解く
3. 解き終わったら `ANSWERS.md` の解説と `examples/` の解答例を確認する

```bash
git clone https://github.com/knhnd/rust-workbook.git
cd rust-workbook

# 問題を実行する（問題によってはわざとコンパイルエラーになる）
cargo run -p ch01 --bin q1_shadowing

# 解答例を実行する
cargo run -p ch01 --example q1_shadowing
```

> [!TIP]
> 問題ファイルの一部は、わざとコンパイルエラーの状態で置いている。VS Code などでファイル名が赤く表示されるのは正常。

## 構成

```
rust-workbook/
├── Cargo.toml              # ワークスペース定義
└── chXX/
    ├── README.md           # 問題文・ヒント
    ├── ANSWERS.md          # 解説（解いた後に読むメモや解説）
    ├── src/bin/qN_名前.rs  # 問題ファイル（1ファイル＝1つの実行ファイル）
    └── examples/qN_名前.rs # 解答例
```

## 章立てと進捗

章立ては学習の進行に合わせて調整する場合あり。

| 章 | テーマ | The Book（英語版の章番号） | 問題 | 解説 |
|---|---|---|---|---|
| [ch01](./ch01/) | 基本文法 | 1〜3 | ✅ | ✅ |
| ch02 | 所有権と借用 | 4 | | |
| ch03 | 構造体とEnum・パターンマッチ | 5〜6 | | |
| ch04 | モジュールとコレクション | 7〜8 | | |
| ch05 | エラー処理 | 9 | | |
| ch06 | ジェネリクス・トレイト・ライフタイム | 10 | | |
| ch07 | テストとCLIプロジェクト | 11〜12 | | |
| ch08 | クロージャ・イテレータ・Cargo | 13〜14 | | |
| ch09 | スマートポインタ | 15 | | |
| ch10 | 並行性とasync | 16〜17 | | |
| ch11 | パターンとOOP | 18〜19 | | |
| ch12 | 高度な機能 | 20 | | |
| ch13 | 最終プロジェクト：Webサーバ | 21 | | |

※ The Bookの章番号は英語版に準拠

## 動作環境

- Rust 1.85 以降（Rust 2024 エディション）
- VSCode を使う場合は、推奨拡張機能の [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) がおすすめ

## 参考文献
- [Rustlings](https://rustlings.rust-lang.org/)
- [The Rust Programming Language - Brown University](https://rust-book.cs.brown.edu/)
