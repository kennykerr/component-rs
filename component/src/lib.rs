mod bindings;
use std::mem::*;
use std::sync::*;
use windows::core::*;
use windows::Win32::Foundation::*;

#[implement(bindings::Class)]
struct Class(RwLock<i32>);

impl bindings::IClass_Impl for Class {
    fn Property(&self) -> ::windows::core::Result<i32> {
        let reader = self.0.read().unwrap();
        Ok(*reader)
    }
    fn SetProperty(&self, value: i32) -> ::windows::core::Result<()> {
        let mut writer = self.0.write().unwrap();
        *writer = value;
        Ok(())
    }
}

#[implement(IActivationFactory)]
struct ClassFactory();

impl IActivationFactory_Impl for ClassFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class(RwLock::new(0)).into())
    }
}

#[no_mangle]
unsafe extern "stdcall" fn DllGetActivationFactory(
    name: ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    let factory: IActivationFactory = ClassFactory().into();
    *result = transmute(factory);
    S_OK
}