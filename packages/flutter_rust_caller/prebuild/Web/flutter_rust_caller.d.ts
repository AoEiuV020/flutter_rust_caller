/* tslint:disable */
/* eslint-disable */
/**
 * 同步调用 Rust 函数
 * 接受方法名和 JSON 参数字符串，返回 JSON 结果字符串
 */
export function rust_call_wasm(method: string, param_json: string): string;
/**
 * 异步调用 Rust 函数
 * 接受方法名和 JSON 参数字符串，返回 Promise
 */
export function rust_call_async_wasm(method: string, param_json: string): Promise<string>;
/**
 * 设置全局 WASM 准备标志
 * 从 JavaScript 调用以表示 WASM 已加载并可用
 */
export function wasm_ready(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly rust_call: (a: number, b: number) => number;
  readonly rust_call_async_wasm: (a: number, b: number, c: number, d: number) => number;
  readonly rust_call_wasm: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly rust_free_string: (a: number) => void;
  readonly wasm_ready: () => void;
  readonly __wasm_bindgen_func_elem_794: (a: number, b: number, c: number) => void;
  readonly __wasm_bindgen_func_elem_793: (a: number, b: number) => void;
  readonly __wasm_bindgen_func_elem_346: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_export: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export2: (a: number, b: number) => number;
  readonly __wbindgen_export3: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
