package top.pingfanh.jni;

public class RustNative {
    static {
        System.loadLibrary("ping"); // 加载你的 JNI 库
    }

    // 声明本地方法
    public native String greeting(String string);
    public native String add(String string);
    public native String fuck1(String string);
}
