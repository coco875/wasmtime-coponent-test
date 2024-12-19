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
pub trait Guest {
    fn add(x: i32, y: i32) -> i32;
    fn test() -> Customer;
}
#[doc(hidden)]
macro_rules! __export_world_example_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "add"] unsafe extern "C" fn export_add(arg0 :
        i32, arg1 : i32,) -> i32 { $($path_to_types)*:: _export_add_cabi::<$ty > (arg0,
        arg1) } #[export_name = "test"] unsafe extern "C" fn export_test() -> * mut u8 {
        $($path_to_types)*:: _export_test_cabi::<$ty > () } #[export_name =
        "cabi_post_test"] unsafe extern "C" fn _post_return_test(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_test::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);
mod _rt {
    pub use alloc_crate::string::String;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
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
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 228] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07g\x01A\x02\x01A\x06\x01\
r\x02\x02idw\x04names\x03\0\x08customer\x03\0\0\x01@\x02\x01xz\x01yz\0z\x04\0\x03\
add\x01\x02\x01@\0\0\x01\x04\0\x04test\x01\x03\x04\0\x20component:rust-component\
/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\x0cprocessed\
-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
