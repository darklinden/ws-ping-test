#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
PROJECT_DIR="$(realpath "${BASEDIR}")"

echo "start server"
SERVER_DIR=$PROJECT_DIR/servers/pinus-ws

cd $SERVER_DIR
yarn run build
nohup node dist/app.js >"${PROJECT_DIR}/pinus-ws-server.log" 2>&1 &

CLIENT_DIR=$PROJECT_DIR/client
if [ ! -f "$CLIENT_DIR/target/release/client" ]; then
    echo "build client"
    cd $CLIENT_DIR
    cargo build --release
fi

echo "wait for server to start"
sleep 5

export WS_URL="ws://localhost:3010"
export RUST_LOG=info
cd $PROJECT_DIR
for i in {1..10}; do
    echo "Running test $i"
    nohup $CLIENT_DIR/target/release/client $i >"${PROJECT_DIR}/pinus-ws-test${i}.log" 2>&1 &
done

sleep 2

echo "closing server"
kill $(lsof -t -i:3005)
kill $(lsof -t -i:3010)

echo "test results:"
find $PROJECT_DIR -name "pinus-ws-test*.log" -exec cat {} \; | grep "Pingpong: {"
