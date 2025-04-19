cp ../../README.md .
bash ../../git_scripts/push.sh
cargo publish
rm -f README.md
cargo clean
