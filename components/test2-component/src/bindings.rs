pub type Actor = spaghettikart::module::iactor::Actor;
#[allow(unused_unsafe, clippy::all)]
pub fn init_actor(a: &Actor) {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "init-actor"]
            fn wit_import(_: i32);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32) {
            unreachable!()
        }
        wit_import((a).handle() as i32);
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
pub unsafe fn _export_actor_init_cabi<T: Guest>(arg0: i32, arg1: f32, arg2: f32) {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let handle0;
    T::actor_init(
        {
            handle0 = spaghettikart::module::iactor::Actor::from_handle(arg0 as u32);
            &handle0
        },
        arg1,
        arg2,
    );
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_actor_update_cabi<T: Guest>(arg0: i32) {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let handle0;
    T::actor_update({
        handle0 = spaghettikart::module::iactor::Actor::from_handle(arg0 as u32);
        &handle0
    });
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_actor_render_cabi<T: Guest>(arg0: i32) {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let handle0;
    T::actor_render({
        handle0 = spaghettikart::module::iactor::Actor::from_handle(arg0 as u32);
        &handle0
    });
}
pub trait Guest {
    fn init();
    fn actor_init(a: &Actor, x: f32, y: f32);
    fn actor_update(a: &Actor);
    fn actor_render(a: &Actor);
}
#[doc(hidden)]
macro_rules! __export_world_mod_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "init"] unsafe extern "C" fn export_init() {
        $($path_to_types)*:: _export_init_cabi::<$ty > () } #[export_name = "actor-init"]
        unsafe extern "C" fn export_actor_init(arg0 : i32, arg1 : f32, arg2 : f32,) {
        $($path_to_types)*:: _export_actor_init_cabi::<$ty > (arg0, arg1, arg2) }
        #[export_name = "actor-update"] unsafe extern "C" fn export_actor_update(arg0 :
        i32,) { $($path_to_types)*:: _export_actor_update_cabi::<$ty > (arg0) }
        #[export_name = "actor-render"] unsafe extern "C" fn export_actor_render(arg0 :
        i32,) { $($path_to_types)*:: _export_actor_render_cabi::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_mod_world_cabi;
#[allow(dead_code)]
pub mod spaghettikart {
    #[allow(dead_code)]
    pub mod module {
        #[allow(dead_code, clippy::all)]
        pub mod iactor {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Actor {
                handle: _rt::Resource<Actor>,
            }
            impl Actor {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Actor {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[resource-drop]actor"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(mod_id: &str, id: &str) -> Self {
                    unsafe {
                        let vec0 = mod_id;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = id;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[constructor]actor"]
                            fn wit_import(
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                        );
                        Actor::from_handle(ret as u32)
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn outside_init(&self, x: f32, y: f32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.outside-init"]
                            fn wit_import(_: i32, _: f32, _: f32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: f32, _: f32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_f32(&x),
                            _rt::as_f32(&y),
                        );
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn update(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.update"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn render(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.render"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                /// getter/setter
                pub fn get_mod_id(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.get-mod-id"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_id(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.get-id"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_x(&self) -> f32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.get-x"]
                            fn wit_import(_: i32) -> f32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> f32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_x(&self, x: f32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.set-x"]
                            fn wit_import(_: i32, _: f32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: f32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_f32(&x));
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_y(&self) -> f32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.get-y"]
                            fn wit_import(_: i32) -> f32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> f32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret
                    }
                }
            }
            impl Actor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_y(&self, y: f32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "spaghettikart:module/iactor")]
                        extern "C" {
                            #[link_name = "[method]actor.set-y"]
                            fn wit_import(_: i32, _: f32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: f32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_f32(&y));
                    }
                }
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
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
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
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
macro_rules! __export_mod_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_mod_world_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_mod_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:spaghettikart:module:mod-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 693] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb5\x04\x01A\x02\x01\
A\x0d\x01B\x14\x04\0\x05actor\x03\x01\x01i\0\x01@\x02\x06mod-ids\x02ids\0\x01\x04\
\0\x12[constructor]actor\x01\x02\x01h\0\x01@\x03\x04self\x03\x01xv\x01yv\x01\0\x04\
\0\x1a[method]actor.outside-init\x01\x04\x01@\x01\x04self\x03\x01\0\x04\0\x14[me\
thod]actor.update\x01\x05\x04\0\x14[method]actor.render\x01\x05\x01@\x01\x04self\
\x03\0s\x04\0\x18[method]actor.get-mod-id\x01\x06\x04\0\x14[method]actor.get-id\x01\
\x06\x01@\x01\x04self\x03\0v\x04\0\x13[method]actor.get-x\x01\x07\x01@\x02\x04se\
lf\x03\x01xv\x01\0\x04\0\x13[method]actor.set-x\x01\x08\x04\0\x13[method]actor.g\
et-y\x01\x07\x01@\x02\x04self\x03\x01yv\x01\0\x04\0\x13[method]actor.set-y\x01\x09\
\x03\0\x1bspaghettikart:module/iactor\x05\0\x02\x03\0\0\x05actor\x03\0\x05actor\x03\
\0\x01\x01h\x02\x01@\x01\x01a\x03\x01\0\x03\0\x0ainit-actor\x01\x04\x01@\0\x01\0\
\x04\0\x04init\x01\x05\x01@\x03\x01a\x03\x01xv\x01yv\x01\0\x04\0\x0aactor-init\x01\
\x06\x04\0\x0cactor-update\x01\x04\x04\0\x0cactor-render\x01\x04\x04\0\x1espaghe\
ttikart:module/mod-world\x04\0\x0b\x0f\x01\0\x09mod-world\x03\0\0\0G\x09producer\
s\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.3\
5.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
