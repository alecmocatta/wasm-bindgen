declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* Run entry point for the main thread.
	*/
	export function startup(): void;
	/**
	* A number evaluation struct
	*
	* This struct will be the main object which responds to messages passed to the
	* worker. It stores the last number which it was passed to have a state. The
	* statefulness is not is not required in this example but should show how
	* larger, more complex scenarios with statefulness can be set up.
	*/
	export class NumberEval {
	  free(): void;
	/**
	* Create new instance.
	* @returns {NumberEval}
	*/
	  static new(): NumberEval;
	/**
	* Check if a number is even and store it as last processed number.
	*
	* # Arguments
	*
	* * `number` - The number to be checked for being even/odd.
	* @param {number} number
	* @returns {boolean}
	*/
	  is_even(number: number): boolean;
	/**
	* Get last number that was checked - this method is added to work with
	* statefulness.
	* @returns {number}
	*/
	  get_last_number(): number;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_numbereval_free: (a: number) => void;
  readonly numbereval_new: () => number;
  readonly numbereval_is_even: (a: number, b: number) => number;
  readonly numbereval_get_last_number: (a: number) => number;
  readonly startup: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h043d3ab6c33d8319: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8aa780f54543f35b: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
