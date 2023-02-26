# unique-id-generation

See https://fly.io/dist-sys/2/

Challenge #2: Unique ID Generation

In this challenge, you'll need to implement a globally-unique ID generation system that runs against Maelstrom's unique-ids workload. Your service should be totally available, meaning that it can continue to operate even in the face of network partitions.


Build with `cargo run`
Then from the directory maelstrom is installed run

```
./maelstrom test -w unique-ids --bin ~/src/fly-io-distributed-systems-challenges/02-unique-id-generation/target/debug/unique-id-generation --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

See test resuts:

```
./maelstrom serve
```
