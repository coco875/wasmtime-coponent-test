#[derive(Clone)]
pub struct Customer {
    pub id: u64,
    pub name: _rt::String,
}
impl ::core::fmt::Debug for Customer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Customer")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct Simple {
    handle: _rt::Resource<Simple>,
}
impl Simple {
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
unsafe impl _rt::WasmResource for Simple {
    #[inline]
    unsafe fn drop(_handle: u32) {
        #[cfg(not(target_arch = "wasm32"))]
        unreachable!();
        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[resource-drop]simple"]
                fn drop(_: u32);
            }
            drop(_handle);
        }
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct RegisterThing {
    handle: _rt::Resource<RegisterThing>,
}
impl RegisterThing {
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
unsafe impl _rt::WasmResource for RegisterThing {
    #[inline]
    unsafe fn drop(_handle: u32) {
        #[cfg(not(target_arch = "wasm32"))]
        unreachable!();
        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[resource-drop]register-thing"]
                fn drop(_: u32);
            }
            drop(_handle);
        }
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct Thing {
    handle: _rt::Resource<Thing>,
}
impl Thing {
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
unsafe impl _rt::WasmResource for Thing {
    #[inline]
    unsafe fn drop(_handle: u32) {
        #[cfg(not(target_arch = "wasm32"))]
        unreachable!();
        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[resource-drop]thing"]
                fn drop(_: u32);
            }
            drop(_handle);
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn register(f: Simple) {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "register"]
            fn wit_import(_: i32);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32) {
            unreachable!()
        }
        wit_import((&f).take_handle() as i32);
    }
}
impl Simple {
    #[allow(unused_unsafe, clippy::all)]
    pub fn f(&self) -> i32 {
        unsafe {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[method]simple.f"]
                fn wit_import(_: i32) -> i32;
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32) -> i32 {
                unreachable!()
            }
            let ret = wit_import((self).handle() as i32);
            ret
        }
    }
}
impl RegisterThing {
    #[allow(unused_unsafe, clippy::all)]
    pub fn call(&self) -> Thing {
        unsafe {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[method]register-thing.call"]
                fn wit_import(_: i32) -> i32;
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32) -> i32 {
                unreachable!()
            }
            let ret = wit_import((self).handle() as i32);
            Thing::from_handle(ret as u32)
        }
    }
}
impl Thing {
    #[allow(unused_unsafe, clippy::all)]
    pub fn new() -> Self {
        unsafe {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[constructor]thing"]
                fn wit_import() -> i32;
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import() -> i32 {
                unreachable!()
            }
            let ret = wit_import();
            Thing::from_handle(ret as u32)
        }
    }
}
impl Thing {
    #[allow(unused_unsafe, clippy::all)]
    pub fn get_a(&self) -> u32 {
        unsafe {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[method]thing.get-a"]
                fn wit_import(_: i32) -> i32;
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32) -> i32 {
                unreachable!()
            }
            let ret = wit_import((self).handle() as i32);
            ret as u32
        }
    }
}
impl Thing {
    #[allow(unused_unsafe, clippy::all)]
    pub fn set_a(&self, operation: u32) {
        unsafe {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[method]thing.set-a"]
                fn wit_import(_: i32, _: i32);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32) {
                unreachable!()
            }
            wit_import((self).handle() as i32, _rt::as_i32(&operation));
        }
    }
}
impl Thing {
    #[allow(unused_unsafe, clippy::all)]
    pub fn execute(&self) -> u32 {
        unsafe {
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[method]thing.execute"]
                fn wit_import(_: i32) -> i32;
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32) -> i32 {
                unreachable!()
            }
            let ret = wit_import((self).handle() as i32);
            ret as u32
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_add_cabi<T: Guest>(arg0: i32, arg1: i32) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::add(arg0, arg1);
    _rt::as_i32(result0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_test_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::test();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let Customer { id: id2, name: name2 } = result0;
    *ptr1.add(0).cast::<i64>() = _rt::as_i64(id2);
    let vec3 = (name2.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr1.add(12).cast::<usize>() = len3;
    *ptr1.add(8).cast::<*mut u8>() = ptr3.cast_mut();
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_test<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(8).cast::<*mut u8>();
    let l1 = *arg0.add(12).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_init_cabi<T: Guest>() {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    T::init();
}
pub trait Guest {
    fn add(x: i32, y: i32) -> i32;
    fn test() -> Customer;
    fn init();
}
#[doc(hidden)]
macro_rules! __export_world_example_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "add"] unsafe extern "C" fn export_add(arg0 :
        i32, arg1 : i32,) -> i32 { $($path_to_types)*:: _export_add_cabi::<$ty > (arg0,
        arg1) } #[export_name = "test"] unsafe extern "C" fn export_test() -> * mut u8 {
        $($path_to_types)*:: _export_test_cabi::<$ty > () } #[export_name =
        "cabi_post_test"] unsafe extern "C" fn _post_return_test(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_test::<$ty > (arg0) } #[export_name = "init"]
        unsafe extern "C" fn export_init() { $($path_to_types)*:: _export_init_cabi::<$ty
        > () } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);
mod _rt {
    pub use alloc_crate::string::String;
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
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 529] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x93\x03\x01A\x02\x01\
A\x1d\x01r\x02\x02idw\x04names\x03\0\x08customer\x03\0\0\x03\0\x06simple\x03\x01\
\x03\0\x0eregister-thing\x03\x01\x03\0\x05thing\x03\x01\x01i\x02\x01@\x01\x01f\x05\
\x01\0\x03\0\x08register\x01\x06\x01h\x02\x01@\x01\x04self\x07\0z\x03\0\x10[meth\
od]simple.f\x01\x08\x01h\x03\x01i\x04\x01@\x01\x04self\x09\0\x0a\x03\0\x1b[metho\
d]register-thing.call\x01\x0b\x01@\0\0\x0a\x03\0\x12[constructor]thing\x01\x0c\x01\
h\x04\x01@\x01\x04self\x0d\0y\x03\0\x13[method]thing.get-a\x01\x0e\x01@\x02\x04s\
elf\x0d\x09operationy\x01\0\x03\0\x13[method]thing.set-a\x01\x0f\x03\0\x15[metho\
d]thing.execute\x01\x0e\x01@\x02\x01xz\x01yz\0z\x04\0\x03add\x01\x10\x01@\0\0\x01\
\x04\0\x04test\x01\x11\x01@\0\x01\0\x04\0\x04init\x01\x12\x04\0\x20component:rus\
t-component/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
