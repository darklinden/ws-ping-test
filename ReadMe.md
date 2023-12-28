# A Websocket Time Request Test For Pinus

## Folder Structure

```text
├── client // client code
├── servers
│   ├── pinus-ws // pinus server
│   └── rust-ws // rust actix server
└── README.md
```

## How to run

* Prepare Environment

  * Install Nodejs
  * Install Rust

* Run Test For Pinus Server

```bash
sh test-pinus-ws.sh
```

* Run Test For Rust Actix Server

```bash
sh test-rust-ws.sh
```

## Test Result

* Pinus Server

```text
[2023-12-28T11:50:54Z INFO  client] 8 Pingpong: {"c":1703764254432,"s":1703764254434} cost 3 server time: 1703764254434
[2023-12-28T11:50:54Z INFO  client] 9 Pingpong: {"c":1703764254432,"s":1703764254433} cost 2 server time: 1703764254433
[2023-12-28T11:50:54Z INFO  client] 4 Pingpong: {"c":1703764254413,"s":1703764254420} cost 7 server time: 1703764254420
[2023-12-28T11:50:54Z INFO  client] 5 Pingpong: {"c":1703764254426,"s":1703764254427} cost 1 server time: 1703764254427
[2023-12-28T11:50:54Z INFO  client] 7 Pingpong: {"c":1703764254432,"s":1703764254432} cost 0 server time: 1703764254432
[2023-12-28T11:50:54Z INFO  client] 6 Pingpong: {"c":1703764254426,"s":1703764254427} cost 2 server time: 1703764254427
[2023-12-28T11:50:54Z INFO  client] 2 Pingpong: {"c":1703764254413,"s":1703764254420} cost 7 server time: 1703764254420
[2023-12-28T11:50:54Z INFO  client] 3 Pingpong: {"c":1703764254413,"s":1703764254416} cost 5 server time: 1703764254416
[2023-12-28T11:50:54Z INFO  client] 1 Pingpong: {"c":1703764254413,"s":1703764254418} cost 5 server time: 1703764254418
[2023-12-28T11:50:54Z INFO  client] 10 Pingpong: {"c":1703764254432,"s":1703764254432} cost 1 server time: 1703764254432
```

* Rust Actix Server

```text
[2023-12-28T11:53:04Z INFO  client] 6 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 7 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 5 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 4 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 1 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 3 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 2 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 10 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 9 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
[2023-12-28T11:53:04Z INFO  client] 8 Pingpong: {"c":1703764384452,"s":1703764384452} cost 0 server time: 1703764384452
```

## Conclusion

Pinus is very easy to use, but it is not suitable for high performance servers.
**DO NOT USE PINUS IF YOU NEED A HIGH PERFORMANCE SERVER.**
