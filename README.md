# rust-shakyo

[The Rust Programming Language 日本語版 - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)


## 開発環境の構築

- [Installation - The Rust Programming Language](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [RustRover: JetBrains の Rust IDE](https://www.jetbrains.com/ja-jp/rust/)

## cargoに関するメモ

```bash
# vcs を指定することで .gitignore などが自動生成される
cargo new --vcs git <project-name>

cd project-name

# ビルド & 実行
cargo run

# ビルド & エラーのチェック
cargo check

# リリースビルド
cargo build --release
```

## 参考

- [RealWorld 業務 Rust #怪文書 - Qiita](https://qiita.com/legokichi/items/4e85ec1e74f4e754fb94)
