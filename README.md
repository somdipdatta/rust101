# rust101
Learning Rust

# Resources


**The book**: https://doc.rust-lang.org/stable/book/


# Installation Diary (Windows)

Reference: https://code.visualstudio.com/docs/languages/rust

- Installed rust in my Windows machine from https://rustup.rs/, which also prompted me to install Visual Studio Community Installer.
- created this repo `rust101` in github, with just the README, and cloned it to my local VS Code.
- Restarted my machine, relaunched VS Code
- Tried `rustc --version`
- In the terminal used `cargo init rust101` to populate this directory with the scaffolding. The file `main.rs` includes the Hello World function.

```md
├── src
│   ├── main.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

- Installed rust-analyzer extension
- Tried `cargo build`, then `cargo run`.
- Success!!
