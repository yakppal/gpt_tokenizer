#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.72.1.

use crate::tokenizer::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_create__static_method__BPEWrapper_impl(
    port_: MessagePort,
    encoder_entries: impl Wire2Api<Vec<EncoderMapEntry>> + UnwindSafe,
    special_tokens_encoder_entries: impl Wire2Api<Vec<SpecialEncoderMapEntry>> + UnwindSafe,
    pattern: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "create__static_method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_encoder_entries = encoder_entries.wire2api();
            let api_special_tokens_encoder_entries = special_tokens_encoder_entries.wire2api();
            let api_pattern = pattern.wire2api();
            move |task_callback| {
                Ok(BPEWrapper::create(
                    api_encoder_entries,
                    api_special_tokens_encoder_entries,
                    api_pattern,
                ))
            }
        },
    )
}
fn wire_encode_ordinary__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    text: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "encode_ordinary__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_text = text.wire2api();
            move |task_callback| Ok(BPEWrapper::encode_ordinary(&api_that, api_text))
        },
    )
}
fn wire_encode__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    text: impl Wire2Api<String> + UnwindSafe,
    allowed_special_entries: impl Wire2Api<Vec<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "encode__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_text = text.wire2api();
            let api_allowed_special_entries = allowed_special_entries.wire2api();
            move |task_callback| {
                Ok(BPEWrapper::encode(
                    &api_that,
                    api_text,
                    api_allowed_special_entries,
                ))
            }
        },
    )
}
fn wire_count_token__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    text: impl Wire2Api<String> + UnwindSafe,
    allowed_special_entries: impl Wire2Api<Vec<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "count_token__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_text = text.wire2api();
            let api_allowed_special_entries = allowed_special_entries.wire2api();
            move |task_callback| {
                Ok(BPEWrapper::count_token(
                    &api_that,
                    api_text,
                    api_allowed_special_entries,
                ))
            }
        },
    )
}
fn wire_encode_bytes__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    bytes: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "encode_bytes__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_bytes = bytes.wire2api();
            move |task_callback| Ok(BPEWrapper::encode_bytes(&api_that, api_bytes))
        },
    )
}
fn wire_encode_single_token__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    piece: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "encode_single_token__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_piece = piece.wire2api();
            move |task_callback| BPEWrapper::encode_single_token(&api_that, api_piece)
        },
    )
}
fn wire_decode_bytes__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    tokens: impl Wire2Api<Vec<u32>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "decode_bytes__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_tokens = tokens.wire2api();
            move |task_callback| Ok(BPEWrapper::decode_bytes(&api_that, api_tokens))
        },
    )
}
fn wire_decode_single_token_bytes__method__BPEWrapper_impl(
    port_: MessagePort,
    that: impl Wire2Api<BPEWrapper> + UnwindSafe,
    token: impl Wire2Api<usize> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "decode_single_token_bytes__method__BPEWrapper",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_token = token.wire2api();
            move |task_callback| Ok(BPEWrapper::decode_single_token_bytes(&api_that, api_token))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for BPEWrapper {
    fn into_dart(self) -> support::DartAbi {
        vec![self.bpe.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BPEWrapper {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
mod web {
    use super::*;
    // Section: wire functions

    #[wasm_bindgen]
    pub fn wire_create__static_method__BPEWrapper(
        port_: MessagePort,
        encoder_entries: JsValue,
        special_tokens_encoder_entries: JsValue,
        pattern: String,
    ) {
        wire_create__static_method__BPEWrapper_impl(
            port_,
            encoder_entries,
            special_tokens_encoder_entries,
            pattern,
        )
    }

    #[wasm_bindgen]
    pub fn wire_encode_ordinary__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        text: String,
    ) {
        wire_encode_ordinary__method__BPEWrapper_impl(port_, that, text)
    }

    #[wasm_bindgen]
    pub fn wire_encode__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        text: String,
        allowed_special_entries: JsValue,
    ) {
        wire_encode__method__BPEWrapper_impl(port_, that, text, allowed_special_entries)
    }

    #[wasm_bindgen]
    pub fn wire_count_token__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        text: String,
        allowed_special_entries: JsValue,
    ) {
        wire_count_token__method__BPEWrapper_impl(port_, that, text, allowed_special_entries)
    }

    #[wasm_bindgen]
    pub fn wire_encode_bytes__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        bytes: Box<[u8]>,
    ) {
        wire_encode_bytes__method__BPEWrapper_impl(port_, that, bytes)
    }

    #[wasm_bindgen]
    pub fn wire_encode_single_token__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        piece: Box<[u8]>,
    ) {
        wire_encode_single_token__method__BPEWrapper_impl(port_, that, piece)
    }

    #[wasm_bindgen]
    pub fn wire_decode_bytes__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        tokens: Box<[u32]>,
    ) {
        wire_decode_bytes__method__BPEWrapper_impl(port_, that, tokens)
    }

    #[wasm_bindgen]
    pub fn wire_decode_single_token_bytes__method__BPEWrapper(
        port_: MessagePort,
        that: JsValue,
        token: usize,
    ) {
        wire_decode_single_token_bytes__method__BPEWrapper_impl(port_, that, token)
    }

    // Section: allocate functions

    // Section: related functions

    #[wasm_bindgen]
    pub fn drop_opaque_ArcCoreBpe(ptr: *const c_void) {
        unsafe {
            Arc::<Arc<CoreBPE>>::decrement_strong_count(ptr as _);
        }
    }

    #[wasm_bindgen]
    pub fn share_opaque_ArcCoreBpe(ptr: *const c_void) -> *const c_void {
        unsafe {
            Arc::<Arc<CoreBPE>>::increment_strong_count(ptr as _);
            ptr
        }
    }

    // Section: impl Wire2Api

    impl Wire2Api<String> for String {
        fn wire2api(self) -> String {
            self
        }
    }
    impl Wire2Api<Vec<String>> for JsValue {
        fn wire2api(self) -> Vec<String> {
            self.dyn_into::<JsArray>()
                .unwrap()
                .iter()
                .map(Wire2Api::wire2api)
                .collect()
        }
    }

    impl Wire2Api<BPEWrapper> for JsValue {
        fn wire2api(self) -> BPEWrapper {
            let self_ = self.dyn_into::<JsArray>().unwrap();
            assert_eq!(
                self_.length(),
                1,
                "Expected 1 elements, got {}",
                self_.length()
            );
            BPEWrapper {
                bpe: self_.get(0).wire2api(),
            }
        }
    }
    impl Wire2Api<EncoderMapEntry> for JsValue {
        fn wire2api(self) -> EncoderMapEntry {
            let self_ = self.dyn_into::<JsArray>().unwrap();
            assert_eq!(
                self_.length(),
                2,
                "Expected 2 elements, got {}",
                self_.length()
            );
            EncoderMapEntry {
                key: self_.get(0).wire2api(),
                value: self_.get(1).wire2api(),
            }
        }
    }
    impl Wire2Api<Vec<EncoderMapEntry>> for JsValue {
        fn wire2api(self) -> Vec<EncoderMapEntry> {
            self.dyn_into::<JsArray>()
                .unwrap()
                .iter()
                .map(Wire2Api::wire2api)
                .collect()
        }
    }
    impl Wire2Api<Vec<SpecialEncoderMapEntry>> for JsValue {
        fn wire2api(self) -> Vec<SpecialEncoderMapEntry> {
            self.dyn_into::<JsArray>()
                .unwrap()
                .iter()
                .map(Wire2Api::wire2api)
                .collect()
        }
    }
    impl Wire2Api<SpecialEncoderMapEntry> for JsValue {
        fn wire2api(self) -> SpecialEncoderMapEntry {
            let self_ = self.dyn_into::<JsArray>().unwrap();
            assert_eq!(
                self_.length(),
                2,
                "Expected 2 elements, got {}",
                self_.length()
            );
            SpecialEncoderMapEntry {
                key: self_.get(0).wire2api(),
                value: self_.get(1).wire2api(),
            }
        }
    }

    impl Wire2Api<Vec<u32>> for Box<[u32]> {
        fn wire2api(self) -> Vec<u32> {
            self.into_vec()
        }
    }
    impl Wire2Api<Vec<u8>> for Box<[u8]> {
        fn wire2api(self) -> Vec<u8> {
            self.into_vec()
        }
    }

    // Section: impl Wire2Api for JsValue

    impl Wire2Api<RustOpaque<Arc<CoreBPE>>> for JsValue {
        fn wire2api(self) -> RustOpaque<Arc<CoreBPE>> {
            #[cfg(target_pointer_width = "64")]
            {
                compile_error!("64-bit pointers are not supported.");
            }

            unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
        }
    }
    impl Wire2Api<String> for JsValue {
        fn wire2api(self) -> String {
            self.as_string().expect("non-UTF-8 string, or not a string")
        }
    }
    impl Wire2Api<u32> for JsValue {
        fn wire2api(self) -> u32 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<u8> for JsValue {
        fn wire2api(self) -> u8 {
            self.unchecked_into_f64() as _
        }
    }
    impl Wire2Api<Vec<u32>> for JsValue {
        fn wire2api(self) -> Vec<u32> {
            self.unchecked_into::<js_sys::Uint32Array>().to_vec().into()
        }
    }
    impl Wire2Api<Vec<u8>> for JsValue {
        fn wire2api(self) -> Vec<u8> {
            self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
        }
    }
    impl Wire2Api<usize> for JsValue {
        fn wire2api(self) -> usize {
            self.unchecked_into_f64() as _
        }
    }
}
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
mod io {
    use super::*;
    // Section: wire functions

    #[no_mangle]
    pub extern "C" fn wire_create__static_method__BPEWrapper(
        port_: i64,
        encoder_entries: *mut wire_list_encoder_map_entry,
        special_tokens_encoder_entries: *mut wire_list_special_encoder_map_entry,
        pattern: *mut wire_uint_8_list,
    ) {
        wire_create__static_method__BPEWrapper_impl(
            port_,
            encoder_entries,
            special_tokens_encoder_entries,
            pattern,
        )
    }

    #[no_mangle]
    pub extern "C" fn wire_encode_ordinary__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        text: *mut wire_uint_8_list,
    ) {
        wire_encode_ordinary__method__BPEWrapper_impl(port_, that, text)
    }

    #[no_mangle]
    pub extern "C" fn wire_encode__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        text: *mut wire_uint_8_list,
        allowed_special_entries: *mut wire_StringList,
    ) {
        wire_encode__method__BPEWrapper_impl(port_, that, text, allowed_special_entries)
    }

    #[no_mangle]
    pub extern "C" fn wire_count_token__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        text: *mut wire_uint_8_list,
        allowed_special_entries: *mut wire_StringList,
    ) {
        wire_count_token__method__BPEWrapper_impl(port_, that, text, allowed_special_entries)
    }

    #[no_mangle]
    pub extern "C" fn wire_encode_bytes__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        bytes: *mut wire_uint_8_list,
    ) {
        wire_encode_bytes__method__BPEWrapper_impl(port_, that, bytes)
    }

    #[no_mangle]
    pub extern "C" fn wire_encode_single_token__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        piece: *mut wire_uint_8_list,
    ) {
        wire_encode_single_token__method__BPEWrapper_impl(port_, that, piece)
    }

    #[no_mangle]
    pub extern "C" fn wire_decode_bytes__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        tokens: *mut wire_uint_32_list,
    ) {
        wire_decode_bytes__method__BPEWrapper_impl(port_, that, tokens)
    }

    #[no_mangle]
    pub extern "C" fn wire_decode_single_token_bytes__method__BPEWrapper(
        port_: i64,
        that: *mut wire_BPEWrapper,
        token: usize,
    ) {
        wire_decode_single_token_bytes__method__BPEWrapper_impl(port_, that, token)
    }

    // Section: allocate functions

    #[no_mangle]
    pub extern "C" fn new_ArcCoreBpe() -> wire_ArcCoreBpe {
        wire_ArcCoreBpe::new_with_null_ptr()
    }

    #[no_mangle]
    pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
        let wrap = wire_StringList {
            ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
            len,
        };
        support::new_leak_box_ptr(wrap)
    }

    #[no_mangle]
    pub extern "C" fn new_box_autoadd_bpe_wrapper_0() -> *mut wire_BPEWrapper {
        support::new_leak_box_ptr(wire_BPEWrapper::new_with_null_ptr())
    }

    #[no_mangle]
    pub extern "C" fn new_list_encoder_map_entry_0(len: i32) -> *mut wire_list_encoder_map_entry {
        let wrap = wire_list_encoder_map_entry {
            ptr: support::new_leak_vec_ptr(<wire_EncoderMapEntry>::new_with_null_ptr(), len),
            len,
        };
        support::new_leak_box_ptr(wrap)
    }

    #[no_mangle]
    pub extern "C" fn new_list_special_encoder_map_entry_0(
        len: i32,
    ) -> *mut wire_list_special_encoder_map_entry {
        let wrap = wire_list_special_encoder_map_entry {
            ptr: support::new_leak_vec_ptr(<wire_SpecialEncoderMapEntry>::new_with_null_ptr(), len),
            len,
        };
        support::new_leak_box_ptr(wrap)
    }

    #[no_mangle]
    pub extern "C" fn new_uint_32_list_0(len: i32) -> *mut wire_uint_32_list {
        let ans = wire_uint_32_list {
            ptr: support::new_leak_vec_ptr(Default::default(), len),
            len,
        };
        support::new_leak_box_ptr(ans)
    }

    #[no_mangle]
    pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
        let ans = wire_uint_8_list {
            ptr: support::new_leak_vec_ptr(Default::default(), len),
            len,
        };
        support::new_leak_box_ptr(ans)
    }

    // Section: related functions

    #[no_mangle]
    pub extern "C" fn drop_opaque_ArcCoreBpe(ptr: *const c_void) {
        unsafe {
            Arc::<Arc<CoreBPE>>::decrement_strong_count(ptr as _);
        }
    }

    #[no_mangle]
    pub extern "C" fn share_opaque_ArcCoreBpe(ptr: *const c_void) -> *const c_void {
        unsafe {
            Arc::<Arc<CoreBPE>>::increment_strong_count(ptr as _);
            ptr
        }
    }

    // Section: impl Wire2Api

    impl Wire2Api<RustOpaque<Arc<CoreBPE>>> for wire_ArcCoreBpe {
        fn wire2api(self) -> RustOpaque<Arc<CoreBPE>> {
            unsafe { support::opaque_from_dart(self.ptr as _) }
        }
    }
    impl Wire2Api<String> for *mut wire_uint_8_list {
        fn wire2api(self) -> String {
            let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()
        }
    }
    impl Wire2Api<Vec<String>> for *mut wire_StringList {
        fn wire2api(self) -> Vec<String> {
            let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(Wire2Api::wire2api).collect()
        }
    }
    impl Wire2Api<BPEWrapper> for *mut wire_BPEWrapper {
        fn wire2api(self) -> BPEWrapper {
            let wrap = unsafe { support::box_from_leak_ptr(self) };
            Wire2Api::<BPEWrapper>::wire2api(*wrap).into()
        }
    }
    impl Wire2Api<BPEWrapper> for wire_BPEWrapper {
        fn wire2api(self) -> BPEWrapper {
            BPEWrapper {
                bpe: self.bpe.wire2api(),
            }
        }
    }
    impl Wire2Api<EncoderMapEntry> for wire_EncoderMapEntry {
        fn wire2api(self) -> EncoderMapEntry {
            EncoderMapEntry {
                key: self.key.wire2api(),
                value: self.value.wire2api(),
            }
        }
    }
    impl Wire2Api<Vec<EncoderMapEntry>> for *mut wire_list_encoder_map_entry {
        fn wire2api(self) -> Vec<EncoderMapEntry> {
            let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(Wire2Api::wire2api).collect()
        }
    }
    impl Wire2Api<Vec<SpecialEncoderMapEntry>> for *mut wire_list_special_encoder_map_entry {
        fn wire2api(self) -> Vec<SpecialEncoderMapEntry> {
            let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(Wire2Api::wire2api).collect()
        }
    }
    impl Wire2Api<SpecialEncoderMapEntry> for wire_SpecialEncoderMapEntry {
        fn wire2api(self) -> SpecialEncoderMapEntry {
            SpecialEncoderMapEntry {
                key: self.key.wire2api(),
                value: self.value.wire2api(),
            }
        }
    }

    impl Wire2Api<Vec<u32>> for *mut wire_uint_32_list {
        fn wire2api(self) -> Vec<u32> {
            unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
        }
    }
    impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
        fn wire2api(self) -> Vec<u8> {
            unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
        }
    }

    // Section: wire structs

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_ArcCoreBpe {
        ptr: *const core::ffi::c_void,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_StringList {
        ptr: *mut *mut wire_uint_8_list,
        len: i32,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_BPEWrapper {
        bpe: wire_ArcCoreBpe,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_EncoderMapEntry {
        key: *mut wire_uint_8_list,
        value: usize,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_list_encoder_map_entry {
        ptr: *mut wire_EncoderMapEntry,
        len: i32,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_list_special_encoder_map_entry {
        ptr: *mut wire_SpecialEncoderMapEntry,
        len: i32,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_SpecialEncoderMapEntry {
        key: *mut wire_uint_8_list,
        value: usize,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_uint_32_list {
        ptr: *mut u32,
        len: i32,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_uint_8_list {
        ptr: *mut u8,
        len: i32,
    }

    // Section: impl NewWithNullPtr

    pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }

    impl NewWithNullPtr for wire_ArcCoreBpe {
        fn new_with_null_ptr() -> Self {
            Self {
                ptr: core::ptr::null(),
            }
        }
    }

    impl NewWithNullPtr for wire_BPEWrapper {
        fn new_with_null_ptr() -> Self {
            Self {
                bpe: wire_ArcCoreBpe::new_with_null_ptr(),
            }
        }
    }

    impl Default for wire_BPEWrapper {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    impl NewWithNullPtr for wire_EncoderMapEntry {
        fn new_with_null_ptr() -> Self {
            Self {
                key: core::ptr::null_mut(),
                value: Default::default(),
            }
        }
    }

    impl Default for wire_EncoderMapEntry {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    impl NewWithNullPtr for wire_SpecialEncoderMapEntry {
        fn new_with_null_ptr() -> Self {
            Self {
                key: core::ptr::null_mut(),
                value: Default::default(),
            }
        }
    }

    impl Default for wire_SpecialEncoderMapEntry {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    // Section: sync execution mode utility

    #[no_mangle]
    pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
        unsafe {
            let _ = support::box_from_leak_ptr(ptr);
        };
    }
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;
