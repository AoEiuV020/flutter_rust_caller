// WASM 加载器 - 使用 wasm-bindgen 生成的 flutter_rust_caller.js

/**
 * 初始化 WASM 模块并设置全局函数
 */
async function initWasm() {
  try {
    console.log('[WASM] Starting initialization...');
    
    // 动态导入 wasm-bindgen 生成的 JavaScript 模块
    // flutter_rust_caller.js 导出 default 初始化函数和具名导出函数
    const wasmModule = await import('./prebuild/flutter_rust_caller.js');
    
    // 调用默认导出的初始化函数，加载和初始化 WASM
    // 它会自动从 import.meta.url 相对路径加载 flutter_rust_caller_bg.wasm
    const wasmInit = wasmModule.default;
    await wasmInit();
    
    console.log('[WASM] Module loaded and initialized successfully');

    // 获取导出的 Rust 函数
    const rustCallWasm = wasmModule.rust_call_wasm;
    const rustCallAsyncWasm = wasmModule.rust_call_async_wasm;

    // 在全局 window 上设置函数
    
    /**
     * 同步调用 Rust 函数
     * @param {string} method - 要调用的方法名
     * @param {string} paramJSON - JSON 格式的参数
     * @returns {string} - JSON 格式的返回值
     */
    window.rust_call = (method, paramJSON) => {
      try {
        const result = rustCallWasm(method, paramJSON);
        return result;
      } catch (error) {
        console.error(`[WASM] Error calling rust_call(${method}):`, error);
        throw error;
      }
    };

    /**
     * 异步调用 Rust 函数
     * @param {string} method - 要调用的方法名
     * @param {string} paramJSON - JSON 格式的参数
     * @returns {Promise<string>} - 返回 Promise，resolve 为 JSON 格式的返回值
     */
    window.rust_call_async = async (method, paramJSON) => {
      try {
        const result = await rustCallAsyncWasm(method, paramJSON);
        return result;
      } catch (error) {
        console.error(`[WASM] Error calling rust_call_async(${method}):`, error);
        throw error;
      }
    };

    // 设置全局变量标记 WASM 已准备好
    window.rustWasmReady = true;
    
    console.log('[WASM] Global functions registered: window.rust_call, window.rust_call_async');
    console.log('[WASM] Initialization complete');
    
    // 触发自定义事件，通知 Flutter 应用 WASM 已准备好
    window.dispatchEvent(new CustomEvent('wasmReady', { detail: { ready: true } }));
    
  } catch (error) {
    console.error('[WASM] Initialization failed:', error);
    window.rustWasmReady = false;
    throw error;
  }
}

// 在 DOM 加载完成后初始化 WASM
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => {
    initWasm().catch(error => {
      console.error('[WASM] Failed to initialize WASM:', error);
    });
  });
} else {
  // DOM 已加载，直接初始化
  initWasm().catch(error => {
    console.error('[WASM] Failed to initialize WASM:', error);
  });
}
