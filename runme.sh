# Run me before commiting
cargo fmt --all -- --check  
cargo fmt --all  
cargo build
cargo clippy --all-targets --all-features -- -D warnings

git status

# git add . 
# git commit -m "fixes"
# git push origin


# To start Crates io push from github actions change version as you go
# git tag -a v0.1.0 -m "Release version 0.1.0"
# git push origin v0.1.0
