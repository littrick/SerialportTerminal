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
    rustup target add "$target"
done
EOF
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
```sh
git commit -am "update workflow" && git pull --rabase && git push
```
```sh
cargo build --release --message-format=json > build_msg.json
```
```sh
jq 'select(.reason == "compiler-artifact" and .target.kind[0] == "bin") | .executable' -r build_msg.json
```