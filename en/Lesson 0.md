# Lession 0: Local two nodes testnet

### Introduction

I's well known that blockchain is a network composed N nodes, each of them communicates via p2p protocal. Only multiple nodes network becomes a real blockchain network, which is neccessary to learn more about substrate. The goal of this lession is to build local two nodes testnet.

Notes:

- All the instructions are operated on Ubuntu 16.04, other operating systems are not guranteed to behave exactly shown in this lession.

### Basics

```bash
// Build a Rust cargo project
cargo build

// Run a Rust cargo project directly
cargo run
```

The network identifier of node is something like `/ip4/0.0.0.0/tcp/30333/p2p/QmVky5aVaPhardUzSfYSQPhAhdkNiMQVzRGyPEe2vkxn3U`.

#### Operating steps

1. Install Rust and dependencies

```bash
sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup update stable
cargo install --git https://github.com/alexcrichton/wasm-gc
```

Refer to [Hacking on Substrate](https://github.com/paritytech/substrate#61-hacking-on-substrate) for more details.

2. Download the code for this demo

```bash
git clone https://github.com/chainx-org/Quick-Start-Substrate.git
cd Quick-Start-Substrate/quick-start-substrate
```

3. Build the demo

```bash
./build.sh
```

If you succeed to build the demo, you'll see the following output and the generated binary `quick-start-substrate` should be present in `target/debug/quick-start-substrate`.

```bash
...
Finished dev [unoptimized + debuginfo] target(s) in 1m 15s
```

Notes:

- Alice and Bob are validators by default, whose seed is the name itself. The seed would be used for running the node in the following section.

#### Run first node

Run the first node that seed is specified to `Alice`:

```bash
./target/debug/quick-start-substrate --chain=local --validator --key=Alice --base-path=./Alice
```

excerpt from `quick-start-substrate --help`:

```
--validator       Enable validator mode
--chain <CHAIN_SPEC>                         Specify the chain specification (one of dev, local or staging)
--key <STRING>                               Specify additional key seed
--base-path <PATH>                           Specify custom base path.
```

-  `--chain=local`: means we are going to run a local testnet with two validators: Alice and Bob.
-  `--validator`: means to run as an authority node.
-  `--key`: validator node needs a private key for block authoring.
-  `--base-path`: where to store the whole blockchain storage.

If everything goes well, you'll see the similar output as follows:

```bash
2019-03-01 21:19:21 Loaded block-time = 10 seconds from genesis on first-launch
2019-03-01 21:19:21 Best block: #0
2019-03-01 21:19:21 Local node address is: /ip4/0.0.0.0/tcp/30333/p2p/QmVky5aVaPhardUzSfYSQPhAhdkNiMQVzRGyPEe2vkxn3U
2019-03-01 21:19:21 Listening for new connections on 127.0.0.1:9944.
```

`/ip4/0.0.0.0/tcp/30333/p2p/QmVky5aVaPhardUzSfYSQPhAhdkNiMQVzRGyPEe2vkxn3U` is the network identifier of current node, which will be used for other nodes to connect to itself via `--bootnodes=NetworkID`.

#### Run second node

Open a new terminal and enter the same directory, run the second node Bob:

```bash
./target/debug/quick-start-substrate --chain=local --validator --key=Bob --base-path=./Bob --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/*******
```

- `--base-path`: use a different path for Bob.
- `--key`: use Bob's seed as key.
- `--bootnodes`: network address of the node we want to connect to, i.e., NetworkID of Alice above. In addition, we have to change `0.0.0.0` to `127.0.0.1` to connect successfully.

```bash
Listening for new connections on 127.0.0.1:52946.
2019-03-01 21:42:52 Kademlia random query has yielded empty results
2019-03-01 21:42:52 Using authority key 5Gw3s7q4QLkSWwknsiPtjujPv3XM4Trxi5d4PgKMMk3gfGTE
2019-03-01 21:42:57 Idle (1 peers), best: #0 (0x4469…06fb), finalized #0 (0x4469…06fb), ⬇ 0.8kiB/s ⬆ 0.8kiB/s
2019-03-01 21:43:00 Imported #1 (0x8881…c674)
2019-03-01 21:43:02 Idle (1 peers), best: #1 (0x8881…c674), finalized #0 (0x4469…06fb), ⬇ 0.2kiB/s ⬆ 0.1kiB/s
2019-03-01 21:43:07 Idle (1 peers), best: #1 (0x8881…c674), finalized #0 (0x4469…06fb), ⬇ 0 ⬆ 0
```

When you see `1 peers`, that means the two nodes Alice and Bob have been connected to each other. Congratulations!

```
2019-03-01 21:43:00 Imported #1 (0x8881…c674)
2019-03-01 21:43:02 Idle (1 peers), best: #1 (0x8881…c674), finalized #0 (0x4469…06fb), ⬇ 0.2kiB/s ⬆ 0.1kiB/s
2019-03-01 21:43:07 Idle (1 peers), best: #1 (0x8881…c674), finalized #0 (0x4469…06fb), ⬇ 0 ⬆ 0
2019-03-01 21:43:10 Starting consensus session on top of parent 0x888165c1c89bab5742f474240daa4c41d09f0a821a4659aa26829417a09bc674
2019-03-01 21:43:10 Prepared block for proposing at 2 [hash: 0xfa1740f96764b23f778b8ee5fedd4ea7f729635425629269940fc6ec5cf520bd; parent_hash: 0x8881…c674; extrinsics: [0x2acc…6d72]]
2019-03-01 21:43:10 Pre-sealed block for proposal at 2. Hash now 0xb4e0b2f0e793a043d9e620077e7793ed374695214da7681a942dc6d4888c65b6, previously 0xfa1740f96764b23f778b8ee5fedd4ea7f729635425629269940fc6ec5cf520bd.
2019-03-01 21:43:10 Imported #2 (0xb4e0…65b6)
2019-03-01 21:43:12 Idle (1 peers), best: #2 (0xb4e0…65b6), finalized #0 (0x4469…06fb), ⬇ 0.2kiB/s ⬆ 0.3kiB/s
```

You could also see `best: #2` is increaingly stable, which means the height of localtest is higher and higher. Now, your have already succeeded to run your first substrate chain!

### Conclusion

In this lession, we have done with building and running a two nodes localtest. We will learn how to interact with the testnet via APIs.
