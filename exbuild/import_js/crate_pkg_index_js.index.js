"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunk"] = self["webpackChunk"] || []).push([["crate_pkg_index_js"],{

/***/ "./crate/pkg/index.js":
/*!****************************!*\
  !*** ./crate/pkg/index.js ***!
  \****************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_log_2666fc61f873cb3f: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_2666fc61f873cb3f),\n/* harmony export */   __wbg_name_cadebad06a1e9644: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_name_cadebad06a1e9644),\n/* harmony export */   __wbg_new_1d8e69622c9d87d1: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_1d8e69622c9d87d1),\n/* harmony export */   __wbg_number_e11e77d8e43d13ab: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_number_e11e77d8e43d13ab),\n/* harmony export */   __wbg_render_c07f740dbf5475f1: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_render_c07f740dbf5475f1),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   __wbg_setnumber_e1ff542a4d0ae94a: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setnumber_e1ff542a4d0ae94a),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw),\n/* harmony export */   run: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.run)\n/* harmony export */ });\n/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./index_bg.wasm */ \"./crate/pkg/index_bg.wasm\");\n/* harmony import */ var _index_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.js */ \"./crate/pkg/index_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.__wbindgen_start();\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///./crate/pkg/index.js?");

/***/ }),

/***/ "./crate/pkg/index_bg.js":
/*!*******************************!*\
  !*** ./crate/pkg/index_bg.js ***!
  \*******************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_log_2666fc61f873cb3f: () => (/* binding */ __wbg_log_2666fc61f873cb3f),\n/* harmony export */   __wbg_name_cadebad06a1e9644: () => (/* binding */ __wbg_name_cadebad06a1e9644),\n/* harmony export */   __wbg_new_1d8e69622c9d87d1: () => (/* binding */ __wbg_new_1d8e69622c9d87d1),\n/* harmony export */   __wbg_number_e11e77d8e43d13ab: () => (/* binding */ __wbg_number_e11e77d8e43d13ab),\n/* harmony export */   __wbg_render_c07f740dbf5475f1: () => (/* binding */ __wbg_render_c07f740dbf5475f1),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   __wbg_setnumber_e1ff542a4d0ae94a: () => (/* binding */ __wbg_setnumber_e1ff542a4d0ae94a),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw),\n/* harmony export */   run: () => (/* binding */ run)\n/* harmony export */ });\n/* harmony import */ var _snippets_import_js_614d6b8c410ac4f9_defined_in_js_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./snippets/import_js-614d6b8c410ac4f9/defined-in-js.js */ \"./crate/pkg/snippets/import_js-614d6b8c410ac4f9/defined-in-js.js\");\n/* module decorator */ module = __webpack_require__.hmd(module);\n/* provided dependency */ var TextDecoder = __webpack_require__(/*! text-encoding */ \"../../node_modules/text-encoding/index.js\")[\"TextDecoder\"];\n/* provided dependency */ var TextEncoder = __webpack_require__(/*! text-encoding */ \"../../node_modules/text-encoding/index.js\")[\"TextEncoder\"];\n\n\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(128).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 132) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n        if (ret.read !== arg.length) throw new Error('failed to pass whole string');\n        offset += ret.written;\n        ptr = realloc(ptr, len, offset, 1) >>> 0;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nfunction logError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        let error = (function () {\n            try {\n                return e instanceof Error ? `${e.message}\\n\\nStack:\\n${e.stack}` : e.toString();\n            } catch(_) {\n                return \"<failed to stringify thrown value>\";\n            }\n        }());\n        console.error(\"wasm-bindgen: imported JS function that was not marked as `catch` threw an error:\", error);\n        throw e;\n    }\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction _assertNum(n) {\n    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);\n}\n/**\n*/\nfunction run() {\n    wasm.run();\n}\n\nfunction __wbg_name_cadebad06a1e9644() { return logError(function (arg0) {\n    const ret = (0,_snippets_import_js_614d6b8c410ac4f9_defined_in_js_js__WEBPACK_IMPORTED_MODULE_0__.name)();\n    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n    const len1 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len1;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr1;\n}, arguments) };\n\nfunction __wbg_new_1d8e69622c9d87d1() { return logError(function () {\n    const ret = new _snippets_import_js_614d6b8c410ac4f9_defined_in_js_js__WEBPACK_IMPORTED_MODULE_0__.MyClass();\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_number_e11e77d8e43d13ab() { return logError(function (arg0) {\n    const ret = getObject(arg0).number;\n    _assertNum(ret);\n    return ret;\n}, arguments) };\n\nfunction __wbg_setnumber_e1ff542a4d0ae94a() { return logError(function (arg0, arg1) {\n    const ret = getObject(arg0).number = arg1 >>> 0;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_render_c07f740dbf5475f1() { return logError(function (arg0, arg1) {\n    const ret = getObject(arg1).render();\n    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n    const len1 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len1;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr1;\n}, arguments) };\n\nfunction __wbg_log_2666fc61f873cb3f() { return logError(function (arg0, arg1) {\n    console.log(getStringFromWasm0(arg0, arg1));\n}, arguments) };\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\n\n\n//# sourceURL=webpack:///./crate/pkg/index_bg.js?");

/***/ }),

/***/ "./crate/pkg/snippets/import_js-614d6b8c410ac4f9/defined-in-js.js":
/*!************************************************************************!*\
  !*** ./crate/pkg/snippets/import_js-614d6b8c410ac4f9/defined-in-js.js ***!
  \************************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   MyClass: () => (/* binding */ MyClass),\n/* harmony export */   name: () => (/* binding */ name)\n/* harmony export */ });\nfunction name() {\n    return 'Rust';\n}\n\nclass MyClass {\n    constructor() {\n        this._number = 42;\n    }\n\n    get number() {\n        return this._number;\n    }\n\n    set number(n) {\n        return this._number = n;\n    }\n\n    render() {\n        return `My number is: ${this.number}`;\n    }\n}\n\n\n//# sourceURL=webpack:///./crate/pkg/snippets/import_js-614d6b8c410ac4f9/defined-in-js.js?");

/***/ }),

/***/ "./crate/pkg/index_bg.wasm":
/*!*********************************!*\
  !*** ./crate/pkg/index_bg.wasm ***!
  \*********************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./index_bg.js */ \"./crate/pkg/index_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"dbf001f5aaaa925cd15b\", {\n\t\"./index_bg.js\": {\n\t\t\"__wbg_name_cadebad06a1e9644\": WEBPACK_IMPORTED_MODULE_0.__wbg_name_cadebad06a1e9644,\n\t\t\"__wbg_new_1d8e69622c9d87d1\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_1d8e69622c9d87d1,\n\t\t\"__wbg_number_e11e77d8e43d13ab\": WEBPACK_IMPORTED_MODULE_0.__wbg_number_e11e77d8e43d13ab,\n\t\t\"__wbg_setnumber_e1ff542a4d0ae94a\": WEBPACK_IMPORTED_MODULE_0.__wbg_setnumber_e1ff542a4d0ae94a,\n\t\t\"__wbg_render_c07f740dbf5475f1\": WEBPACK_IMPORTED_MODULE_0.__wbg_render_c07f740dbf5475f1,\n\t\t\"__wbg_log_2666fc61f873cb3f\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_2666fc61f873cb3f,\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw,\n\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref\n\t}\n});\n\n//# sourceURL=webpack:///./crate/pkg/index_bg.wasm?");

/***/ })

}]);