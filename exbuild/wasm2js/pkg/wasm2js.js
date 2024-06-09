import * as wasm from "./wasm2js_bg.wasm.js";
import { __wbg_set_wasm } from "./wasm2js_bg.js";
__wbg_set_wasm(wasm);
export * from "./wasm2js_bg.js";

wasm.__wbindgen_start();
