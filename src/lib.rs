use jni::objects::JClass;
use jni::sys::{JNIEnv, jobject};
use lazy_static::lazy_static;
use ndk::bitmap::{AndroidBitmap, BitmapFormat};
use maikor_vm_interface::{PIXEL_SIZE, SCREEN_BYTES, VMHost};
use maikor_vm_core::constants::graphics::{SCREEN_HEIGHT, SCREEN_WIDTH};

lazy_static! {
    static ref VM_INSTANCE: VMHost = {
        let mut vm = VMHost::new();
        vm.pop_test();
        vm.run();
        vm
    };
}

#[no_mangle]
pub extern "C" fn Java_app_maikor_adapter_VmInterface_getScreenWidth(_env: *mut JNIEnv,
                                                                     _obj: JClass) -> i32 {
    SCREEN_WIDTH as i32
}

#[no_mangle]
pub extern "C" fn Java_app_maikor_adapter_VmInterface_getScreenHeight(_env: *mut JNIEnv,
                                                                      _obj: JClass) -> i32 {
    SCREEN_HEIGHT as i32
}

#[no_mangle]
pub extern "C" fn Java_app_maikor_adapter_VmInterface_render(env: *mut JNIEnv,
                                                             _obj: JClass, bitmap: jobject) {
    unsafe {
        let bitmap = AndroidBitmap::from_jni(env, bitmap);
        assert_eq!(bitmap.get_info().unwrap().format(), BitmapFormat::RGBA_8888);
        assert_eq!(bitmap.get_info().unwrap().width(), SCREEN_WIDTH as u32);
        assert_eq!(bitmap.get_info().unwrap().height(), SCREEN_HEIGHT as u32);
        assert_eq!(bitmap.get_info().unwrap().stride(), (PIXEL_SIZE * SCREEN_WIDTH) as u32);

        let mut pointer = bitmap.lock_pixels().unwrap();
        let pixels = std::slice::from_raw_parts_mut(pointer as *mut u8, SCREEN_BYTES);
        VM_INSTANCE.render(pixels);
        bitmap.lock_pixels().unwrap();
    }
}