name=out
cargo fmt
buny package .
cp -R ./target/wasm32-unknown-unknown/release/Payload ${name}
simload ./${name}
rm -r ${name}