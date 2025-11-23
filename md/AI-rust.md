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

