check:
    mise exec -- cargo fmt --all -- --check
    mise exec -- cargo test --workspace --all-features
    mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
