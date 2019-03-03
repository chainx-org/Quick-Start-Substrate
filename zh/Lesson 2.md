## Lesson 2 学习自定义Runtime模块

*欢迎转载，请务必注明出处*

### 前言

Runtime是理解和实现Substrate链关键，如果我们想要在Substrate基础上叠加各种各样业务功能，那么就需要对Runtime新增多个定制化模块的开发。

那么本课就从学习如何实现一个自定义Runtime模块。

从一个简单幸运数字游戏模块开始，体验Runtime模块对存储定义、接口定义、事件定义对完整过程。

幸运数字游戏需求：

- 用户参与游戏时指定一个小于10的幸运数字
- 如果当前块的随机数的第一个字节%10 = 用户的幸运数字，则该用户可以获得2*10个Coin的奖励，否则收取用户10个Coin。



*注意：本课程所有操作在Ubuntu 16操作系统下为示例，其他操作系统不保证预期完全一样*

### 背景知识

- 基本概念：
  - *Runtime* Substrate框架的内置核心组件和自定义功能组件的集装箱
  - *新模块* 必须具有存储定义、接口定义、事件定义三部分描述

### 操作步骤

#### 新建Runtime模块

在quick-start-substrate/runtime/src/下新建game.rs文件，进行编辑

#### 导入模块依赖

导入依赖，并继承`balances::Trait`和`system::Trait`

```rust
use codec::Encode;
use runtime_primitives::traits::As;
use runtime_primitives::traits::Hash;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait + balances::Trait {
    // TODO: Add other types and constants required configure this module.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

#### 定义存储结构

 decl_storage宏用来定义模块在链上的存储结构，在本模块中，作为示例，我们定义一个计数器Count，代表当前游戏已进行多少次，`Count`代表存储key到名字，`get(count)` 代表该存储项到getter，存储类型为u32。

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Game {
        // `get(count)` is the default getter which returns either the stored `u32`
        Count get(count) build(|_| 0u32): u32;
    }
}
```

#### 定义Event结构

Event代表的是Runtime模块在运行时的重要信息记录，将被存储到链上。

在本模块中，我们使用decl_event宏定义一个`Win`，记录用户获奖记录(用户账户，轮次)。

```rust
decl_event!(
    /// An event in this module.
    pub enum Event<T>  where
    <T as system::Trait>::AccountId
    {
        // Event `Win` is declared with a parameter of the type `AccountId` and `u32`
        Win(AccountId, u32),
    }
);
```



#### 定义模块接口

decl_module宏用于Runtime模块接口描述，在本模块中，我们定义用户参与游戏的接口`play`，接受一个u32的参数，代表用户的幸运数字。

```rust
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;
        // function that can be called by the external world as an extrinsics call
        // takes a parameter of the type `AccountId`, stores it and emits an event
        pub fn play(origin, lucky: u32) -> Result {
            //Todo
            Ok(())
        }
    }
}
```

#### 实现模块接口

`play` 接口是链对用户游戏请求的处理逻辑，是本游戏的重要部分。

在`play`部分，我们需要对用户的参数进行处理和校验，然后根据游戏规则，对其进行中奖判断，如果中奖则给予Coin奖励,并调用存储到``put`接口更新游戏轮次。将以下代码填上`play`函数体中。

```rust
 // TODO: You only need this if you want to check it was signed.
let who = ensure_signed(origin)?;
//make sure < 10
let catch:u8=(lucky as u8)%10;
//User must pays 10 coins
<balances::Module<T>>::decrease_free_balance(&who, As::sa(10))?;

// Then we check if the first byte of the hash is equal lucky
if (<system::Module<T>>::random_seed(), &who)
.using_encoded(<T as system::Trait>::Hashing::hash)
.using_encoded(|e| (e[0] % 10) == catch )
{
    //Catch Lucky , Double Coin Back
    <balances::Module<T>>::increase_free_balance_creating(&who, As::sa(20));

    //Update  count
    let mut count=<Count<T>>::get();
    count=count+1;
    <Count<T>>::put(count);
}

// here we are raising the Something event
Self::deposit_event(RawEvent::Win(who, lucky));
Ok(())
```

至此，模块自身的逻辑功能已经完成！

#### 集成到Runtime中

新模块开发完成之后，还需要将其添加到Runtime中。打开quick-start-substrate/runtime/src/lib.rs进行编辑，将以下代码添加进去

```rust
mod game;
......
impl game::Trait for Runtime {
    type Event = Event;
}
```

接着，需要将game模块添加到construct_runtime宏定义中

```rust
construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		......
		Game: game::{Module, Call, Storage, Event<T>},   //add this line
	}
);
```



至此，新模块已经大功告成了！

回到`quick-start-substrate/`目录执行`./build.sh`重新构建节点程序。

接着参考[Lesson 0]((https://github.com/chainx-org/Quick-Start-Substrate/blob/master/zh/Lesson%200.md) )中的操作步骤清除数据目录，重启测试网！

#### 客户端调用Game模块接口

参考[Lesson 1]((https://github.com/chainx-org/Quick-Start-Substrate/blob/master/zh/Lesson%201.md) ) 中`transfer.js`对交易的处理，新增`game.js`  文件，调用链上game模块的play接口，代码如下：

```javascript
let lucky=5
substrate.calls.game.play(lucky).tie((data) => {
//发送交易
    substrate.post({
        sender: Alice_KeyPair.publicKey,
        call: data,
    }).tie((msg) => {
    	console.log(msg);
    });
})
```

执行`node game.js`   发送交易，即可开始游戏！

### 总结

本课中，我们学习了如何使用`decl_storage`、`decl_module`、`decl_event`来定义新Runtime模块，示例了如何将新模块集成到Runtime中，并在客户端通过runtime api与链进行交互，进行游戏。

经过以上三个课程的学习，恭喜你已经完成了Substrate开发的入门，接下来将进入精通Substrate开发阶段的学习！

本课代码可在https://github.com/chainx-org/Quick-Start-Substrate/tree/master/quick-start-substrate 查看



