
        #[no_mangle]
        pub extern "system" fn JNI_OnLoad(vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {
            let mut env = vm.get_env().expect("Cannot get reference to the JNIEnv");
            let class_name = "top/pingfanh/jni/RustNative"; // 类名
            let class = env.find_class(class_name).unwrap();

            let methods = [
                
          jni::NativeMethod {
                name: JNIString::from("greeting"),
                sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
                fn_ptr: greeting as *mut _,
            },
        
          jni::NativeMethod {
                name: JNIString::from("add"),
                sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
                fn_ptr: add as *mut _,
            },
        
          jni::NativeMethod {
                name: JNIString::from("hi"),
                sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
                fn_ptr: hi as *mut _,
            },
        
            ];

            jni::sys::JNI_VERSION_1_6
        }
    