// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use cornucopia_async::GenericClient;
use futures_util::{StreamExt, TryStreamExt};
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(
        "INSERT INTO Book (title)
  VALUES ($1)",
        None,
    )
}
pub struct InsertBookStmt(&'static str, Option<tokio_postgres::Statement>);
impl InsertBookStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
        &'a self,
        client: &'a C,
        title: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[title]).await
    }
}
