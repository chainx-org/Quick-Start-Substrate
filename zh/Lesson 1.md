## Lesson 1 学习调用区块链API接口实现用户转账

*欢迎转载，请务必注明出处*

### 前言

RPC是区块链对外提供的API方式之一。因此用户可以选择通过RPC调用的方式与区块链交互。Substrate的RPC高度抽象，只对外提供了四个模块，但它涵盖链的全部功能接口。并且Substrate有优秀的Metadata机制设计，链功能接口的变更会自动传导到调用客户端，客户端无需关注细节。

oo7-substrate框架是与Substrate高度匹配的API库。自动完成与Substrate链Metadata的对接。客户端只需关注具体功能接口而无需关注链本身逻辑。

本课学习如何利用oo7-substrate调用[Lesson 0](https://github.com/chainx-org/Quick-Start-Substrate/blob/master/zh/Lesson%200.md) 中构建的Substrate网络，获取链信息，并实现用户转账功能。

*注意：本课程所有操作在Ubuntu 16操作系统下为示例，其他操作系统不保证预期完全一样*

### 背景知识

- 基本概念：
  - *Header* 区块头代表块所有信息，它包括父哈希，存储根和外部trie根，摘要和块高
  - *Extrinsic* 交易代表区块链的外部数据，如合约调用
  - *Block*  区块代表Header和Extrinsic的组合

### 操作步骤

#### 安装NodeJs环境

参考[NodeJs官网](https://nodejs.org/en/) 安装NodeJs，并且将其升级到node>v10.10.0,npm>6.4.1。

#### 初始化客户端环境，安装oo7依赖

新建app目录，安装oo7-substrate包

```shell
mkdir app
cd app
npm install oo7-substrate tweetnacl
```

#### 调用接口，获得链信息

新建chain.js 文件，使用oo7连接测试网，并订阅块高度增长信息

```javascript
const substrate = require('oo7-substrate');

substrate.setNodeUri(["ws://127.0.0.1:9944"]); //设置节点RPC ws协议地址
//初始化oo7环境
substrate.runtimeUp.then(() => {
    substrate.chain.height.tie(height => { //订阅链的块高度变更
        console.log(`new block by block number: ${height}`);
      });
})
```

 执行chain.js 文件，会有类似持续输出：

```
...
new block by block number: 220
new block by block number: 221
new block by block number: 222
new block by block number: 223
```

以上输出代表客户端已经成功连接上测试网，并且监听到了网络信息的变更！

*substrate.chain对象下面还有其他对Header、Hash、Block订阅的接口，可以深入学习*

#### 调用接口，查询账户余额

测试网中已为Alice和Bob预置了一定数量的Coin。接下来我们就利用*substrate.calls*提供的API接口，实现查询账户功能。

新建balance.js文件，代码如下：

```
const substrate = require('oo7-substrate');
const nacl = require('tweetnacl');
substrate.setNodeUri(["ws://127.0.0.1:9944"]); //设置节点RPC ws协议地址

let Alice_KeyPair=nacl.sign.keyPair.fromSeed(substrate.stringToSeed('Alice'))//Alice的seed

//初始化oo7环境
substrate.runtimeUp.then(() => {
    substrate.runtime.balances.freeBalance(Alice_KeyPair.publicKey).then((v) => {
        console.log(substrate.bytesToHex(Alice_KeyPair.publicKey),'Alice FreeBalance=' ,v.toString())
    })
})

```

*其中substrate.runtime.balances.freeBalance是测试网runtime中balances模块提供的转账接口，在substrate.runtime对象下面还有其他模块和接口可以调用，其与quick_start_substrate/rntime/src/lib.rs 中的声明是完全一致的，当链上runtime模块和接口变更时oo7的substrate.runtime也会自动适应变更，因此客户端程序非常容易接入Substrate链！*

执行以上文件node balance.js 即可看到以下类似输出：

```
d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4f Alice FreeBalance= 100000000
```

FreeBalance= 100000000即是Alice的余额。



#### 调用接口，实现账户之间转账

发送交易需要涉及到用户账户私钥管理，oo7中提供了secretstore类作为私钥管理器。因此在发送交易之前需要先 提交账户私钥信息到本地管理器中，接口自动调用签名。

在这里我们调用substrate.calls.balances.transfer接口发起转账，新建transfer.js文件代码如下：

```
const substrate = require('oo7-substrate');
const nacl = require('tweetnacl');
let secretstore = substrate.secretStore();
substrate.setNodeUri(["ws://127.0.0.1:9944"]); //设置节点RPC ws协议地址

//将Alice放入私钥管理器，便于交易签名
let Alice_seed=substrate.stringToSeed('Alice') //Alice的seed
let Alice_KeyPair=nacl.sign.keyPair.fromSeed(Alice_seed)
secretstore._keys.push({ seed:Alice_seed, name:'Alice' });
secretstore._sync();

let Bob_KeyPair=nacl.sign.keyPair.fromSeed(substrate.stringToSeed('Bob'))//Bob的seed

//初始化oo7环境
substrate.runtimeUp.then(() => {
    substrate.calls.balances.transfer(Bob_KeyPair.publicKey,100).tie((data) => {
        //发送交易
        substrate.post({
            sender: Alice_KeyPair.publicKey,
            call: data,
        }).tie((msg) => {
            console.log(msg);
        });
    })
})
```

执行transfer.js，即可看到类似下面信息:

```
{ sending: true }
ready
{ broadcast: [ 'Qmcbbyx5zJuHSp2gmA9E6WjJirvj3pFdFUYSFQBpjsQBKJ' ] }
{ finalised:
   '0x4cf58f2373d63d591533e88b1c53c4bbbb2b53b75162dce4f9cc7151c3289e01' }
```

以上`sending`：表明交易已被签名，`broadcast`：已被广播，`finalised`：交易已被区块链处理。此时，我们再去执行`node balance.js`查询Alice到余额，返回结果中可以看到Alice余额与转账之前已发生变化。

### 总结

本课中，我们学习了利用oo7与Substrate链API进行交互，实现了余额查询和转账功能。更进一步学习，可以利用API实时获取Substrate链的所有信息，搭建一个区块链浏览器。







