#[doc(hidden)]
#[repr(transparent)]
pub struct IClass(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClass {
    type Vtable = IClass_Vtbl;
}
unsafe impl ::windows::core::Interface for IClass {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x86104948_2a35_51f1_a53c_0201d700f4ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClass_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub MakeTypeErased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClassFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClassFactory {
    type Vtable = IClassFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IClassFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17021f4e_76fe_5914_91b2_5d71075c7de6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        property: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
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
        static SHARED: ::windows::core::FactoryCache<Class, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetProperty)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Make(&self, value: i32) -> ::windows::core::Result<Class> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Make)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MakeTypeErased)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CreateInstance(property: i32) -> ::windows::core::Result<Class> {
        Self::IClassFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                property,
                result__.as_mut_ptr(),
            )
            .from_abi::<Class>(result__)
        })
    }
    pub fn IClassFactory<R, F: FnOnce(&IClassFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Class, IClassFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for Class {
    type Vtable = IClass_Vtbl;
}
unsafe impl ::windows::core::Interface for Class {
    const IID: ::windows::core::GUID = <IClass as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Class {
    const NAME: &'static str = "Component.Class";
}
::windows::core::interface_hierarchy!(
    Class,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for Class {}
unsafe impl ::core::marker::Sync for Class {}
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
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IClass_Impl,
        const OFFSET: isize,
    >() -> IClass_Vtbl {
        unsafe extern "system" fn Property<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Property() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(value).into()
        }
        unsafe extern "system" fn Make<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Make(value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeTypeErased<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MakeTypeErased(value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IClass, OFFSET>(),
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
pub trait IClassFactory_Impl: Sized {
    fn CreateInstance(&self, property: i32) -> ::windows::core::Result<Class>;
}
impl ::windows::core::RuntimeName for IClassFactory {
    const NAME: &'static str = "Component.IClassFactory";
}
impl IClassFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IClassFactory_Impl,
        const OFFSET: isize,
    >() -> IClassFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IClassFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            property: i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(property) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IClassFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassFactory as ::windows::core::Interface>::IID
    }
}
