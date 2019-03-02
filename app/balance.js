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
