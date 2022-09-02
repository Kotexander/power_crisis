cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/power_crisis.wasm .
if [[ $OSTYPE == 'linux'* ]]; then 
    xdg-open http://127.0.0.1:4000
elif [[ $OSTYPE == 'msys'* ]]; then 
    start http://127.0.0.1:4000
fi
basic-http-server .