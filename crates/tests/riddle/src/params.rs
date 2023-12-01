// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IParams(::windows_core::IUnknown);
impl IParams {
    pub fn Nothing(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Nothing()).ok() }
    }
    pub fn Bool(&self, a: &mut bool, b: &mut bool) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Bool(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn I8(&self, a: &mut i8, b: &mut i8) -> ::windows_core::Result<i8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.I8(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn U8(&self, a: &mut u8, b: &mut u8) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.U8(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn I16(&self, a: &mut i16, b: &mut i16) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.I16(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn U16(&self, a: &mut u16, b: &mut u16) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.U16(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn I32(&self, a: &mut i32, b: &mut i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.I32(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn U32(&self, a: &mut u32, b: &mut u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.U32(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn I64(&self, a: &mut i64, b: &mut i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.I64(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn U64(&self, a: &mut u64, b: &mut u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.U64(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn F32(&self, a: &mut f32, b: &mut f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.F32(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn F64(&self, a: &mut f64, b: &mut f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.F64(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn ISize(&self, a: &mut isize, b: &mut isize) -> ::windows_core::Result<isize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ISize(a, b, &mut result__)).from_abi(result__)
        }
    }
    pub fn USize(&self, a: &mut usize, b: &mut usize) -> ::windows_core::Result<usize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.USize(a, b, &mut result__)).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IParams,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IParams {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"TODO");
}
unsafe impl ::windows_core::Interface for IParams {
    type Vtable = IParams_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IParams {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IParams_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Nothing:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Bool: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut bool,
        b: *mut bool,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub I8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i8,
        b: *mut i8,
        result__: *mut i8,
    ) -> ::windows_core::HRESULT,
    pub U8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u8,
        b: *mut u8,
        result__: *mut u8,
    ) -> ::windows_core::HRESULT,
    pub I16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i16,
        b: *mut i16,
        result__: *mut i16,
    ) -> ::windows_core::HRESULT,
    pub U16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u16,
        b: *mut u16,
        result__: *mut u16,
    ) -> ::windows_core::HRESULT,
    pub I32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i32,
        b: *mut i32,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub U32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u32,
        b: *mut u32,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub I64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut i64,
        b: *mut i64,
        result__: *mut i64,
    ) -> ::windows_core::HRESULT,
    pub U64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut u64,
        b: *mut u64,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    pub F32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut f32,
        b: *mut f32,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub F64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut f64,
        b: *mut f64,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ISize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut isize,
        b: *mut isize,
        result__: *mut isize,
    ) -> ::windows_core::HRESULT,
    pub USize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: *mut usize,
        b: *mut usize,
        result__: *mut usize,
    ) -> ::windows_core::HRESULT,
}
