cargo build --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/debug/power_crisis.wasm .
basic-http-server .