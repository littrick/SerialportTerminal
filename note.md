```sh
cargo r -- connect com14
```
```sh
cat | bash <<'EOF'
target_list=(
    aarch64-pc-windows-msvc
    x86_64-pc-windows-msvc
    aarch64-unknown-linux-musl
    x86_64-unknown-linux-musl
)
for target in "${target_list[@]}"; do
    echo "Building for target: $target"
    cargo build --release --target ${target} \
    -Z unstable-options --artifact-dir target/artifacts/${target}
done
EOF
```