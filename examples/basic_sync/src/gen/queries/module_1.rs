// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use cornucopia_sync::GenericClient;
use postgres::fallible_iterator::FallibleIterator;
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(
        "INSERT INTO Book (title)
  VALUES ($1)",
        None,
    )
}
pub struct InsertBookStmt(&'static str, Option<postgres::Statement>);
impl InsertBookStmt {
    pub fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a mut C,
    ) -> Result<Self, postgres::Error> {
        self.1 = Some(client.prepare(self.0)?);
        Ok(self)
    }
    pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
        &'a self,
        client: &'a mut C,
        title: &'a T1,
    ) -> Result<u64, postgres::Error> {
        client.execute(self.0, &[title])
    }
}
