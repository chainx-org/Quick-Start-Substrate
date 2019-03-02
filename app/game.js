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
    let lucky=1
    substrate.calls.game.play(lucky).tie((data) => {
    //发送交易
        substrate.post({
            sender: Alice_KeyPair.publicKey,
            call: data,
        }).tie((msg) => {
            console.log(msg);
        });
    })
})

