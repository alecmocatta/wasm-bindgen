"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunk"] = self["webpackChunk"] || []).push([["pkg_index_js"],{

/***/ "./pkg/index.js":
/*!**********************!*\
  !*** ./pkg/index.js ***!
  \**********************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_log_1746d5c75ec89963: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_1746d5c75ec89963),\n/* harmony export */   __wbg_log_2949f3d792510a3e: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_2949f3d792510a3e),\n/* harmony export */   __wbg_log_40ca944c2cefeec1: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_40ca944c2cefeec1),\n/* harmony export */   __wbg_log_5bb5f88f245d7762: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_5bb5f88f245d7762),\n/* harmony export */   __wbg_log_738c31fa5cdb7577: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_738c31fa5cdb7577),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   __wbindgen_number_new: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_number_new),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_string_new: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_string_new),\n/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw),\n/* harmony export */   run: () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.run)\n/* harmony export */ });\n/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./index_bg.wasm */ \"./pkg/index_bg.wasm\");\n/* harmony import */ var _index_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.js */ \"./pkg/index_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n_index_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.__wbindgen_start();\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///./pkg/index.js?");

/***/ }),

/***/ "./pkg/index_bg.js":
/*!*************************!*\
  !*** ./pkg/index_bg.js ***!
  \*************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_log_1746d5c75ec89963: () => (/* binding */ __wbg_log_1746d5c75ec89963),\n/* harmony export */   __wbg_log_2949f3d792510a3e: () => (/* binding */ __wbg_log_2949f3d792510a3e),\n/* harmony export */   __wbg_log_40ca944c2cefeec1: () => (/* binding */ __wbg_log_40ca944c2cefeec1),\n/* harmony export */   __wbg_log_5bb5f88f245d7762: () => (/* binding */ __wbg_log_5bb5f88f245d7762),\n/* harmony export */   __wbg_log_738c31fa5cdb7577: () => (/* binding */ __wbg_log_738c31fa5cdb7577),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   __wbindgen_number_new: () => (/* binding */ __wbindgen_number_new),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_string_new: () => (/* binding */ __wbindgen_string_new),\n/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw),\n/* harmony export */   run: () => (/* binding */ run)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\n/* provided dependency */ var TextDecoder = __webpack_require__(/*! text-encoding */ \"../../node_modules/text-encoding/index.js\")[\"TextDecoder\"];\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst heap = new Array(128).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');\n\n    heap[idx] = obj;\n    return idx;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 132) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n/**\n*/\nfunction run() {\n    wasm.run();\n}\n\nfunction logError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        let error = (function () {\n            try {\n                return e instanceof Error ? `${e.message}\\n\\nStack:\\n${e.stack}` : e.toString();\n            } catch(_) {\n                return \"<failed to stringify thrown value>\";\n            }\n        }());\n        console.error(\"wasm-bindgen: imported JS function that was not marked as `catch` threw an error:\", error);\n        throw e;\n    }\n}\n\nfunction __wbg_log_738c31fa5cdb7577() { return logError(function (arg0, arg1) {\n    console.log(getStringFromWasm0(arg0, arg1));\n}, arguments) };\n\nfunction __wbg_log_40ca944c2cefeec1() { return logError(function (arg0) {\n    console.log(arg0 >>> 0);\n}, arguments) };\n\nfunction __wbg_log_2949f3d792510a3e() { return logError(function (arg0, arg1, arg2, arg3) {\n    console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));\n}, arguments) };\n\nfunction __wbindgen_number_new(arg0) {\n    const ret = arg0;\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_string_new(arg0, arg1) {\n    const ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_log_5bb5f88f245d7762() { return logError(function (arg0) {\n    console.log(getObject(arg0));\n}, arguments) };\n\nfunction __wbg_log_1746d5c75ec89963() { return logError(function (arg0, arg1) {\n    console.log(getObject(arg0), getObject(arg1));\n}, arguments) };\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\n\n\n//# sourceURL=webpack:///./pkg/index_bg.js?");

/***/ }),

/***/ "./pkg/index_bg.wasm":
/*!***************************!*\
  !*** ./pkg/index_bg.wasm ***!
  \***************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./index_bg.js */ \"./pkg/index_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"b9d25e489cc3882df282\", {\n\t\"./index_bg.js\": {\n\t\t\"__wbg_log_738c31fa5cdb7577\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_738c31fa5cdb7577,\n\t\t\"__wbg_log_40ca944c2cefeec1\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_40ca944c2cefeec1,\n\t\t\"__wbg_log_2949f3d792510a3e\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_2949f3d792510a3e,\n\t\t\"__wbindgen_number_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_number_new,\n\t\t\"__wbindgen_string_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_new,\n\t\t\"__wbg_log_5bb5f88f245d7762\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_5bb5f88f245d7762,\n\t\t\"__wbg_log_1746d5c75ec89963\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_1746d5c75ec89963,\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw,\n\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref\n\t}\n});\n\n//# sourceURL=webpack:///./pkg/index_bg.wasm?");

/***/ })

}]);