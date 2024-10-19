package top.pingfanh.jni
class RustNative {
    companion object {
        init {
            System.loadLibrary("ping")
        }
    }

    external fun greeting(string: String): String
    external fun add(string: String): String
    external fun hi(string: String): String
}