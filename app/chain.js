const substrate = require('oo7-substrate');

substrate.setNodeUri(["ws://127.0.0.1:9944"]); //设置节点RPC ws协议地址
//初始化oo7环境
substrate.runtimeUp.then(() => {
    substrate.chain.height.tie(height => { //订阅链的块高度变更
        console.log(`new block by block number: ${height}`);
      });
})



