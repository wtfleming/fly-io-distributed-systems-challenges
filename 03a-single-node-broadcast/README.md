# single-node-broadcast

See https://fly.io/dist-sys/3a/

Challenge #3a: Single-Node Broadcast
In this challenge, you'll need to implement a broadcast system that gossips messages between all nodes in the cluster. Gossiping is a common way to propagate information across a cluster when you don't need strong consistency guarantees.

Build with `cargo run`
Then from the directory maelstrom is installed run

```
./maelstrom test -w broadcast --bin ~/src/fly-io-distributed-systems-challenges/03a-single-node-broadcast/target/debug/single-node-broadcast --node-count 1 --time-limit 20 --rate 10
```

See test resuts:

```
./maelstrom serve
```
