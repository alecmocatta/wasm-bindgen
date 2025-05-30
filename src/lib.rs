//! Runtime support for the `wasm-bindgen` tool
//!
//! This crate contains the runtime support necessary for `wasm-bindgen` the
//! attribute and tool. Crates pull in the `#[wasm_bindgen]` attribute through
//! this crate and this crate also provides JS bindings through the `JsValue`
//! interface.
//!
//! ## Features
//!
//! ### `enable-interning`
//!
//! Enables the internal cache for [`wasm_bindgen::intern`].
//!
//! This feature currently enables the `std` feature, meaning that it is not
//! compatible with `no_std` environments.
//!
//! ### `msrv` (default)
//!
//! Enables Rust language features that require a higher MSRV. Enabling this
//! feature on older compilers will NOT result in a compilation error, the newer
//! language features will simply not be used.
//!
//! When compiling with Rust v1.78 or later, this feature enables better error messages for invalid methods on structs and enums.
//!
//! ### `std` (default)
//!
//! Enabling this feature will make the crate depend on the Rust standard library.
//!
//! Disable this feature to use this crate in `no_std` environments.
//!
//! ### `strict-macro`
//!
//! All warnings the `#[wasm_bindgen]` macro emits are turned into hard errors.
//! This mainly affects unused attribute options.
//!
//! ### Deprecated features
//!
//! #### `serde-serialize`
//!
//! **Deprecated:** Use the [`serde-wasm-bindgen`](https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/) crate instead.
//!
//! Enables the `JsValue::from_serde` and `JsValue::into_serde` methods for
//! serializing and deserializing Rust types to and from JavaScript.
//!
//! #### `spans`
//!
//! **Deprecated:** This feature became a no-op in wasm-bindgen v0.2.20 (Sep 7, 2018).

#![no_std]
#![cfg_attr(wasm_bindgen_unstable_test_coverage, feature(coverage_attribute))]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(
    any(target_feature = "atomics", wasm_bindgen_unstable_test_coverage),
    feature(allow_internal_unstable),
    allow(internal_features)
)]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen/0.2")]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::mem;
use core::ops::{
    Add, BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub,
};
use core::ptr::NonNull;

use crate::convert::{FromWasmAbi, TryFromJsValue, WasmRet, WasmSlice};

macro_rules! externs {
    ($(#[$attr:meta])* extern "C" { $(fn $name:ident($($args:tt)*) -> $ret:ty;)* }) => (
        #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
        $(#[$attr])*
        extern "C" {
            $(fn $name($($args)*) -> $ret;)*
        }

        $(
            #[cfg(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))))]
            #[allow(unused_variables)]
            unsafe extern fn $name($($args)*) -> $ret {
                panic!("function not implemented on non-wasm32 targets")
            }
        )*
    )
}

/// A module which is typically glob imported.
///
/// ```
/// use wasm_bindgen::prelude::*;
/// ```
pub mod prelude {
    pub use crate::closure::Closure;
    pub use crate::JsCast;
    pub use crate::JsValue;
    pub use crate::UnwrapThrowExt;
    #[doc(hidden)]
    pub use wasm_bindgen_macro::__wasm_bindgen_class_marker;
    pub use wasm_bindgen_macro::wasm_bindgen;

    pub use crate::JsError;
}

pub use wasm_bindgen_macro::link_to;

pub mod closure;
pub mod convert;
pub mod describe;
mod externref;
mod link;

mod cast;
pub use crate::cast::{JsCast, JsObject};

mod cache;
pub use cache::intern::{intern, unintern};

#[doc(hidden)]
#[path = "rt/mod.rs"]
pub mod __rt;

/// Representation of an object owned by JS.
///
/// A `JsValue` doesn't actually live in Rust right now but actually in a table
/// owned by the `wasm-bindgen` generated JS glue code. Eventually the ownership
/// will transfer into Wasm directly and this will likely become more efficient,
/// but for now it may be slightly slow.
pub struct JsValue {
    idx: u32,
    _marker: PhantomData<*mut u8>, // not at all threadsafe
}

const JSIDX_OFFSET: u32 = 128; // keep in sync with js/mod.rs
const JSIDX_UNDEFINED: u32 = JSIDX_OFFSET;
const JSIDX_NULL: u32 = JSIDX_OFFSET + 1;
const JSIDX_TRUE: u32 = JSIDX_OFFSET + 2;
const JSIDX_FALSE: u32 = JSIDX_OFFSET + 3;
const JSIDX_RESERVED: u32 = JSIDX_OFFSET + 4;

impl JsValue {
    /// The `null` JS value constant.
    pub const NULL: JsValue = JsValue::_new(JSIDX_NULL);

    /// The `undefined` JS value constant.
    pub const UNDEFINED: JsValue = JsValue::_new(JSIDX_UNDEFINED);

    /// The `true` JS value constant.
    pub const TRUE: JsValue = JsValue::_new(JSIDX_TRUE);

    /// The `false` JS value constant.
    pub const FALSE: JsValue = JsValue::_new(JSIDX_FALSE);

    #[inline]
    const fn _new(idx: u32) -> JsValue {
        JsValue {
            idx,
            _marker: PhantomData,
        }
    }

    /// Creates a new JS value which is a string.
    ///
    /// The utf-8 string provided is copied to the JS heap and the string will
    /// be owned by the JS garbage collector.
    #[allow(clippy::should_implement_trait)] // cannot fix without breaking change
    #[inline]
    pub fn from_str(s: &str) -> JsValue {
        unsafe { JsValue::_new(__wbindgen_string_new(s.as_ptr(), s.len())) }
    }

    /// Creates a new JS value which is a number.
    ///
    /// This function creates a JS value representing a number (a heap
    /// allocated number) and returns a handle to the JS version of it.
    #[inline]
    pub fn from_f64(n: f64) -> JsValue {
        unsafe { JsValue::_new(__wbindgen_number_new(n)) }
    }

    /// Creates a new JS value which is a bigint from a string representing a number.
    ///
    /// This function creates a JS value representing a bigint (a heap
    /// allocated large integer) and returns a handle to the JS version of it.
    #[inline]
    pub fn bigint_from_str(s: &str) -> JsValue {
        unsafe { JsValue::_new(__wbindgen_bigint_from_str(s.as_ptr(), s.len())) }
    }

    /// Creates a new JS value which is a boolean.
    ///
    /// This function creates a JS object representing a boolean (a heap
    /// allocated boolean) and returns a handle to the JS version of it.
    #[inline]
    pub const fn from_bool(b: bool) -> JsValue {
        if b {
            JsValue::TRUE
        } else {
            JsValue::FALSE
        }
    }

    /// Creates a new JS value representing `undefined`.
    #[inline]
    pub const fn undefined() -> JsValue {
        JsValue::UNDEFINED
    }

    /// Creates a new JS value representing `null`.
    #[inline]
    pub const fn null() -> JsValue {
        JsValue::NULL
    }

    /// Creates a new JS symbol with the optional description specified.
    ///
    /// This function will invoke the `Symbol` constructor in JS and return the
    /// JS object corresponding to the symbol created.
    pub fn symbol(description: Option<&str>) -> JsValue {
        unsafe {
            match description {
                Some(description) => JsValue::_new(__wbindgen_symbol_named_new(
                    description.as_ptr(),
                    description.len(),
                )),
                None => JsValue::_new(__wbindgen_symbol_anonymous_new()),
            }
        }
    }

    /// Creates a new `JsValue` from the JSON serialization of the object `t`
    /// provided.
    ///
    /// **This function is deprecated**, due to [creating a dependency cycle in
    /// some circumstances][dep-cycle-issue]. Use [`serde-wasm-bindgen`] or
    /// [`gloo_utils::format::JsValueSerdeExt`] instead.
    ///
    /// [dep-cycle-issue]: https://github.com/rustwasm/wasm-bindgen/issues/2770
    /// [`serde-wasm-bindgen`]: https://docs.rs/serde-wasm-bindgen
    /// [`gloo_utils::format::JsValueSerdeExt`]: https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.JsValueSerdeExt.html
    ///
    /// This function will serialize the provided value `t` to a JSON string,
    /// send the JSON string to JS, parse it into a JS object, and then return
    /// a handle to the JS object. This is unlikely to be super speedy so it's
    /// not recommended for large payloads, but it's a nice to have in some
    /// situations!
    ///
    /// Usage of this API requires activating the `serde-serialize` feature of
    /// the `wasm-bindgen` crate.
    ///
    /// # Errors
    ///
    /// Returns any error encountered when serializing `T` into JSON.
    #[cfg(feature = "serde-serialize")]
    #[deprecated = "causes dependency cycles, use `serde-wasm-bindgen` or `gloo_utils::format::JsValueSerdeExt` instead"]
    pub fn from_serde<T>(t: &T) -> serde_json::Result<JsValue>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let s = serde_json::to_string(t)?;
        unsafe { Ok(JsValue::_new(__wbindgen_json_parse(s.as_ptr(), s.len()))) }
    }

    /// Invokes `JSON.stringify` on this value and then parses the resulting
    /// JSON into an arbitrary Rust value.
    ///
    /// **This function is deprecated**, due to [creating a dependency cycle in
    /// some circumstances][dep-cycle-issue]. Use [`serde-wasm-bindgen`] or
    /// [`gloo_utils::format::JsValueSerdeExt`] instead.
    ///
    /// [dep-cycle-issue]: https://github.com/rustwasm/wasm-bindgen/issues/2770
    /// [`serde-wasm-bindgen`]: https://docs.rs/serde-wasm-bindgen
    /// [`gloo_utils::format::JsValueSerdeExt`]: https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.JsValueSerdeExt.html
    ///
    /// This function will first call `JSON.stringify` on the `JsValue` itself.
    /// The resulting string is then passed into Rust which then parses it as
    /// JSON into the resulting value.
    ///
    /// Usage of this API requires activating the `serde-serialize` feature of
    /// the `wasm-bindgen` crate.
    ///
    /// # Errors
    ///
    /// Returns any error encountered when parsing the JSON into a `T`.
    #[cfg(feature = "serde-serialize")]
    #[deprecated = "causes dependency cycles, use `serde-wasm-bindgen` or `gloo_utils::format::JsValueSerdeExt` instead"]
    pub fn into_serde<T>(&self) -> serde_json::Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>,
    {
        unsafe {
            let ret = __wbindgen_json_serialize(self.idx);
            let s = String::from_abi(ret);
            serde_json::from_str(&s)
        }
    }

    /// Returns the `f64` value of this JS value if it's an instance of a
    /// number.
    ///
    /// If this JS value is not an instance of a number then this returns
    /// `None`.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        unsafe { __wbindgen_number_get(self.idx).join() }
    }

    /// Tests whether this JS value is a JS string.
    #[inline]
    pub fn is_string(&self) -> bool {
        unsafe { __wbindgen_is_string(self.idx) == 1 }
    }

    /// If this JS value is a string value, this function copies the JS string
    /// value into Wasm linear memory, encoded as UTF-8, and returns it as a
    /// Rust `String`.
    ///
    /// To avoid the copying and re-encoding, consider the
    /// `JsString::try_from()` function from [js-sys](https://docs.rs/js-sys)
    /// instead.
    ///
    /// If this JS value is not an instance of a string or if it's not valid
    /// utf-8 then this returns `None`.
    ///
    /// # UTF-16 vs UTF-8
    ///
    /// JavaScript strings in general are encoded as UTF-16, but Rust strings
    /// are encoded as UTF-8. This can cause the Rust string to look a bit
    /// different than the JS string sometimes. For more details see the
    /// [documentation about the `str` type][caveats] which contains a few
    /// caveats about the encodings.
    ///
    /// [caveats]: https://rustwasm.github.io/docs/wasm-bindgen/reference/types/str.html
    #[inline]
    pub fn as_string(&self) -> Option<String> {
        unsafe { FromWasmAbi::from_abi(__wbindgen_string_get(self.idx)) }
    }

    /// Returns the `bool` value of this JS value if it's an instance of a
    /// boolean.
    ///
    /// If this JS value is not an instance of a boolean then this returns
    /// `None`.
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        unsafe {
            match __wbindgen_boolean_get(self.idx) {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    }

    /// Tests whether this JS value is `null`
    #[inline]
    pub fn is_null(&self) -> bool {
        unsafe { __wbindgen_is_null(self.idx) == 1 }
    }

    /// Tests whether this JS value is `undefined`
    #[inline]
    pub fn is_undefined(&self) -> bool {
        unsafe { __wbindgen_is_undefined(self.idx) == 1 }
    }

    /// Tests whether the type of this JS value is `symbol`
    #[inline]
    pub fn is_symbol(&self) -> bool {
        unsafe { __wbindgen_is_symbol(self.idx) == 1 }
    }

    /// Tests whether `typeof self == "object" && self !== null`.
    #[inline]
    pub fn is_object(&self) -> bool {
        unsafe { __wbindgen_is_object(self.idx) == 1 }
    }

    /// Tests whether this JS value is an instance of Array.
    #[inline]
    pub fn is_array(&self) -> bool {
        unsafe { __wbindgen_is_array(self.idx) == 1 }
    }

    /// Tests whether the type of this JS value is `function`.
    #[inline]
    pub fn is_function(&self) -> bool {
        unsafe { __wbindgen_is_function(self.idx) == 1 }
    }

    /// Tests whether the type of this JS value is `bigint`.
    #[inline]
    pub fn is_bigint(&self) -> bool {
        unsafe { __wbindgen_is_bigint(self.idx) == 1 }
    }

    /// Applies the unary `typeof` JS operator on a `JsValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof)
    #[inline]
    pub fn js_typeof(&self) -> JsValue {
        unsafe { JsValue::_new(__wbindgen_typeof(self.idx)) }
    }

    /// Applies the binary `in` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/in)
    #[inline]
    pub fn js_in(&self, obj: &JsValue) -> bool {
        unsafe { __wbindgen_in(self.idx, obj.idx) == 1 }
    }

    /// Tests whether the value is ["truthy"].
    ///
    /// ["truthy"]: https://developer.mozilla.org/en-US/docs/Glossary/Truthy
    #[inline]
    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }

    /// Tests whether the value is ["falsy"].
    ///
    /// ["falsy"]: https://developer.mozilla.org/en-US/docs/Glossary/Falsy
    #[inline]
    pub fn is_falsy(&self) -> bool {
        unsafe { __wbindgen_is_falsy(self.idx) == 1 }
    }

    /// Get a string representation of the JavaScript object for debugging.
    fn as_debug_string(&self) -> String {
        unsafe {
            let mut ret = [0; 2];
            __wbindgen_debug_string(&mut ret, self.idx);
            let data = Vec::from_raw_parts(ret[0] as *mut u8, ret[1], ret[1]);
            String::from_utf8_unchecked(data)
        }
    }

    /// Compare two `JsValue`s for equality, using the `==` operator in JS.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Equality)
    #[inline]
    pub fn loose_eq(&self, other: &Self) -> bool {
        unsafe { __wbindgen_jsval_loose_eq(self.idx, other.idx) != 0 }
    }

    /// Applies the unary `~` JS operator on a `JsValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_NOT)
    #[inline]
    pub fn bit_not(&self) -> JsValue {
        unsafe { JsValue::_new(__wbindgen_bit_not(self.idx)) }
    }

    /// Applies the binary `>>>` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unsigned_right_shift)
    #[inline]
    pub fn unsigned_shr(&self, rhs: &Self) -> u32 {
        unsafe { __wbindgen_unsigned_shr(self.idx, rhs.idx) }
    }

    /// Applies the binary `/` JS operator on two `JsValue`s, catching and returning any `RangeError` thrown.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Division)
    #[inline]
    pub fn checked_div(&self, rhs: &Self) -> Self {
        unsafe { JsValue::_new(__wbindgen_checked_div(self.idx, rhs.idx)) }
    }

    /// Applies the binary `**` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Exponentiation)
    #[inline]
    pub fn pow(&self, rhs: &Self) -> Self {
        unsafe { JsValue::_new(__wbindgen_pow(self.idx, rhs.idx)) }
    }

    /// Applies the binary `<` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Less_than)
    #[inline]
    pub fn lt(&self, other: &Self) -> bool {
        unsafe { __wbindgen_lt(self.idx, other.idx) == 1 }
    }

    /// Applies the binary `<=` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Less_than_or_equal)
    #[inline]
    pub fn le(&self, other: &Self) -> bool {
        unsafe { __wbindgen_le(self.idx, other.idx) == 1 }
    }

    /// Applies the binary `>=` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Greater_than_or_equal)
    #[inline]
    pub fn ge(&self, other: &Self) -> bool {
        unsafe { __wbindgen_ge(self.idx, other.idx) == 1 }
    }

    /// Applies the binary `>` JS operator on the two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Greater_than)
    #[inline]
    pub fn gt(&self, other: &Self) -> bool {
        unsafe { __wbindgen_gt(self.idx, other.idx) == 1 }
    }

    /// Applies the unary `+` JS operator on a `JsValue`. Can throw.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)
    #[inline]
    pub fn unchecked_into_f64(&self) -> f64 {
        unsafe { __wbindgen_as_number(self.idx) }
    }
}

impl PartialEq for JsValue {
    /// Compares two `JsValue`s for equality, using the `===` operator in JS.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Strict_equality)
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { __wbindgen_jsval_eq(self.idx, other.idx) != 0 }
    }
}

impl PartialEq<bool> for JsValue {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == Some(*other)
    }
}

impl PartialEq<str> for JsValue {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        *self == JsValue::from_str(other)
    }
}

impl<'a> PartialEq<&'a str> for JsValue {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        <JsValue as PartialEq<str>>::eq(self, other)
    }
}

impl PartialEq<String> for JsValue {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        <JsValue as PartialEq<str>>::eq(self, other)
    }
}
impl<'a> PartialEq<&'a String> for JsValue {
    #[inline]
    fn eq(&self, other: &&'a String) -> bool {
        <JsValue as PartialEq<str>>::eq(self, other)
    }
}

macro_rules! forward_deref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for $t {
            type Output = <&'static $t as $imp>::Output;

            #[inline]
            fn $method(self) -> <&'static $t as $imp>::Output {
                $imp::$method(&self)
            }
        }
    };
}

macro_rules! forward_deref_binop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<$t> for &'a $t {
            type Output = <&'static $t as $imp<&'static $t>>::Output;

            #[inline]
            fn $method(self, other: $t) -> <&'static $t as $imp<&'static $t>>::Output {
                $imp::$method(self, &other)
            }
        }

        impl $imp<&$t> for $t {
            type Output = <&'static $t as $imp<&'static $t>>::Output;

            #[inline]
            fn $method(self, other: &$t) -> <&'static $t as $imp<&'static $t>>::Output {
                $imp::$method(&self, other)
            }
        }

        impl $imp<$t> for $t {
            type Output = <&'static $t as $imp<&'static $t>>::Output;

            #[inline]
            fn $method(self, other: $t) -> <&'static $t as $imp<&'static $t>>::Output {
                $imp::$method(&self, &other)
            }
        }
    };
}

impl Not for &JsValue {
    type Output = bool;

    /// Applies the `!` JS operator on a `JsValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT)
    #[inline]
    fn not(self) -> Self::Output {
        JsValue::is_falsy(self)
    }
}

forward_deref_unop!(impl Not, not for JsValue);

impl TryFrom<JsValue> for f64 {
    type Error = JsValue;

    /// Applies the unary `+` JS operator on a `JsValue`.
    /// Returns the numeric result on success, or the JS error value on error.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)
    #[inline]
    fn try_from(val: JsValue) -> Result<Self, Self::Error> {
        f64::try_from(&val)
    }
}

impl TryFrom<&JsValue> for f64 {
    type Error = JsValue;

    /// Applies the unary `+` JS operator on a `JsValue`.
    /// Returns the numeric result on success, or the JS error value on error.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)
    #[inline]
    fn try_from(val: &JsValue) -> Result<Self, Self::Error> {
        let jsval = unsafe { JsValue::_new(__wbindgen_try_into_number(val.idx)) };
        match jsval.as_f64() {
            Some(num) => Ok(num),
            None => Err(jsval),
        }
    }
}

impl Neg for &JsValue {
    type Output = JsValue;

    /// Applies the unary `-` JS operator on a `JsValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_negation)
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_neg(self.idx)) }
    }
}

forward_deref_unop!(impl Neg, neg for JsValue);

impl BitAnd for &JsValue {
    type Output = JsValue;

    /// Applies the binary `&` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_AND)
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_bit_and(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl BitAnd, bitand for JsValue);

impl BitOr for &JsValue {
    type Output = JsValue;

    /// Applies the binary `|` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_OR)
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_bit_or(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl BitOr, bitor for JsValue);

impl BitXor for &JsValue {
    type Output = JsValue;

    /// Applies the binary `^` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_XOR)
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_bit_xor(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl BitXor, bitxor for JsValue);

impl Shl for &JsValue {
    type Output = JsValue;

    /// Applies the binary `<<` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Left_shift)
    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_shl(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Shl, shl for JsValue);

impl Shr for &JsValue {
    type Output = JsValue;

    /// Applies the binary `>>` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Right_shift)
    #[inline]
    fn shr(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_shr(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Shr, shr for JsValue);

impl Add for &JsValue {
    type Output = JsValue;

    /// Applies the binary `+` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Addition)
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_add(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Add, add for JsValue);

impl Sub for &JsValue {
    type Output = JsValue;

    /// Applies the binary `-` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Subtraction)
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_sub(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Sub, sub for JsValue);

impl Div for &JsValue {
    type Output = JsValue;

    /// Applies the binary `/` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Division)
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_div(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Div, div for JsValue);

impl Mul for &JsValue {
    type Output = JsValue;

    /// Applies the binary `*` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Multiplication)
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_mul(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Mul, mul for JsValue);

impl Rem for &JsValue {
    type Output = JsValue;

    /// Applies the binary `%` JS operator on two `JsValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Remainder)
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        unsafe { JsValue::_new(__wbindgen_rem(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Rem, rem for JsValue);

impl<'a> From<&'a str> for JsValue {
    #[inline]
    fn from(s: &'a str) -> JsValue {
        JsValue::from_str(s)
    }
}

impl<T> From<*mut T> for JsValue {
    #[inline]
    fn from(s: *mut T) -> JsValue {
        JsValue::from(s as usize)
    }
}

impl<T> From<*const T> for JsValue {
    #[inline]
    fn from(s: *const T) -> JsValue {
        JsValue::from(s as usize)
    }
}

impl<T> From<NonNull<T>> for JsValue {
    #[inline]
    fn from(s: NonNull<T>) -> JsValue {
        JsValue::from(s.as_ptr() as usize)
    }
}

impl<'a> From<&'a String> for JsValue {
    #[inline]
    fn from(s: &'a String) -> JsValue {
        JsValue::from_str(s)
    }
}

impl From<String> for JsValue {
    #[inline]
    fn from(s: String) -> JsValue {
        JsValue::from_str(&s)
    }
}

impl TryFrom<JsValue> for String {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        match value.as_string() {
            Some(s) => Ok(s),
            None => Err(value),
        }
    }
}

impl TryFromJsValue for String {
    type Error = JsValue;

    fn try_from_js_value(value: JsValue) -> Result<Self, Self::Error> {
        match value.as_string() {
            Some(s) => Ok(s),
            None => Err(value),
        }
    }
}

impl From<bool> for JsValue {
    #[inline]
    fn from(s: bool) -> JsValue {
        JsValue::from_bool(s)
    }
}

impl<'a, T> From<&'a T> for JsValue
where
    T: JsCast,
{
    #[inline]
    fn from(s: &'a T) -> JsValue {
        s.as_ref().clone()
    }
}

impl<T> From<Option<T>> for JsValue
where
    JsValue: From<T>,
{
    #[inline]
    fn from(s: Option<T>) -> JsValue {
        match s {
            Some(s) => s.into(),
            None => JsValue::undefined(),
        }
    }
}

impl JsCast for JsValue {
    // everything is a `JsValue`!
    #[inline]
    fn instanceof(_val: &JsValue) -> bool {
        true
    }
    #[inline]
    fn unchecked_from_js(val: JsValue) -> Self {
        val
    }
    #[inline]
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        val
    }
}

impl AsRef<JsValue> for JsValue {
    #[inline]
    fn as_ref(&self) -> &JsValue {
        self
    }
}

macro_rules! numbers {
    ($($n:ident)*) => ($(
        impl PartialEq<$n> for JsValue {
            #[inline]
            fn eq(&self, other: &$n) -> bool {
                self.as_f64() == Some(f64::from(*other))
            }
        }

        impl From<$n> for JsValue {
            #[inline]
            fn from(n: $n) -> JsValue {
                JsValue::from_f64(n.into())
            }
        }
    )*)
}

numbers! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! big_numbers {
    (|$arg:ident|, $($n:ident = $handle:expr,)*) => ($(
        impl PartialEq<$n> for JsValue {
            #[inline]
            fn eq(&self, other: &$n) -> bool {
                self == &JsValue::from(*other)
            }
        }

        impl From<$n> for JsValue {
            #[inline]
            fn from($arg: $n) -> JsValue {
                unsafe { JsValue::_new($handle) }
            }
        }
    )*)
}

fn bigint_get_as_i64(v: &JsValue) -> Option<i64> {
    unsafe { __wbindgen_bigint_get_as_i64(v.idx).join() }
}

macro_rules! try_from_for_num64 {
    ($ty:ty) => {
        impl TryFrom<JsValue> for $ty {
            type Error = JsValue;

            #[inline]
            fn try_from(v: JsValue) -> Result<Self, JsValue> {
                bigint_get_as_i64(&v)
                    // Reinterpret bits; ABI-wise this is safe to do and allows us to avoid
                    // having separate intrinsics per signed/unsigned types.
                    .map(|as_i64| as_i64 as Self)
                    // Double-check that we didn't truncate the bigint to 64 bits.
                    .filter(|as_self| v == *as_self)
                    // Not a bigint or not in range.
                    .ok_or(v)
            }
        }
    };
}

try_from_for_num64!(i64);
try_from_for_num64!(u64);

macro_rules! try_from_for_num128 {
    ($ty:ty, $hi_ty:ty) => {
        impl TryFrom<JsValue> for $ty {
            type Error = JsValue;

            #[inline]
            fn try_from(v: JsValue) -> Result<Self, JsValue> {
                // Truncate the bigint to 64 bits, this will give us the lower part.
                let lo = match bigint_get_as_i64(&v) {
                    // The lower part must be interpreted as unsigned in both i128 and u128.
                    Some(lo) => lo as u64,
                    // Not a bigint.
                    None => return Err(v),
                };
                // Now we know it's a bigint, so we can safely use `>> 64n` without
                // worrying about a JS exception on type mismatch.
                let hi = v >> JsValue::from(64_u64);
                // The high part is the one we want checked against a 64-bit range.
                // If it fits, then our original number is in the 128-bit range.
                let hi = <$hi_ty>::try_from(hi)?;
                Ok(Self::from(hi) << 64 | Self::from(lo))
            }
        }
    };
}

try_from_for_num128!(i128, i64);
try_from_for_num128!(u128, u64);

big_numbers! {
    |n|,
    i64 = __wbindgen_bigint_from_i64(n),
    u64 = __wbindgen_bigint_from_u64(n),
    i128 = __wbindgen_bigint_from_i128((n >> 64) as i64, n as u64),
    u128 = __wbindgen_bigint_from_u128((n >> 64) as u64, n as u64),
}

// `usize` and `isize` have to be treated a bit specially, because we know that
// they're 32-bit but the compiler conservatively assumes they might be bigger.
// So, we have to manually forward to the `u32`/`i32` versions.
impl PartialEq<usize> for JsValue {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self == (*other as u32)
    }
}

impl From<usize> for JsValue {
    #[inline]
    fn from(n: usize) -> Self {
        Self::from(n as u32)
    }
}

impl PartialEq<isize> for JsValue {
    #[inline]
    fn eq(&self, other: &isize) -> bool {
        *self == (*other as i32)
    }
}

impl From<isize> for JsValue {
    #[inline]
    fn from(n: isize) -> Self {
        Self::from(n as i32)
    }
}

externs! {
    #[link(wasm_import_module = "__wbindgen_placeholder__")]
    extern "C" {
        fn __wbindgen_object_clone_ref(idx: u32) -> u32;
        fn __wbindgen_object_drop_ref(idx: u32) -> ();

        fn __wbindgen_string_new(ptr: *const u8, len: usize) -> u32;
        fn __wbindgen_number_new(f: f64) -> u32;
        fn __wbindgen_bigint_from_str(ptr: *const u8, len: usize) -> u32;
        fn __wbindgen_bigint_from_i64(n: i64) -> u32;
        fn __wbindgen_bigint_from_u64(n: u64) -> u32;
        fn __wbindgen_bigint_from_i128(hi: i64, lo: u64) -> u32;
        fn __wbindgen_bigint_from_u128(hi: u64, lo: u64) -> u32;
        fn __wbindgen_symbol_named_new(ptr: *const u8, len: usize) -> u32;
        fn __wbindgen_symbol_anonymous_new() -> u32;

        fn __wbindgen_externref_heap_live_count() -> u32;

        fn __wbindgen_is_null(idx: u32) -> u32;
        fn __wbindgen_is_undefined(idx: u32) -> u32;
        fn __wbindgen_is_symbol(idx: u32) -> u32;
        fn __wbindgen_is_object(idx: u32) -> u32;
        fn __wbindgen_is_array(idx: u32) -> u32;
        fn __wbindgen_is_function(idx: u32) -> u32;
        fn __wbindgen_is_string(idx: u32) -> u32;
        fn __wbindgen_is_bigint(idx: u32) -> u32;
        fn __wbindgen_typeof(idx: u32) -> u32;

        fn __wbindgen_in(prop: u32, obj: u32) -> u32;

        fn __wbindgen_is_falsy(idx: u32) -> u32;
        fn __wbindgen_as_number(idx: u32) -> f64;
        fn __wbindgen_try_into_number(idx: u32) -> u32;
        fn __wbindgen_neg(idx: u32) -> u32;
        fn __wbindgen_bit_and(a: u32, b: u32) -> u32;
        fn __wbindgen_bit_or(a: u32, b: u32) -> u32;
        fn __wbindgen_bit_xor(a: u32, b: u32) -> u32;
        fn __wbindgen_bit_not(idx: u32) -> u32;
        fn __wbindgen_shl(a: u32, b: u32) -> u32;
        fn __wbindgen_shr(a: u32, b: u32) -> u32;
        fn __wbindgen_unsigned_shr(a: u32, b: u32) -> u32;
        fn __wbindgen_add(a: u32, b: u32) -> u32;
        fn __wbindgen_sub(a: u32, b: u32) -> u32;
        fn __wbindgen_div(a: u32, b: u32) -> u32;
        fn __wbindgen_checked_div(a: u32, b: u32) -> u32;
        fn __wbindgen_mul(a: u32, b: u32) -> u32;
        fn __wbindgen_rem(a: u32, b: u32) -> u32;
        fn __wbindgen_pow(a: u32, b: u32) -> u32;
        fn __wbindgen_lt(a: u32, b: u32) -> u32;
        fn __wbindgen_le(a: u32, b: u32) -> u32;
        fn __wbindgen_ge(a: u32, b: u32) -> u32;
        fn __wbindgen_gt(a: u32, b: u32) -> u32;

        fn __wbindgen_number_get(idx: u32) -> WasmRet<Option<f64>>;
        fn __wbindgen_boolean_get(idx: u32) -> u32;
        fn __wbindgen_string_get(idx: u32) -> WasmSlice;
        fn __wbindgen_bigint_get_as_i64(idx: u32) -> WasmRet<Option<i64>>;

        fn __wbindgen_debug_string(ret: *mut [usize; 2], idx: u32) -> ();

        fn __wbindgen_throw(a: *const u8, b: usize) -> !;
        fn __wbindgen_rethrow(a: u32) -> !;
        fn __wbindgen_error_new(a: *const u8, b: usize) -> u32;

        fn __wbindgen_cb_drop(idx: u32) -> u32;

        fn __wbindgen_describe(v: u32) -> ();
        fn __wbindgen_describe_closure(a: u32, b: u32, c: u32) -> u32;

        fn __wbindgen_json_parse(ptr: *const u8, len: usize) -> u32;
        fn __wbindgen_json_serialize(idx: u32) -> WasmSlice;
        fn __wbindgen_jsval_eq(a: u32, b: u32) -> u32;
        fn __wbindgen_jsval_loose_eq(a: u32, b: u32) -> u32;

        fn __wbindgen_copy_to_typed_array(ptr: *const u8, len: usize, idx: u32) -> ();

        fn __wbindgen_uint8_array_new(ptr: *mut u8, len: usize) -> u32;
        fn __wbindgen_uint8_clamped_array_new(ptr: *mut u8, len: usize) -> u32;
        fn __wbindgen_uint16_array_new(ptr: *mut u16, len: usize) -> u32;
        fn __wbindgen_uint32_array_new(ptr: *mut u32, len: usize) -> u32;
        fn __wbindgen_biguint64_array_new(ptr: *mut u64, len: usize) -> u32;
        fn __wbindgen_int8_array_new(ptr: *mut i8, len: usize) -> u32;
        fn __wbindgen_int16_array_new(ptr: *mut i16, len: usize) -> u32;
        fn __wbindgen_int32_array_new(ptr: *mut i32, len: usize) -> u32;
        fn __wbindgen_bigint64_array_new(ptr: *mut i64, len: usize) -> u32;
        fn __wbindgen_float32_array_new(ptr: *mut f32, len: usize) -> u32;
        fn __wbindgen_float64_array_new(ptr: *mut f64, len: usize) -> u32;

        fn __wbindgen_array_new() -> u32;
        fn __wbindgen_array_push(array: u32, value: u32) -> ();

        fn __wbindgen_not(idx: u32) -> u32;

        fn __wbindgen_exports() -> u32;
        fn __wbindgen_memory() -> u32;
        fn __wbindgen_module() -> u32;
        fn __wbindgen_function_table() -> u32;
    }
}

impl Clone for JsValue {
    #[inline]
    fn clone(&self) -> JsValue {
        unsafe {
            let idx = __wbindgen_object_clone_ref(self.idx);
            JsValue::_new(idx)
        }
    }
}

impl core::fmt::Debug for JsValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "JsValue({})", self.as_debug_string())
    }
}

impl Drop for JsValue {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // We definitely should never drop anything in the stack area
            debug_assert!(self.idx >= JSIDX_OFFSET, "free of stack slot {}", self.idx);

            // Otherwise if we're not dropping one of our reserved values,
            // actually call the intrinsic. See #1054 for eventually removing
            // this branch.
            if self.idx >= JSIDX_RESERVED {
                __wbindgen_object_drop_ref(self.idx);
            }
        }
    }
}

impl Default for JsValue {
    fn default() -> Self {
        Self::UNDEFINED
    }
}

/// Wrapper type for imported statics.
///
/// This type is used whenever a `static` is imported from a JS module, for
/// example this import:
///
/// ```ignore
/// #[wasm_bindgen]
/// extern "C" {
///     static console: JsValue;
/// }
/// ```
///
/// will generate in Rust a value that looks like:
///
/// ```ignore
/// static console: JsStatic<JsValue> = ...;
/// ```
///
/// This type implements `Deref` to the inner type so it's typically used as if
/// it were `&T`.
#[cfg(feature = "std")]
#[deprecated = "use with `#[wasm_bindgen(thread_local_v2)]` instead"]
pub struct JsStatic<T: 'static> {
    #[doc(hidden)]
    pub __inner: &'static std::thread::LocalKey<T>,
}

#[cfg(feature = "std")]
#[allow(deprecated)]
#[cfg(not(target_feature = "atomics"))]
impl<T: FromWasmAbi + 'static> Deref for JsStatic<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.__inner.with(|ptr| &*(ptr as *const T)) }
    }
}

/// Wrapper type for imported statics.
///
/// This type is used whenever a `static` is imported from a JS module, for
/// example this import:
///
/// ```ignore
/// #[wasm_bindgen]
/// extern "C" {
///     #[wasm_bindgen(thread_local_v2)]
///     static console: JsValue;
/// }
/// ```
///
/// will generate in Rust a value that looks like:
///
/// ```ignore
/// static console: JsThreadLocal<JsValue> = ...;
/// ```
pub struct JsThreadLocal<T: 'static> {
    #[doc(hidden)]
    #[cfg(not(target_feature = "atomics"))]
    pub __inner: &'static __rt::LazyCell<T>,
    #[doc(hidden)]
    #[cfg(target_feature = "atomics")]
    pub __inner: fn() -> *const T,
}

impl<T> JsThreadLocal<T> {
    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        #[cfg(not(target_feature = "atomics"))]
        return f(self.__inner);
        #[cfg(target_feature = "atomics")]
        f(unsafe { &*(self.__inner)() })
    }
}

#[cold]
#[inline(never)]
#[deprecated(note = "renamed to `throw_str`")]
#[doc(hidden)]
pub fn throw(s: &str) -> ! {
    throw_str(s)
}

/// Throws a JS exception.
///
/// This function will throw a JS exception with the message provided. The
/// function will not return as the Wasm stack will be popped when the exception
/// is thrown.
///
/// Note that it is very easy to leak memory with this function because this
/// function, unlike `panic!` on other platforms, **will not run destructors**.
/// It's recommended to return a `Result` where possible to avoid the worry of
/// leaks.
#[cold]
#[inline(never)]
pub fn throw_str(s: &str) -> ! {
    unsafe {
        __wbindgen_throw(s.as_ptr(), s.len());
    }
}

/// Rethrow a JS exception
///
/// This function will throw a JS exception with the JS value provided. This
/// function will not return and the Wasm stack will be popped until the point
/// of entry of Wasm itself.
///
/// Note that it is very easy to leak memory with this function because this
/// function, unlike `panic!` on other platforms, **will not run destructors**.
/// It's recommended to return a `Result` where possible to avoid the worry of
/// leaks.
#[cold]
#[inline(never)]
pub fn throw_val(s: JsValue) -> ! {
    unsafe {
        let idx = s.idx;
        mem::forget(s);
        __wbindgen_rethrow(idx);
    }
}

/// Get the count of live `externref`s / `JsValue`s in `wasm-bindgen`'s heap.
///
/// ## Usage
///
/// This is intended for debugging and writing tests.
///
/// To write a test that asserts against unnecessarily keeping `anref`s /
/// `JsValue`s alive:
///
/// * get an initial live count,
///
/// * perform some series of operations or function calls that should clean up
///   after themselves, and should not keep holding onto `externref`s / `JsValue`s
///   after completion,
///
/// * get the final live count,
///
/// * and assert that the initial and final counts are the same.
///
/// ## What is Counted
///
/// Note that this only counts the *owned* `externref`s / `JsValue`s that end up in
/// `wasm-bindgen`'s heap. It does not count borrowed `externref`s / `JsValue`s
/// that are on its stack.
///
/// For example, these `JsValue`s are accounted for:
///
/// ```ignore
/// #[wasm_bindgen]
/// pub fn my_function(this_is_counted: JsValue) {
///     let also_counted = JsValue::from_str("hi");
///     assert!(wasm_bindgen::externref_heap_live_count() >= 2);
/// }
/// ```
///
/// While this borrowed `JsValue` ends up on the stack, not the heap, and
/// therefore is not accounted for:
///
/// ```ignore
/// #[wasm_bindgen]
/// pub fn my_other_function(this_is_not_counted: &JsValue) {
///     // ...
/// }
/// ```
pub fn externref_heap_live_count() -> u32 {
    unsafe { __wbindgen_externref_heap_live_count() }
}

#[doc(hidden)]
pub fn anyref_heap_live_count() -> u32 {
    externref_heap_live_count()
}

/// An extension trait for `Option<T>` and `Result<T, E>` for unwrapping the `T`
/// value, or throwing a JS error if it is not available.
///
/// These methods should have a smaller code size footprint than the normal
/// `Option::unwrap` and `Option::expect` methods, but they are specific to
/// working with Wasm and JS.
///
/// On non-wasm32 targets, defaults to the normal unwrap/expect calls.
///
/// # Example
///
/// ```
/// use wasm_bindgen::prelude::*;
///
/// // If the value is `Option::Some` or `Result::Ok`, then we just get the
/// // contained `T` value.
/// let x = Some(42);
/// assert_eq!(x.unwrap_throw(), 42);
///
/// let y: Option<i32> = None;
///
/// // This call would throw an error to JS!
/// //
/// //     y.unwrap_throw()
/// //
/// // And this call would throw an error to JS with a custom error message!
/// //
/// //     y.expect_throw("woopsie daisy!")
/// ```
pub trait UnwrapThrowExt<T>: Sized {
    #[cfg(feature = "std-unwrap")]
    #[track_caller]
    fn unwrap_throw(self) -> T;

    /// Unwrap this `Option` or `Result`, but instead of panicking on failure,
    /// throw an exception to JavaScript.
    #[cfg(not(feature = "std-unwrap"))]
    #[cfg_attr(
        any(
            debug_assertions,
            not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))
        ),
        track_caller
    )]
    fn unwrap_throw(self) -> T {
        if cfg!(all(
            debug_assertions,
            all(
                target_arch = "wasm32",
                any(target_os = "unknown", target_os = "none")
            )
        )) {
            let loc = core::panic::Location::caller();
            let msg = alloc::format!(
                "called `{}::unwrap_throw()` ({}:{}:{})",
                core::any::type_name::<Self>(),
                loc.file(),
                loc.line(),
                loc.column()
            );
            self.expect_throw(&msg)
        } else {
            self.expect_throw("called `unwrap_throw()`")
        }
    }

    #[cfg(feature = "std-unwrap")]
    #[track_caller]
    fn expect_throw(self, message: &str) -> T;

    /// Unwrap this container's `T` value, or throw an error to JS with the
    /// given message if the `T` value is unavailable (e.g. an `Option<T>` is
    /// `None`).
    #[cfg(not(feature = "std-unwrap"))]
    #[cfg_attr(
        any(
            debug_assertions,
            not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))
        ),
        track_caller
    )]
    fn expect_throw(self, message: &str) -> T;
}

impl<T> UnwrapThrowExt<T> for Option<T> {
    fn unwrap_throw(self) -> T {
        const MSG: &str = "called `Option::unwrap_throw()` on a `None` value";

        if cfg!(feature = "std-unwrap") {
            self.unwrap()
        } else if cfg!(all(
            target_arch = "wasm32",
            any(target_os = "unknown", target_os = "none")
        )) {
            if let Some(val) = self {
                val
            } else if cfg!(debug_assertions) {
                let loc = core::panic::Location::caller();
                let msg =
                    alloc::format!("{} ({}:{}:{})", MSG, loc.file(), loc.line(), loc.column(),);

                throw_str(&msg)
            } else {
                throw_str(MSG)
            }
        } else {
            self.expect(MSG)
        }
    }

    fn expect_throw(self, message: &str) -> T {
        if cfg!(feature = "std-unwrap") {
            self.expect(message)
        } else if cfg!(all(
            target_arch = "wasm32",
            any(target_os = "unknown", target_os = "none")
        )) {
            if let Some(val) = self {
                val
            } else if cfg!(debug_assertions) {
                let loc = core::panic::Location::caller();
                let msg = alloc::format!(
                    "{} ({}:{}:{})",
                    message,
                    loc.file(),
                    loc.line(),
                    loc.column(),
                );

                throw_str(&msg)
            } else {
                throw_str(message)
            }
        } else {
            self.expect(message)
        }
    }
}

impl<T, E> UnwrapThrowExt<T> for Result<T, E>
where
    E: core::fmt::Debug,
{
    fn unwrap_throw(self) -> T {
        const MSG: &str = "called `Result::unwrap_throw()` on an `Err` value";

        if cfg!(feature = "std-unwrap") {
            self.unwrap()
        } else if cfg!(all(
            target_arch = "wasm32",
            any(target_os = "unknown", target_os = "none")
        )) {
            match self {
                Ok(val) => val,
                Err(err) => {
                    if cfg!(debug_assertions) {
                        let loc = core::panic::Location::caller();
                        let msg = alloc::format!(
                            "{} ({}:{}:{}): {:?}",
                            MSG,
                            loc.file(),
                            loc.line(),
                            loc.column(),
                            err
                        );

                        throw_str(&msg)
                    } else {
                        throw_str(MSG)
                    }
                }
            }
        } else {
            self.expect(MSG)
        }
    }

    fn expect_throw(self, message: &str) -> T {
        if cfg!(feature = "std-unwrap") {
            self.expect(message)
        } else if cfg!(all(
            target_arch = "wasm32",
            any(target_os = "unknown", target_os = "none")
        )) {
            match self {
                Ok(val) => val,
                Err(err) => {
                    if cfg!(debug_assertions) {
                        let loc = core::panic::Location::caller();
                        let msg = alloc::format!(
                            "{} ({}:{}:{}): {:?}",
                            message,
                            loc.file(),
                            loc.line(),
                            loc.column(),
                            err
                        );

                        throw_str(&msg)
                    } else {
                        throw_str(message)
                    }
                }
            }
        } else {
            self.expect(message)
        }
    }
}

/// Returns a handle to this Wasm instance's `WebAssembly.Module`.
/// This is only available when the final Wasm app is built with
/// `--target no-modules` or `--target web`.
pub fn module() -> JsValue {
    unsafe { JsValue::_new(__wbindgen_module()) }
}

/// Returns a handle to this Wasm instance's `WebAssembly.Instance.prototype.exports`
pub fn exports() -> JsValue {
    unsafe { JsValue::_new(__wbindgen_exports()) }
}

/// Returns a handle to this Wasm instance's `WebAssembly.Memory`
pub fn memory() -> JsValue {
    unsafe { JsValue::_new(__wbindgen_memory()) }
}

/// Returns a handle to this Wasm instance's `WebAssembly.Table` which is the
/// indirect function table used by Rust
pub fn function_table() -> JsValue {
    unsafe { JsValue::_new(__wbindgen_function_table()) }
}

/// A wrapper type around slices and vectors for binding the `Uint8ClampedArray`
/// array in JS.
///
/// If you need to invoke a JS API which must take `Uint8ClampedArray` array,
/// then you can define it as taking one of these types:
///
/// * `Clamped<&[u8]>`
/// * `Clamped<&mut [u8]>`
/// * `Clamped<Vec<u8>>`
///
/// All of these types will show up as `Uint8ClampedArray` in JS and will have
/// different forms of ownership in Rust.
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Clamped<T>(pub T);

impl<T> Deref for Clamped<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Clamped<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Convenience type for use on exported `fn() -> Result<T, JsError>` functions, where you wish to
/// throw a JavaScript `Error` object.
///
/// You can get wasm_bindgen to throw basic errors by simply returning
/// `Err(JsError::new("message"))` from such a function.
///
/// For more complex error handling, `JsError` implements `From<T> where T: std::error::Error` by
/// converting it to a string, so you can use it with `?`. Many Rust error types already do this,
/// and you can use [`thiserror`](https://crates.io/crates/thiserror) to derive Display
/// implementations easily or use any number of boxed error types that implement it already.
///
///
/// To allow JavaScript code to catch only your errors, you may wish to add a subclass of `Error`
/// in a JS module, and then implement `Into<JsValue>` directly on a type and instantiate that
/// subclass. In that case, you would not need `JsError` at all.
///
/// ### Basic example
///
/// ```rust,no_run
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// pub fn throwing_function() -> Result<(), JsError> {
///     Err(JsError::new("message"))
/// }
/// ```
///
/// ### Complex Example
///
/// ```rust,no_run
/// use wasm_bindgen::prelude::*;
///
/// #[derive(Debug, Clone)]
/// enum MyErrorType {
///     SomeError,
/// }
///
/// use core::fmt;
/// impl std::error::Error for MyErrorType {}
/// impl fmt::Display for MyErrorType {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "display implementation becomes the error message")
///     }
/// }
///
/// fn internal_api() -> Result<(), MyErrorType> {
///     Err(MyErrorType::SomeError)
/// }
///
/// #[wasm_bindgen]
/// pub fn throwing_function() -> Result<(), JsError> {
///     internal_api()?;
///     Ok(())
/// }
///
/// ```
#[derive(Clone, Debug)]
pub struct JsError {
    value: JsValue,
}

impl JsError {
    /// Construct a JavaScript `Error` object with a string message
    #[inline]
    pub fn new(s: &str) -> JsError {
        Self {
            value: unsafe { JsValue::_new(crate::__wbindgen_error_new(s.as_ptr(), s.len())) },
        }
    }
}

#[cfg(feature = "std")]
impl<E> From<E> for JsError
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        use std::string::ToString;

        JsError::new(&error.to_string())
    }
}

impl From<JsError> for JsValue {
    fn from(error: JsError) -> Self {
        error.value
    }
}

macro_rules! typed_arrays {
        ($($ty:ident $ctor:ident $clamped_ctor:ident,)*) => {
            $(
                impl From<Box<[$ty]>> for JsValue {
                    fn from(mut vector: Box<[$ty]>) -> Self {
                        let result = unsafe { JsValue::_new($ctor(vector.as_mut_ptr(), vector.len())) };
                        mem::forget(vector);
                        result
                    }
                }

                impl From<Clamped<Box<[$ty]>>> for JsValue {
                    fn from(mut vector: Clamped<Box<[$ty]>>) -> Self {
                        let result = unsafe { JsValue::_new($clamped_ctor(vector.as_mut_ptr(), vector.len())) };
                        mem::forget(vector);
                        result
                    }
                }
            )*
        };
    }

typed_arrays! {
    u8 __wbindgen_uint8_array_new __wbindgen_uint8_clamped_array_new,
    u16 __wbindgen_uint16_array_new __wbindgen_uint16_array_new,
    u32 __wbindgen_uint32_array_new __wbindgen_uint32_array_new,
    u64 __wbindgen_biguint64_array_new __wbindgen_biguint64_array_new,
    i8 __wbindgen_int8_array_new __wbindgen_int8_array_new,
    i16 __wbindgen_int16_array_new __wbindgen_int16_array_new,
    i32 __wbindgen_int32_array_new __wbindgen_int32_array_new,
    i64 __wbindgen_bigint64_array_new __wbindgen_bigint64_array_new,
    f32 __wbindgen_float32_array_new __wbindgen_float32_array_new,
    f64 __wbindgen_float64_array_new __wbindgen_float64_array_new,
}

impl __rt::VectorIntoJsValue for JsValue {
    fn vector_into_jsvalue(vector: Box<[JsValue]>) -> JsValue {
        __rt::js_value_vector_into_jsvalue::<JsValue>(vector)
    }
}

impl<T: JsObject> __rt::VectorIntoJsValue for T {
    fn vector_into_jsvalue(vector: Box<[T]>) -> JsValue {
        __rt::js_value_vector_into_jsvalue::<T>(vector)
    }
}

impl __rt::VectorIntoJsValue for String {
    fn vector_into_jsvalue(vector: Box<[String]>) -> JsValue {
        __rt::js_value_vector_into_jsvalue::<String>(vector)
    }
}

impl<T> From<Vec<T>> for JsValue
where
    JsValue: From<Box<[T]>>,
{
    fn from(vector: Vec<T>) -> Self {
        JsValue::from(vector.into_boxed_slice())
    }
}

impl<T> From<Clamped<Vec<T>>> for JsValue
where
    JsValue: From<Clamped<Box<[T]>>>,
{
    fn from(vector: Clamped<Vec<T>>) -> Self {
        JsValue::from(Clamped(vector.0.into_boxed_slice()))
    }
}
