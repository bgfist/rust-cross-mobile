use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::sql_types::{Binary, Nullable};
use napi_derive::napi;

#[cfg_attr(any(target_env = "ohos", target_os = "macos"), napi)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = super::schema::t_question)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub _id: i32,
    pub question_id: i32,
    pub chapter_id: i32,
    #[diesel(deserialize_as = EncodedBin)]
    pub question: String,
    pub answer: i32,
    pub option_a: Option<String>,
    pub option_b: Option<String>,
    pub option_c: Option<String>,
    pub option_d: Option<String>,
    #[diesel(deserialize_as = EncodedBinOpt)]
    pub explain: Option<String>,
    pub difficulty: i32,
    pub wrong_rate: f64,
    pub option_type: i32,
}

pub struct EncodedBin(String);

impl Into<String> for EncodedBin {
    fn into(self) -> String {
        self.0
    }
}

impl<DB> Queryable<Binary, DB> for EncodedBin
where
    DB: Backend,
    Vec<u8>: FromSql<Binary, DB>,
{
    type Row = Vec<u8>;

    fn build(mut buf: Self::Row) -> deserialize::Result<Self> {
        let key: &[u8] = b"_jiakaobaodian.com_";
        for (j, byte) in buf.iter_mut().enumerate() {
            *byte = *byte ^ key[j % key.len()];
        }
        let str = String::from_utf8(buf)?;
        Ok(EncodedBin(str))
    }
}

pub struct EncodedBinOpt(Option<String>);

impl Into<Option<String>> for EncodedBinOpt {
    fn into(self) -> Option<String> {
        self.0
    }
}

impl<DB> Queryable<Nullable<Binary>, DB> for EncodedBinOpt
where
    DB: Backend,
    Option<Vec<u8>>: FromSql<Nullable<Binary>, DB>,
{
    type Row = Option<Vec<u8>>;

    fn build(buf: Self::Row) -> deserialize::Result<Self> {
        match buf {
            Some(buf) => {
                let s = EncodedBin::build(buf)?;
                Ok(EncodedBinOpt(Some(s.into())))
            }
            None => Ok(EncodedBinOpt(None)),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::t_chapter)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Chapter {
    pub _id: i32,
    pub title: String,
    pub chapter: i32,
    pub count: i32,
}
