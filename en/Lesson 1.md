# Lession 1: Transfer via blockchain API

### Introduction

RPC is one way of providing services to external world in blockchain. We could use RPC call to interact with the chain. Substrate RPC is highly abstract and only contains four modules, but it has covered all the functions. Thanks to the remarkable metadata machanism, all changes in the chain will be passed to the client automatically.

[oo7-substrate](https://github.com/paritytech/oo7/tree/master/packages/oo7-substrate) is a package focusing on providing the substrate related functionalities, which encapsulates the metadata of substrate so that the client could merely focus on the functions and don't need to pay any attention to the logic of blockchain itself.

In this lession we'll learn how to get information of the chain built in previous lession [Lesson 0](Lesson 0.md) and transfer between accounts via oo7-substrate.

Notes:

- All the instructions are operated on Ubuntu 16.04, other operating systems are not guranteed to behave exactly shown in this lession.
- Code used in this lession can be found at [app](../app/) directory.

### Basics

Excerpt from [substrate.readme.io](https://substrate.readme.io/v1.0.0/docs#section-core-datatypes):

- `Header`: a type which is representative (cryptographically or otherwise) of all information relevant to a block. It includes the parent hash, the storage root and the extrinsics trie root, the digest and a block number.
- `Extrinsic`: a type to represent a single piece of data external to the blockchain that is recognised by the blockchain. This typically involves one or more signatures, and some sort of encoded instruction (e.g. for transferring ownership of funds or calling into a smart contract).
- `Block`: essentially just a combination of `Header` and a series of `Extrinsic`s, together with a specification of the hashing algorithm to be used.

### Operating steps

#### Install NodeJs

Refer to [NodeJs Website](https://nodejs.org/en/) to install NodeJs and ensure:

- node > v10.10.0
- npm > 6.4.1

#### Install oo7 dependencies

Let's create a new directory called `app` and install oo7-substrate package:

```bash
mkdir app
cd app
npm install oo7-substrate
```

#### Get info from blockchain via APIs

Create new a javascript file called `chain.js` and copy all the snippets below into it. We'll use oo7 to connect to the local testnet and subscribe the information of blockchain height.

```javascript
const substrate = require('oo7-substrate');

substrate.setNodeUri(["ws://127.0.0.1:9944"]); // Set websocket address of node PRC
// Initialize oo7 environment
substrate.runtimeUp.then(() => {
    substrate.chain.height.tie(height => { // Subscribe the changes of blockchain height
        console.log(`new block by block number: ${height}`);
      });
})
```

Run `chain.js`, you'll see a output stream like this:

```javascript
...
new block by block number: 220
new block by block number: 221
new block by block number: 222
new block by block number: 223
```

That means the client has connected to the testnet and monitored the changes of blockchain successfully.

The object `substrate.chain` also contains other subscribe interfaces, such as `Header`、`Hash` and `Block`. Try it by yourself!

#### Check account balance

Alice and Bob are endowed some coins by default in the local testnet. Next, we'll implement a new functionality to check account balance based on `substrate.calls`.

Create a new file called `balance.js` and copy all the snippets into it:

```javascript
const substrate = require('oo7-substrate');
const nacl = require('tweetnacl');
substrate.setNodeUri(["ws://127.0.0.1:9944"]); // Set websocket address of node PRC

let Alice_KeyPair=nacl.sign.keyPair.fromSeed(substrate.stringToSeed('Alice'))//Alice的seed

// Initialize oo7 environment
substrate.runtimeUp.then(() => {
    substrate.runtime.balances.freeBalance(Alice_KeyPair.publicKey).then((v) => {
        console.log(substrate.bytesToHex(Alice_KeyPair.publicKey),'Alice FreeBalance=' ,v.toString())
    })
})
```

It's notable that `substrate.runtime` is adaptive to the changes of runtime of chain. When runtime on chain changes, `substrate.runtime` will update automatically, making the client easier to interact with substrate chain.

You'll see the output similar to the following content by running this command `node balance.js`:

```bash
d172a74cda4c865912c32ba0a80a57ae69abae410e5ccb59dee84e2f4432db4f Alice FreeBalance= 100000000
```

Obviously, the balance of Alice is `100000000`.

#### Transfer to another account

Sending transction involves the private key management of account. oo7 provides the class `secretstore` as the private key manager. In order to be able to sign the transaction automatically, we need to submit our private key to the local key manager before sending transction.

We use `substrate.calls.balances.transfer` to send a transfer transaction. Create a new file called `transfer.js`:

```javascript
const substrate = require('oo7-substrate');
const nacl = require('tweetnacl');
let secretstore = substrate.secretStore();
substrate.setNodeUri(["ws://127.0.0.1:9944"]); // Set websocket address of node PRC

// Add Alice to the secret storge so that we could use it to sign the transaction
let Alice_seed = substrate.stringToSeed('Alice') //Alice的seed
let Alice_KeyPair = nacl.sign.keyPair.fromSeed(Alice_seed)
secretstore._keys.push({
    seed: Alice_seed,
    name: 'Alice'
});
secretstore._sync();

let Bob_KeyPair = nacl.sign.keyPair.fromSeed(substrate.stringToSeed('Bob')) //Bob的seed

// Initialize oo7 environment
substrate.runtimeUp.then(() => {
    substrate.calls.balances.transfer(Bob_KeyPair.publicKey, 100).tie((data) => {
        // Sending transction
        substrate.post({
            sender: Alice_KeyPair.publicKey,
            call: data,
        }).tie((msg) => {
            console.log(msg);
        });
    })
})```

Runn `transfer.js` and you'll see the following messages:

```javascript
{ sending: true }
ready
{ broadcast: [ 'Qmcbbyx5zJuHSp2gmA9E6WjJirvj3pFdFUYSFQBpjsQBKJ' ] }
{ finalised:
   '0x4cf58f2373d63d591533e88b1c53c4bbbb2b53b75162dce4f9cc7151c3289e01' }
```

- `sending`：means the transaction has been signed.
- `broadcast`：means the transaction has been broadcasted.
- `finalised`：means the transaction bas been processed by the blockchain.

Now, we check the balance of Alice again by running `node balance.js`. We could clearly see the balance has been altered from the response.

### Conclusion

In this lesson, we learnt how to interact with Substrate chain via its API and oo7, implemented balance checking and transfering between accounts. If you want to learn more about substrate, you could definitely build a substrate explorer by grabing various information in real time based the APIs!
