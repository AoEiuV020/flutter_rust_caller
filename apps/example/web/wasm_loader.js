// WASM 加载器 - 初始化 Rust WASM 模块和全局函数

class WasmLoader {
  constructor() {
    this.wasmModule = null;
    this.ready = false;
  }

  /**
   * 初始化 WASM 模块并设置全局函数
   * @returns {Promise<void>}
   */
  async init() {
    try {
      console.log('[WASM] Starting initialization...');
      
      // 加载 WASM 文件
      const wasmPath = 'prebuild/libflutter_rust_caller.wasm';
      const response = await fetch(wasmPath);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch WASM file: ${wasmPath} (${response.status})`);
      }

      // 使用 WebAssembly.instantiateStreaming 加载
      const wasmBuffer = await response.arrayBuffer();
      const wasmModule = await WebAssembly.instantiate(wasmBuffer);
      this.wasmModule = wasmModule.instance;
      
      console.log('[WASM] Module loaded successfully');

      // 设置全局函数和变量
      this._setupGlobalFunctions();
      
      // 标记为就绪
      this.ready = true;
      window.rustWasmReady = true;
      
      console.log('[WASM] Initialization complete');
      
      // 触发自定义事件，通知 Flutter 应用 WASM 已准备好
      window.dispatchEvent(new CustomEvent('wasmReady', { detail: { ready: true } }));
      
    } catch (error) {
      console.error('[WASM] Initialization failed:', error);
      window.rustWasmReady = false;
      throw error;
    }
  }

  /**
   * 在全局 window 对象上设置 Rust 调用函数
   * @private
   */
  _setupGlobalFunctions() {
    const self = this;

    /**
     * 同步调用 Rust 函数
     * @param {string} method - 要调用的方法名
     * @param {string} paramJSON - JSON 格式的参数
     * @returns {string} - JSON 格式的返回值
     */
    window.rust_call = (method, paramJSON) => {
      if (!self.ready || !self.wasmModule) {
        throw new Error('WASM module is not ready');
      }
      
      try {
        // 调用 WASM 导出的函数
        // wasm-bindgen 为每个导出的函数生成 JavaScript 包装器
        const result = self.wasmModule.rust_call_wasm(method, paramJSON);
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
      if (!self.ready || !self.wasmModule) {
        throw new Error('WASM module is not ready');
      }
      
      try {
        // 异步调用 WASM 导出的函数
        const result = self.wasmModule.rust_call_async_wasm(method, paramJSON);
        // 如果是 Promise，等待其解决
        if (result instanceof Promise) {
          return await result;
        }
        // 否则包装在 Promise 中返回
        return result;
      } catch (error) {
        console.error(`[WASM] Error calling rust_call_async(${method}):`, error);
        throw error;
      }
    };

    console.log('[WASM] Global functions registered: window.rust_call, window.rust_call_async');
  }
}

// 创建全局加载器实例
window.wasmLoader = new WasmLoader();

// 在 DOM 加载完成后初始化 WASM
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => {
    window.wasmLoader.init().catch(error => {
      console.error('[WASM] Failed to initialize WASM:', error);
    });
  });
} else {
  // DOM 已加载，直接初始化
  window.wasmLoader.init().catch(error => {
    console.error('[WASM] Failed to initialize WASM:', error);
  });
}
