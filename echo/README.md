# echo

See https://fly.io/dist-sys/1/

Our first challenge is more of a "getting started" guide" to get the hang of working with Maelstrom. In Maelstrom, we create a node which is a binary that receives JSON messages from STDIN and sends JSON messages to STDOUT. You can find a full protocol specification on the Maelstrom project. https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md

Build with `cargo run`
Then from the directory maelstrom is installed run

```
./maelstrom test -w echo --bin ~/src/fly-io-distributed-systems-challenges/echo/target/debug/echo --node-count 1 --time-limit 10
```

See test resuts:

```
./maelstrom serve
```
