use std::sync::Mutex;
use std::time::Duration;

use super::models::*;
use crate::error::{error, Error};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use napi_derive::napi;

#[cfg_attr(ffi_uniffi, derive(uniffi::Object))]
#[napi]
pub struct QuestionDb {
    conn: Mutex<Option<SqliteConnection>>,
}

macro_rules! access_mutex_field {
    ($self:tt, $field:tt) => {
        let mut _conn_lock = $self.$field.lock().unwrap();
        let $field = match _conn_lock.as_mut() {
            Some(conn) => conn,
            None => return Err(error("连接已关闭")),
        };
    };
}

#[cfg_attr(ffi_uniffi, uniffi::export)]
#[napi]
impl QuestionDb {
    #[cfg_attr(ffi_uniffi, uniffi::constructor)]
    #[napi(constructor)]
    pub fn new(db_path: String) -> Self {
        QuestionDb {
            conn: Mutex::new(Some(SqliteConnection::establish(db_path.as_str()).unwrap())),
        }
    }

    #[napi]
    pub fn disconnect(&self) {
        self.conn.lock().unwrap().as_mut().take();
    }

    #[napi]
    pub fn get_questions(&self) -> Result<Vec<Question>, Error> {
        access_mutex_field!(self, conn);

        // 实现获取问题的逻辑
        use super::schema::t_question::dsl::*;
        let results = t_question
            .limit(2)
            .select(Question::as_select())
            .load(conn)
            .unwrap();

        println!("Displaying {} questions", results.len());

        return Ok(results);
    }

    #[napi]
    pub fn get_question_by_id(&self, id: i32) -> Result<Option<Question>, Error> {
        access_mutex_field!(self, conn);

        // 实现根据id获取问题的逻辑
        use super::schema::t_question::dsl::*;
        let q = t_question
            .find(id)
            .select(Question::as_select())
            .first(conn)
            .ok();
        return Ok(q);
    }
}

#[cfg_attr(ffi_uniffi, uniffi::export(async_runtime = "tokio"))]
#[napi]
async fn test_async(seconds: u32) -> String {
    tokio::time::sleep(Duration::from_secs(seconds as u64)).await;
    "finished".to_owned()
}

#[cfg(target_os = "android")]
mod _android_native {
    use jni::objects::{JClass, JObject, JValue};
    use jni::sys::jobject;
    use jni::JNIEnv;

    #[export_name = "Java_cn_mucang_android_jk_core_question_AsyncRt_testNative"]
    pub extern "system" fn test_native(env: JNIEnv, _class: JClass, ctx: JObject) {
        let jvm = env.get_java_vm().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let builder = env
            .new_object(
                "android/app/AlertDialog$Builder",
                "(Landroid/content/Context;)V",
                &[JValue::from(&ctx)],
            )
            .unwrap();
        let title = &env.new_string("警告").unwrap();
        env.call_method(
            &builder,
            "setTitle",
            "(Ljava/lang/CharSequence;)Landroid/app/AlertDialog$Builder;",
            &[JValue::from(title)],
        )
        .unwrap();
        let message = &env.new_string("这是一个警告弹窗").unwrap();
        env.call_method(
            &builder,
            "setMessage",
            "(Ljava/lang/CharSequence;)Landroid/app/AlertDialog$Builder;",
            &[JValue::from(message)],
        )
        .unwrap();
        let alert_dialog = env
            .call_method(&builder, "create", "()Landroid/app/AlertDialog;", &[])
            .unwrap()
            .l()
            .unwrap();
        env.call_method(&alert_dialog, "show", "()V", &[]).unwrap();

        // // 创建AlertDialog.Builder实例
        // AlertDialog.Builder builder = new AlertDialog.Builder(this);
        // // 设置标题和消息
        // builder.setTitle("警告")
        //         .setMessage("这是一个警告弹窗")
        //         // 添加确定按钮及其点击事件
        //         .setPositiveButton("确定", new DialogInterface.OnClickListener() {
        //             public void onClick(DialogInterface dialog, int id) {
        //                 // 用户点击了确定按钮
        //             }
        //         })
        //         // 添加取消按钮及其点击事件
        //         .setNegativeButton("取消", new DialogInterface.OnClickListener() {
        //             public void onClick(DialogInterface dialog, int id) {
        //                 // 用户点击了取消按钮
        //                 dialog.dismiss();
        //             }
        //         });

        // // 创建AlertDialog实例并显示
        // AlertDialog alertDialog = builder.create();
        // alertDialog.show();
    }
}

#[cfg(target_os = "ios")]
mod _ios_native {
    use block2::RcBlock;
    use objc2::rc::Retained;
    use objc2_ui_kit::{UIAlertAction, UIAlertActionStyle, UIViewController};

    #[derive(uniffi::Object)]
    struct Handle;

    #[uniffi::export]
    fn test_native(ctx: &Handle) {
        use objc2_foundation::{ns_string, MainThreadMarker};
        use objc2_ui_kit::{UIAlertController, UIAlertControllerStyle};

        let mtm = MainThreadMarker::new().expect("must be on the main thread");
        unsafe {
            // TODO: 这里用retain会有问题，不知道是不是跟Arc的引用计数冲突了
            let ctx = std::mem::transmute::<_, &UIViewController>(ctx);
            let ctl = UIAlertController::alertControllerWithTitle_message_preferredStyle(
                Some(ns_string!("警告")),
                Some(ns_string!("这是一个警告弹窗")),
                UIAlertControllerStyle::Alert,
                mtm,
            );
            let action = UIAlertAction::actionWithTitle_style_handler(
                Some(ns_string!("确定")),
                UIAlertActionStyle::Default,
                Some(&RcBlock::new(|a| println!("用户点击了确定按钮"))),
                mtm,
            );
            ctl.addAction(&action);
            ctx.presentViewController_animated_completion(&Retained::cast(ctl), true, None);
        }
    }
}

#[cfg(target_env = "ohos")]
mod _ohos_native {
    use napi::bindgen_prelude::*;
    use napi::JsUnknown;
    use napi_derive::napi;

    use crate::_napi_inner::load_module;

    #[napi]
    fn test_native(env: Env, obj: Object) {
        let func = obj.get::<_, JsFunction>("showDialog").unwrap().unwrap();
        let mut options = env.create_object().unwrap();
        options.set("title", "警告").unwrap();
        options.set("message", "这是一个警告弹窗").unwrap();
        let _p: Promise<JsUnknown> = func.apply1(Null, options).unwrap();

        let hilog = load_module::<Object>(env, "@ohos.hilog").unwrap();
        let hilog_debug = hilog.get::<_, JsFunction>("debug").unwrap().unwrap();
        //  function debug(domain: number, tag: string, format: string, ...args: any[]): void;
        let _p: Undefined = hilog_debug.apply3(Null, 0x01, "[rust动态库]", "成功展示鸿蒙弹窗").unwrap();
    }
}
