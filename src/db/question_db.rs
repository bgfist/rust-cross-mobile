use std::sync::Mutex;

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
