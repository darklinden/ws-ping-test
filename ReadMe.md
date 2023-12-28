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
[2023-12-28T12:09:22Z INFO  client] 8 Pingpong: {"c":1703765362612,"s":1703765362619} cost 7 server time: 1703765362619
[2023-12-28T12:09:22Z INFO  client] 9 Pingpong: {"c":1703765362612,"s":1703765362618} cost 6 server time: 1703765362618
[2023-12-28T12:09:22Z INFO  client] 4 Pingpong: {"c":1703765362620,"s":1703765362627} cost 7 server time: 1703765362627
[2023-12-28T12:09:22Z INFO  client] 5 Pingpong: {"c":1703765362612,"s":1703765362615} cost 5 server time: 1703765362615
[2023-12-28T12:09:22Z INFO  client] 7 Pingpong: {"c":1703765362612,"s":1703765362618} cost 6 server time: 1703765362618
[2023-12-28T12:09:22Z INFO  client] 6 Pingpong: {"c":1703765362620,"s":1703765362625} cost 5 server time: 1703765362625
[2023-12-28T12:09:22Z INFO  client] 2 Pingpong: {"c":1703765362612,"s":1703765362617} cost 5 server time: 1703765362617
[2023-12-28T12:09:22Z INFO  client] 3 Pingpong: {"c":1703765362620,"s":1703765362625} cost 5 server time: 1703765362625
[2023-12-28T12:09:22Z INFO  client] 1 Pingpong: {"c":1703765362620,"s":1703765362626} cost 6 server time: 1703765362626
[2023-12-28T12:09:22Z INFO  client] 10 Pingpong: {"c":1703765362620,"s":1703765362626} cost 6 server time: 1703765362626
```

* Rust Actix Server

```text
[2023-12-28T12:09:38Z INFO  client] 6 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 7 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 5 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 4 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 1 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 3 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 2 Pingpong: {"c":1703765378261,"s":1703765378261} cost 0 server time: 1703765378261
[2023-12-28T12:09:38Z INFO  client] 10 Pingpong: {"c":1703765378263,"s":1703765378263} cost 0 server time: 1703765378263
[2023-12-28T12:09:38Z INFO  client] 9 Pingpong: {"c":1703765378263,"s":1703765378263} cost 0 server time: 1703765378263
[2023-12-28T12:09:38Z INFO  client] 8 Pingpong: {"c":1703765378263,"s":1703765378263} cost 0 server time: 1703765378263
```

## Conclusion

Pinus is very easy to use, but it is not suitable for high performance servers.
**DO NOT USE PINUS IF YOU NEED A HIGH PERFORMANCE SERVER.**
