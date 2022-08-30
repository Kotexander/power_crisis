cargo build --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/debug/power_crisis.wasm .
if [[ $OSTYPE == 'linux'* ]]; then 
    xdg-open http://127.0.0.1:4000
elif [[ $OSTYPE == 'linux'* ]]; then 
    start http://127.0.0.1:4000
fi
basic-http-server .