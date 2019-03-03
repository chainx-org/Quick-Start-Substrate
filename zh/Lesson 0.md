## Lesson 0 学习搭建Substrate多节点测试网

*欢迎转载，请务必注明出处*

### 前言

我们知道区块链网络是由N个节点组成的网络，节点与节点之间通过P2P协议进行通信，只有多节点网络才是完整的区块链网络。因此学习多节点组成一个区块链网络，是进一步学习Substrate的基础。为了更好的理解，我们本课目标是搭建两个节点组成网络，并能正常共识出块。

*注意：本课程所有操作在Ubuntu 16操作系统下为示例，其他操作系统不保证预期完全一致*

### 背景知识

- 常用命令：
  - cargo build |run 构建程序|运行程序
  - /ip4/\*.\*.\*.\*/tcp/31125/p2p/Qm\****  节点在网络中地址的标识格式

### 操作步骤

#### 准备环境&Substrate节点模版

1. 安装Rust环境和依赖库

```shell
sudo apt install cmake pkg-config libssl-dev git clang libclang-dev  #安装依赖
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly #为工具链安装目标
rustup update stable
cargo install --git https://github.com/alexcrichton/wasm-gc #安装工具链
```

2. 下载节点模版

```shell
git clone https://github.com/chainx-org/Quick-Start-Substrate.git
cd Quick-Start-Substrate/quick-start-substrate
```

3. 构建节点程序

   ```
   ./build.sh
   ```

   若构建成功，则有类似以下输出，并在target/debug/下看到节点程序文件quick-start-substrate

   ```
   Finished dev [unoptimized + debuginfo] target(s) in 1m 15s
   ```

   *注意，节点程序中默认配置了Alice和Bob为验证人，其中*

   *Alice的key为0x416c696365202020202020202020202020202020202020202020202020202020*

   *Bob的key为0x426f622020202020202020202020202020202020202020202020202020202020*

   *在下文启动节点会用到*

#### 运行第一个节点

执行第一个节点Alice启动命令

```
./target/debug/quick-start-substrate --chain=local --validator --key=0x416c696365202020202020202020202020202020202020202020202020202020 --base-path=./Alice
```

启动参数说明

-  --chain=local  代表启动一个具有两个验证人Alice和Bob的测试网
-  --validator 代表以验证人身份启动
-  --key 代表验证人对应的私钥
-  --base-path代表节点数据库路径

正常启动之后，就看到类似的输出：

```
2019-03-01 21:19:21 Loaded block-time = 10 seconds from genesis on first-launch
2019-03-01 21:19:21 Best block: #0
2019-03-01 21:19:21 Local node address is: /ip4/0.0.0.0/tcp/30333/p2p/QmVky5aVaPhardUzSfYSQPhAhdkNiMQVzRGyPEe2vkxn3U
2019-03-01 21:19:21 Listening for new connections on 127.0.0.1:9944.
```

*注意输出中的/ip4/0.0.0.0/tcp/30333/p2p/QmVky5aVaPhardUzSfYSQPhAhdkNiMQVzRGyPEe2vkxn3U 是节点地址标识，在启动其他节点时会用到其标识来连接*

#### 运行第二个节点

新开一个终端，进入同样的目录，执行第二个节点Bob的启动命令

```
./target/debug/quick-start-substrate --chain=local --validator --key=0x426f622020202020202020202020202020202020202020202020202020202020 --base-path=./Bob --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/QmVky5aVaPhardUzSfYSQPhAhdkNiMQVzRGyPEe2vkxn3U
```

启动参数说明

- --base-path 代表节点数据库路径，与Alice节点区分开来
- --key 使用Bob的私钥，与Alice的区分开来
- --bootnodes 代表要连接的种子节点地址，在本课程中填的是第一个节点的地址标识，注意IP部分换成127.0.0.1即可

正常启动之后，会看到类似输出：

```
Listening for new connections on 127.0.0.1:52946.
2019-03-01 21:42:52 Kademlia random query has yielded empty results
2019-03-01 21:42:52 Using authority key 5Gw3s7q4QLkSWwknsiPtjujPv3XM4Trxi5d4PgKMMk3gfGTE
2019-03-01 21:42:57 Idle (1 peers), best: #0 (0x4469…06fb), finalized #0 (0x4469…06fb), ⬇ 0.8kiB/s ⬆ 0.8kiB/s
2019-03-01 21:43:00 Imported #1 (0x8881…c674)
2019-03-01 21:43:02 Idle (1 peers), best: #1 (0x8881…c674), finalized #0 (0x4469…06fb), ⬇ 0.2kiB/s ⬆ 0.1kiB/s
2019-03-01 21:43:07 Idle (1 peers), best: #1 (0x8881…c674), finalized #0 (0x4469…06fb), ⬇ 0 ⬆ 0
```

其中“1 peers” 代表节点Alice和节点Bob已连接成功。



#### 组网成功，查看日志

查看节点Alice或节点Bob的输出日志，会看到类似输出：

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

其中会看到类似有“1 peers” 的信息，代表两个节点已组网成功，会有类似“best: #2”的信息，代表测试网的块高度在不断增长。至此，说明测试网已经正常运转出块，恭喜你的第一条Substrate链已经跑起来了！

### 总结

本课中，我们通过几个简单的步骤，完成了节点程序的构建、节点的启动和节点之间的组网，并成功使得测试网正常共识出块。

下个课中，我们将学习，如何调用测试网的API接口与其交互。



