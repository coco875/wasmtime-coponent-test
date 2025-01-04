#[derive(Clone)]
pub struct Actor {
    pub mod_id: _rt::String,
    pub id: _rt::String,
    pub x: f32,
    pub y: f32,
}
impl ::core::fmt::Debug for Actor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Actor")
            .field("mod-id", &self.mod_id)
            .field("id", &self.id)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn init_actor(a: &Actor) {
    unsafe {
        let Actor { mod_id: mod_id0, id: id0, x: x0, y: y0 } = a;
        let vec1 = mod_id0;
        let ptr1 = vec1.as_ptr().cast::<u8>();
        let len1 = vec1.len();
        let vec2 = id0;
        let ptr2 = vec2.as_ptr().cast::<u8>();
        let len2 = vec2.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "init-actor"]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize, _: f32, _: f32);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize, _: f32, _: f32) {
            unreachable!()
        }
        wit_import(
            ptr1.cast_mut(),
            len1,
            ptr2.cast_mut(),
            len2,
            _rt::as_f32(x0),
            _rt::as_f32(y0),
        );
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_init_cabi<T: Guest>() {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    T::init();
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_actor_init_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: *mut u8,
    arg3: usize,
    arg4: f32,
    arg5: f32,
) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let result2 = T::actor_init(Actor {
        mod_id: _rt::string_lift(bytes0),
        id: _rt::string_lift(bytes1),
        x: arg4,
        y: arg5,
    });
    match result2 {
        true => 1,
        false => 0,
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_actor_update_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: *mut u8,
    arg3: usize,
    arg4: f32,
    arg5: f32,
) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let result2 = T::actor_update(Actor {
        mod_id: _rt::string_lift(bytes0),
        id: _rt::string_lift(bytes1),
        x: arg4,
        y: arg5,
    });
    match result2 {
        true => 1,
        false => 0,
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_actor_render_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: *mut u8,
    arg3: usize,
    arg4: f32,
    arg5: f32,
) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let result2 = T::actor_render(Actor {
        mod_id: _rt::string_lift(bytes0),
        id: _rt::string_lift(bytes1),
        x: arg4,
        y: arg5,
    });
    match result2 {
        true => 1,
        false => 0,
    }
}
pub trait Guest {
    fn init();
    fn actor_init(a: Actor) -> bool;
    fn actor_update(a: Actor) -> bool;
    fn actor_render(a: Actor) -> bool;
}
#[doc(hidden)]
macro_rules! __export_world_example_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "init"] unsafe extern "C" fn export_init() {
        $($path_to_types)*:: _export_init_cabi::<$ty > () } #[export_name = "actor-init"]
        unsafe extern "C" fn export_actor_init(arg0 : * mut u8, arg1 : usize, arg2 : *
        mut u8, arg3 : usize, arg4 : f32, arg5 : f32,) -> i32 { $($path_to_types)*::
        _export_actor_init_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5) }
        #[export_name = "actor-update"] unsafe extern "C" fn export_actor_update(arg0 : *
        mut u8, arg1 : usize, arg2 : * mut u8, arg3 : usize, arg4 : f32, arg5 : f32,) ->
        i32 { $($path_to_types)*:: _export_actor_update_cabi::<$ty > (arg0, arg1, arg2,
        arg3, arg4, arg5) } #[export_name = "actor-render"] unsafe extern "C" fn
        export_actor_render(arg0 : * mut u8, arg1 : usize, arg2 : * mut u8, arg3 : usize,
        arg4 : f32, arg5 : f32,) -> i32 { $($path_to_types)*::
        _export_actor_render_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
mod _rt {
    pub use alloc_crate::string::String;
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_example_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_example_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:component:rust-component:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 295] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa9\x01\x01A\x02\x01\
A\x0a\x01r\x04\x06mod-ids\x02ids\x01xv\x01yv\x03\0\x05actor\x03\0\0\x01@\x01\x01\
a\x01\x01\0\x03\0\x0ainit-actor\x01\x02\x01@\0\x01\0\x04\0\x04init\x01\x03\x01@\x01\
\x01a\x01\0\x7f\x04\0\x0aactor-init\x01\x04\x04\0\x0cactor-update\x01\x04\x04\0\x0c\
actor-render\x01\x04\x04\0\x20component:rust-component/example\x04\0\x0b\x0d\x01\
\0\x07example\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
