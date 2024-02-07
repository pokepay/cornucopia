// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct InsertBookParams<T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql> {
    pub author: Option<T1>,
    pub name: T2,
}
#[derive(Clone, Copy, Debug)]
pub struct ParamsOrderParams {
    pub c: i32,
    pub a: i32,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct SelectBook {
    pub name: String,
    pub author: Option<String>,
}
pub struct SelectBookBorrowed<'a> {
    pub name: &'a str,
    pub author: Option<&'a str>,
}
impl<'a> From<SelectBookBorrowed<'a>> for SelectBook {
    fn from(SelectBookBorrowed { name, author }: SelectBookBorrowed<'a>) -> Self {
        Self {
            name: name.into(),
            author: author.map(|v| v.into()),
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct FindBooks {
    pub name: String,
    pub author: Option<String>,
}
pub struct FindBooksBorrowed<'a> {
    pub name: &'a str,
    pub author: Option<&'a str>,
}
impl<'a> From<FindBooksBorrowed<'a>> for FindBooks {
    fn from(FindBooksBorrowed { name, author }: FindBooksBorrowed<'a>) -> Self {
        Self {
            name: name.into(),
            author: author.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::SelectBookBorrowed,
        mapper: fn(super::SelectBookBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectBookBorrowed) -> R,
        ) -> SelectBookQuery<'a, C, R, N> {
            SelectBookQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::FindBooksBorrowed,
        mapper: fn(super::FindBooksBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::FindBooksBorrowed) -> R,
        ) -> FindBooksQuery<'a, C, R, N> {
            FindBooksQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub fn insert_book() -> InsertBookStmt {
        InsertBookStmt("INSERT INTO book (author, name) VALUES ($1, $2)", None)
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
        pub fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::StringSql,
        >(
            &'a self,
            client: &'a mut C,
            author: &'a Option<T1>,
            name: &'a T2,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[author, name])
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql, T2: cornucopia_sync::StringSql>
        cornucopia_sync::Params<
            'a,
            super::InsertBookParams<T1, T2>,
            Result<u64, postgres::Error>,
            C,
        > for InsertBookStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::InsertBookParams<T1, T2>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.author, &params.name)
        }
    }
    pub fn select_book() -> SelectBookStmt {
        SelectBookStmt("SELECT * FROM book", None)
    }
    pub struct SelectBookStmt(&'static str, Option<postgres::Statement>);
    impl SelectBookStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
        ) -> SelectBookQuery<'a, C, super::SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::SelectBookBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::SelectBook>::from(it),
            }
        }
    }
    pub fn find_books() -> FindBooksStmt {
        FindBooksStmt("SELECT * FROM book WHERE name = ANY ($1)", None)
    }
    pub struct FindBooksStmt(&'static str, Option<postgres::Statement>);
    impl FindBooksStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::ArraySql<Item = T1>,
        >(
            &'a self,
            client: &'a mut C,
            title: &'a T2,
        ) -> FindBooksQuery<'a, C, super::FindBooks, 1> {
            FindBooksQuery {
                client,
                params: [title],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::FindBooksBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::FindBooks>::from(it),
            }
        }
    }
    pub fn params_use_twice() -> ParamsUseTwiceStmt {
        ParamsUseTwiceStmt(
            "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
            None,
        )
    }
    pub struct ParamsUseTwiceStmt(&'static str, Option<postgres::Statement>);
    impl ParamsUseTwiceStmt {
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
            name: &'a T1,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[name])
        }
    }
    pub fn params_order() -> ParamsOrderStmt {
        ParamsOrderStmt("UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1", None)
    }
    pub struct ParamsOrderStmt(&'static str, Option<postgres::Statement>);
    impl ParamsOrderStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
            c: &'a i32,
            a: &'a i32,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[c, a])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::ParamsOrderParams, Result<u64, postgres::Error>, C>
        for ParamsOrderStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::ParamsOrderParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.c, &params.a)
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::SelectBookBorrowed,
        mapper: fn(super::SelectBookBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectBookBorrowed) -> R,
        ) -> SelectBookQuery<'a, C, R, N> {
            SelectBookQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::FindBooksBorrowed,
        mapper: fn(super::FindBooksBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::FindBooksBorrowed) -> R,
        ) -> FindBooksQuery<'a, C, R, N> {
            FindBooksQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub fn insert_book() -> InsertBookStmt {
        InsertBookStmt("INSERT INTO book (author, name) VALUES ($1, $2)", None)
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
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        >(
            &'a self,
            client: &'a C,
            author: &'a Option<T1>,
            name: &'a T2,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[author, name]).await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        >
        cornucopia_async::Params<
            'a,
            super::InsertBookParams<T1, T2>,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for InsertBookStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::InsertBookParams<T1, T2>,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.author, &params.name))
        }
    }
    pub fn select_book() -> SelectBookStmt {
        SelectBookStmt("SELECT * FROM book", None)
    }
    pub struct SelectBookStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectBookStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
        ) -> SelectBookQuery<'a, C, super::SelectBook, 0> {
            SelectBookQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::SelectBookBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::SelectBook>::from(it),
            }
        }
    }
    pub fn find_books() -> FindBooksStmt {
        FindBooksStmt("SELECT * FROM book WHERE name = ANY ($1)", None)
    }
    pub struct FindBooksStmt(&'static str, Option<tokio_postgres::Statement>);
    impl FindBooksStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::ArraySql<Item = T1>,
        >(
            &'a self,
            client: &'a C,
            title: &'a T2,
        ) -> FindBooksQuery<'a, C, super::FindBooks, 1> {
            FindBooksQuery {
                client,
                params: [title],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::FindBooksBorrowed {
                    name: row.get(0),
                    author: row.get(1),
                },
                mapper: |it| <super::FindBooks>::from(it),
            }
        }
    }
    pub fn params_use_twice() -> ParamsUseTwiceStmt {
        ParamsUseTwiceStmt(
            "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
            None,
        )
    }
    pub struct ParamsUseTwiceStmt(&'static str, Option<tokio_postgres::Statement>);
    impl ParamsUseTwiceStmt {
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
            name: &'a T1,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[name]).await
        }
    }
    pub fn params_order() -> ParamsOrderStmt {
        ParamsOrderStmt("UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1", None)
    }
    pub struct ParamsOrderStmt(&'static str, Option<tokio_postgres::Statement>);
    impl ParamsOrderStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
            c: &'a i32,
            a: &'a i32,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[c, a]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::ParamsOrderParams,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for ParamsOrderStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::ParamsOrderParams,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.c, &params.a))
        }
    }
}
