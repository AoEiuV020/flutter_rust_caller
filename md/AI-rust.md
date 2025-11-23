claude haiku 4.5

1. 参考 flutter_go_caller/packages/flutter_go_caller/README.md 修改packages/flutter_rust_caller，改成使用prebuild中的预编译库，
1. 参考 flutter_go_caller/packages/flutter_go_caller/go 在 packages/flutter_rust_caller/rust 写上rust代码和编译脚本， 目标是生成预编译库放到 prebuild 目录下，包括makefile,
1. 写完用make编译，检查结果，修好了告诉我，

1. 不是我说的flutter_go_caller就在当前项目根目录下， 你看不到吗？

1. packages/flutter_rust_caller/rust/src/lib.rs 拆分一下，不要都挤在一个rs文件里，至少一个负责业务逻辑，一个负责ffi接口，尽量和go那边保持一致，再来个测试，
1. 不要管flutter代码层面，我自己处理， 
1. 参考 flutter_go_caller/packages/flutter_go_caller/README.md 修改packages/flutter_rust_caller，改成使用prebuild中的预编译库，这里是让你参考“平台集成指南”，修改flutter各端工程代码，依赖prebuild中的预编译库，

1. packages/flutter_rust_caller/rust/Makefile 太多flutter_rust_caller了，这里有变量PROJECT_NAME，尽量使用变量而不是硬编码， 
1. packages/flutter_rust_caller/rust/Makefile:63 你这个TARGET_MODE变量和CARGO_MODE完全一样， 是不是多余了，
1. 最终要执行make编译所有目标成功，好好参考flutter_go_caller/packages/flutter_go_caller/go/Makefile， 默认目标和目标名要完全一样， 
   目前 prebuild 目录下只有 macos-arm64 一个目标， 还差x86_64, 以及android/ios,
1. packages/flutter_rust_caller/rust/Makefile:196 web端生成wasm别偷懒， 你别管我怎么使用， 你先生成wasm到prebuild,

1. 好好对比 packages/flutter_rust_caller/rust/Makefile
flutter_go_caller/packages/flutter_go_caller/go/Makefile 任务本身尽可能完全一样，尤其是.PHONY和PLATFORM，
1. makefile里还是一大堆flutter_rust_caller，用变量替换掉，
1. makefile windows平台一大堆的dll.a这是什么东西， windows不是dll吗？ 好好参考 flutter_go_caller/packages/flutter_go_caller/go/Makefile 最终产物要完全一样， 除了go改名rust，

1. 别忽略错误啊， 编译安卓有问题你就先make android把问题解决了，而不是忽略错误， 
1. 参考 flutter_go_caller/script/docker/u2004.Dockerfile
flutter_go_caller/script/docker/docker-entrypoint.sh
flutter_go_caller/script/linux_docker.sh
flutter_go_caller/script/linux_multi_arch_docker.sh 参考我当前的rust版本， 写上rust linux docker编译脚本， 目标是能编译出linux的预编译库放到 prebuild 目录下，
1. 执行linux_multi_arch_docker编译linux版，

1. 谁告诉你的“macOS 上无法使用 Android NDK”，我电脑上的~/Library/Android/sdk/ndk本来就是mac版的，go那边都正常用的， 你好好参考go 的makefile，禁止跳过，

1. 不要在mac上编译linux版， 我说了执行linux_multi_arch_docker.sh脚本用docker编译linux版， 
1. mac真的没法编译android版so动态库吗？你再想想办法， 

1. 我听说可以指定linker实现android版编译， 你不知道吗， 
```
6. 配置 NDK 工具链
创建 .cargo/config 文件，告诉 Rust 如何调用 Android NDK 的编译器：
toml
[
target.armv7-linux-androideabi
]
linker = "~/Android/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi24-clang"

[
target.aarch64-linux-android
]
linker = "~/Android/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android24-clang"

[
target.i686-linux-android
]
linker = "~/Android/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android24-clang"

[
target.x86_64-linux-android
]
linker = "~/Android/ndk/25.1.8937393/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android24-clang"
注意：
把 ~/Android/ndk/25.1.8937393 替换成你的 NDK 实际路径。
24 是 Android API 级别，你可以根据需要修改（比如 21, 26 等）。
```

1. flutter_go_caller/packages/flutter_go_caller/README.md漏说了要把flutter_go_caller/packages/flutter_go_caller/ios/Classes/flutter_go_caller.c
flutter_go_caller/packages/flutter_go_caller/macos/Classes/flutter_go_caller.c自动生成的include代码删除， 
你在go的readme补上说明， rust这边追加处理一下， 
1. 你到最后也没运行script/linux_multi_arch_docker.sh， 我说过几遍了？运行脚本使用docker生成linux预编译库， 


1. wasm靠谱？我听说需要wasm_bindgen你都没写，
1. 我最终需要的是js全局添加函数 rust_call/rust_call_async，添加变量rustWasmReady，以便 packages/flutter_rust_caller/lib/src/rust_web.dart 使用，你看情况添加wasm导出，最好单独一个文件，
1. 你看情况调整 apps/example/web/index.html， 加载apps/example/web/prebuild/libflutter_rust_caller.wasm并创建这三个变量，

1. 你不要管测试， 你测不了， 写完了让我测试，
1. 报错了， wasm_loader.js:117 [WASM] Failed to initialize WASM: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
(anonymous)	@	wasm_loader.js:117
Promise.catch		
(anonymous)	@	wasm_loader.js:116

1. 报错， 
[WASM] Starting initialization...
wasm_loader.js:45 [WASM] Initialization failed: TypeError: WebAssembly.instantiate(): Import #0 "__wbindgen_placeholder__": module is not an object or function
init	@	wasm_loader.js:45
await in init		
(anonymous)	@	wasm_loader.js:116
wasm_loader.js:117 [WASM] Failed to initialize WASM: TypeError: WebAssembly.instantiate(): Import #0 "__wbindgen_placeholder__": module is not an object or function
(anonymous)	@	wasm_loader.js:117
Promise.catch		
(anonymous)	@	wasm_loader.js:116

1. 还是一堆的报错， 
[WASM] Starting initialization...
wasm_loader.js:13 [WASM] Module loaded successfully
wasm_loader.js:56 [WASM] Global functions registered: window.rust_call, window.rust_call_async
wasm_loader.js:57 [WASM] Initialization complete
wasm_loader.js:27 [WASM] Error calling rust_call(Increase): TypeError: Cannot read properties of undefined (reading '__wbindgen_add_to_stack_pointer')
    at Module.rust_call_wasm (flutter_rust_caller.js:170:14)
    at window.rust_call (wasm_loader.js:24:35)
    at testIncrease (wasm_test.html:200:31)
    at HTMLButtonElement.onclick (wasm_test.html:114:67)
wasm_loader.js:27 [WASM] Error calling rust_call(Sum): TypeError: Cannot read properties of undefined (reading '__wbindgen_add_to_stack_pointer')
    at Module.rust_call_wasm (flutter_rust_caller.js:170:14)
    at window.rust_call (wasm_loader.js:24:35)
    at testSum (wasm_test.html:216:31)
    at HTMLButtonElement.onclick (wasm_test.html:125:59)
wasm_loader.js:27 [WASM] Error calling rust_call(SumLongRunning): TypeError: Cannot read properties of undefined (reading '__wbindgen_add_to_stack_pointer')
    at Module.rust_call_wasm (flutter_rust_caller.js:170:14)
    at window.rust_call (wasm_loader.js:24:35)
    at testSumLongRunning (wasm_test.html:232:31)
    at HTMLButtonElement.onclick (wasm_test.html:137:75)
window.rust_call	@	wasm_loader.js:27
testSumLongRunning	@	wasm_test.html:232
onclick	@	wasm_test.html:137
1. 不是，我寻思现在wasm-bindgen已经生成了js绑定代码， 就不需要你那个apps/example/web/wasm_loader.js了吧，至少应该需要完全重写使用上apps/example/web/prebuild/flutter_rust_caller.js吧？

1. 异步调用还是报错了， 
[WASM] Starting initialization...
wasm_loader.js:19 [WASM] Module loaded and initialized successfully
wasm_loader.js:67 [WASM] Global functions registered: window.rust_call, window.rust_call_async
wasm_loader.js:68 [WASM] Initialization complete
wasm_loader.js:38 [WASM] Error calling rust_call(SumLongRunning): RuntimeError: unreachable
    at flutter_rust_caller_bg.wasm:0xfbc2
    at flutter_rust_caller_bg.wasm:0xf1ca
    at flutter_rust_caller_bg.wasm:0xead6
    at flutter_rust_caller_bg.wasm:0xfba9
    at flutter_rust_caller_bg.wasm:0xf8cf
    at flutter_rust_caller_bg.wasm:0xf8a2
    at flutter_rust_caller_bg.wasm:0xf819
    at flutter_rust_caller_bg.wasm:0x64ba
    at flutter_rust_caller_bg.wasm:0xcdae
    at rust_call_wasm (flutter_rust_caller.js:163:14)


1. 这样太丑了， 应该封装一个自用的sleep， 条件编译，wasm使用特殊处理， 其他平台就线程sleep， 最好单独一个文件封装这种sleep，

卡住了，undo，

1. packages/flutter_rust_caller/rust/src/business.rs:21 这里的thread::sleep在wasm无法使用， 添加一个单独的文件封装sleep函数， 条件编译wasm和非wasm两种实现，

1. sleep也已经回滚了， 全都回滚了，你检查一下重新实现， 
1. packages/flutter_rust_caller/rust/src/wasm.rs:41 为什么还是非要在导出的地方特殊处理sleep，我不是说了在business调用自己封装的跨平台sleep就好，
1. wasm.rs 又回滚了， 我希望不要修改这个文件， 你检查一下其他文件， 

1. packages/flutter_rust_caller/rust/src/call.rs:28 为啥， 我搞半天你直接在入口抛了异常，那还搞个屁啊， 
1. 如果确实 packages/flutter_rust_caller/rust/src/wasm.rs:22 这里不对那就看情况修改，不要有特殊处理就好， 该怎么实现异步好好考虑一下， 要通用， 

1. packages/flutter_rust_caller/rust/src/wasm.rs:33 你™我反复说了不要写死特殊处理，要通用， 你干嘛非要判断函数名在这里休息？我要的真正通用的异步处理， 
参考 flutter_go_caller/packages/flutter_go_caller/go/wasm_export.go:13

1. apps/example/web/wasm_loader.js:52 你™到底在想什么， 为什么就非要判断函数名特殊处理， 还在加载wasm的代码里模拟延迟到底有什么意义， 

1. packages/flutter_rust_caller/rust/src/call.rs:81 什么鬼东西你是非要特殊处理吗， 之前是特殊处理一个函数， 现在改成特殊处理锁哟函数这究竟解决了什么问题？
1. 好好参考 flutter_go_caller/packages/flutter_go_caller/go/call.go  flutter_go_caller/packages/flutter_go_caller/go/main.go ， SumLongRunning 本身就该是耗时操作， 不需要管同步异步调用， 进来就耗时，

1. 总之你纠结了那么久最终结论就是同步调用无法sleep？确实无法那就注释说清楚然后放弃， 不要瞎实现，
1. packages/flutter_rust_caller/rust/src/call.rs:81 不要两份execute, 非要特殊处理也只应该特殊处理一个SumLongRunning，async版只处理这一个函数， else直接调用非async版，然后非async版直接不处理这个函数， 

1. 对比 flutter_go_caller/packages/flutter_go_caller/go/Makefile ， rust这边的ios编译漏了模拟器版本， 产物参考 flutter_go_caller/packages/flutter_go_caller/prebuild/iOS ， 应该包含 ios-x86_64-sim ios-arm64-sim ios-arm64， 我说过了rust这边makefile要所有任务目标都和go那边一致，

1. packages/flutter_rust_caller/README.md 太简陋了， 参考 flutter_go_caller/packages/flutter_go_caller/README.md 来写， 
1. 两边的README都加上关于 ios/Classes/flutter_rust_caller.c
macos/Classes/flutter_rust_caller.c 删除include相关说明， 
