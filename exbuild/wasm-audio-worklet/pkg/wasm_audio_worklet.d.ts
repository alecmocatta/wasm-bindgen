/* tslint:disable */
/* eslint-disable */
/**
* @returns {Promise<void>}
*/
export function web_main(): Promise<void>;
/**
*/
export class WasmAudioProcessor {
  free(): void;
/**
* @param {Float32Array} buf
* @returns {boolean}
*/
  process(buf: Float32Array): boolean;
/**
* @returns {number}
*/
  pack(): number;
/**
* @param {number} val
* @returns {WasmAudioProcessor}
*/
  static unpack(val: number): WasmAudioProcessor;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly web_main: () => number;
  readonly __wbg_wasmaudioprocessor_free: (a: number) => void;
  readonly wasmaudioprocessor_process: (a: number, b: number, c: number, d: number) => number;
  readonly wasmaudioprocessor_pack: (a: number) => number;
  readonly wasmaudioprocessor_unpack: (a: number) => number;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbe3e89d02573744d: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h12106c24d8c64dda: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h3ca30edae8027c3d: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput, maybe_memory?: WebAssembly.Memory): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>, maybe_memory?: WebAssembly.Memory): Promise<InitOutput>;
