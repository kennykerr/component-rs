#[repr(transparent)]
pub struct Class(::windows::core::IUnknown);
impl Class {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Class, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Property(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Property)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetProperty)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Make(&self, value: i32) -> ::windows::core::Result<Class> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Make)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<Class>(result__)
        }
    }
    pub fn MakeTypeErased(
        &self,
        value: i32,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MakeTypeErased)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Class {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Class {}
impl ::core::fmt::Debug for Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Class").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Class {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Component.Class;{86104948-2a35-51f1-a53c-0201d700f4ee})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Class {
    type Vtable = IClass_Vtbl;
    const IID: ::windows::core::GUID = <IClass as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Class {
    const NAME: &'static str = "Component.Class";
}
impl ::core::convert::From<Class> for ::windows::core::IUnknown {
    fn from(value: Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Class> for ::windows::core::IUnknown {
    fn from(value: &Class) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Class {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Class {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Class> for ::windows::core::IInspectable {
    fn from(value: Class) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Class> for ::windows::core::IInspectable {
    fn from(value: &Class) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Class {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Class {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Class {}
unsafe impl ::core::marker::Sync for Class {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClass(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClass {
    type Vtable = IClass_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x86104948_2a35_51f1_a53c_0201d700f4ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClass_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub Make: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub MakeTypeErased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
pub trait IClass_Impl: Sized {
    fn Property(&self) -> ::windows::core::Result<i32>;
    fn SetProperty(&self, value: i32) -> ::windows::core::Result<()>;
    fn Make(&self, value: i32) -> ::windows::core::Result<Class>;
    fn MakeTypeErased(&self, value: i32) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IClass {
    const NAME: &'static str = "Component.IClass";
}
impl IClass_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IClass_Impl,
        const OFFSET: isize,
    >() -> IClass_Vtbl {
        unsafe extern "system" fn Property<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(value).into()
        }
        unsafe extern "system" fn Make<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Make(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeTypeErased<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MakeTypeErased(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IClass, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Make: Make::<Identity, Impl, OFFSET>,
            MakeTypeErased: MakeTypeErased::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClass as ::windows::core::Interface>::IID
    }
}
