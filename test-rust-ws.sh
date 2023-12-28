#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
PROJECT_DIR="$(realpath "${BASEDIR}")"

echo "start server"
SERVER_DIR=$PROJECT_DIR/servers/rust-ws

cd $SERVER_DIR
export RUST_LOG=info
export RUST_BACKTRACE=1
export HTTP_SERVER_CLUSTER=0 # 0 means 1 worker
export HTTP_SERVER_PORT=3010
nohup cargo run --release >"${PROJECT_DIR}/rust-ws-server.log" 2>&1 &

CLIENT_DIR=$PROJECT_DIR/client
if [ ! -f "$CLIENT_DIR/target/release/client" ]; then
    echo "build client"
    cd $CLIENT_DIR
    cargo build --release
fi

echo "wait for server to start"
sleep 1

export WS_URL="ws://localhost:3010"
export RUST_LOG=info
cd $PROJECT_DIR
for i in {1..10}; do
    echo "Running test $i"
    nohup $CLIENT_DIR/target/release/client $i >"${PROJECT_DIR}/rust-ws-test${i}.log" 2>&1 &
done

sleep 1

echo "closing server"
kill $(lsof -t -i:3010)

echo "test results:"
find $PROJECT_DIR -name "rust-ws-test*.log" -exec cat {} \; | grep "Pingpong: {"
