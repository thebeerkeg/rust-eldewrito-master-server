# rust-eldewrito-master-server
A master server for the Halo Online mod called "ElDewrito", written in Rust.

[NodeJS]: https://github.com/ElDewrito/ElDewrito-MasterServer

JMeter Benchmarks: Concurrent requests per second = average latency in miliseconds.

| Requests / second | Rust(ms) | [NodeJS](ms) |
|------------|------|--------|
|     1       |   1   |    2   |
|     10       |   1   |    2    |
|      50      |   1   |    2    |
|      250      |   2   |    13    |
|      500      |   2   |    50-80    |
|      1000      |   2   |    100-400    |
|      2000      |   2   |    100-400    |
|      10000      |   2   |    1000+ (died on 5000 requests)   |

As you can see here, the Rust based master server performs significantly better than the NodeJS based master server under load.
