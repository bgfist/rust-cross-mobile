package cn.mucang.android.jk.core.question;

public class AsyncRt {
    public static native void initRuntime();
    public static native void testNative(android.content.Context ctx);

    static {
        System.loadLibrary("jk_core_question");

        System.out.println("请调用initRuntime初始化异步运行时");
    }
}
