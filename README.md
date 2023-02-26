# fly-io-distributed-systems-challenges

Solutions for a series of distributed systems challenges brought to you by Fly.io.

https://fly.io/dist-sys/



# Maelstrom
The challenges are built on top of a platform called Maelstrom, which in turn, is built on Jepsen. This platform lets you build out a "node" in your distributed system and Maelstrom will handle the routing of messages between the those nodes. This lets Maelstrom inject failures and perform verification checks based on the consistency guarantees required by each challenge.

## Installing Maelstrom
Maelstrom is built in Clojure so you'll need to install OpenJDK. It also provides some plotting and graphing utilities which rely on Graphviz & gnuplot. If you're using Homebrew, you can install these with this command:


Next, you'll need to download Maelstrom itself. These challenges have been tested against the Maelstrom 0.2.2. Download the tarball & unpack it. You can run the maelstrom binary from inside this directory.
https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.2
