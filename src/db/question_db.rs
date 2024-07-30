use super::models::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use napi_derive::napi;

#[cfg_attr(any(target_env = "ohos", target_os = "macos"), napi)]
pub struct QuestionDb {
    conn: SqliteConnection,
}

#[cfg_attr(any(target_env = "ohos", target_os = "macos"), napi)]
impl QuestionDb {
    #[napi(constructor)]
    pub fn new(db_path: String) -> Self {
        QuestionDb {
            conn: SqliteConnection::establish(db_path.as_str()).unwrap(),
        }
    }

    #[napi]
    pub fn get_questions(&mut self) -> Vec<Question> {
        // 实现获取问题的逻辑
        use super::schema::t_question::dsl::*;
        let results = t_question
            .limit(2)
            .select(Question::as_select())
            .load(&mut self.conn)
            .unwrap();

        println!("Displaying {} questions", results.len());

        return results;
    }

    pub fn get_question_by_id(&mut self, id: i32) -> Option<Question> {
        // 实现根据id获取问题的逻辑
        use super::schema::t_question::dsl::*;
        let q = t_question
            .find(id)
            .select(Question::as_select())
            .first(&mut self.conn)
            .ok();
        return q;
    }
}
