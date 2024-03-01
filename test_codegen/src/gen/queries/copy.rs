// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::super::super::types::public::CloneCompositeBorrowed,
        mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
        ) -> PublicCloneCompositeQuery<'a, C, R, N> {
            PublicCloneCompositeQuery {
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
    pub struct PublicCopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::super::super::types::public::CopyComposite,
        mapper: fn(super::super::super::types::public::CopyComposite) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicCopyCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::CopyComposite) -> R,
        ) -> PublicCopyCompositeQuery<'a, C, R, N> {
            PublicCopyCompositeQuery {
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
    pub fn insert_clone() -> InsertCloneStmt {
        InsertCloneStmt("INSERT INTO clone (composite) VALUES ($1)", None)
    }
    pub struct InsertCloneStmt(&'static str, Option<postgres::Statement>);
    impl InsertCloneStmt {
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
            composite: &'a super::super::super::types::public::CloneCompositeBorrowed<'a>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[composite])
        }
    }
    pub fn select_clone() -> SelectCloneStmt {
        SelectCloneStmt("SELECT * FROM clone", None)
    }
    pub struct SelectCloneStmt(&'static str, Option<postgres::Statement>);
    impl SelectCloneStmt {
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
        ) -> PublicCloneCompositeQuery<'a, C, super::super::super::types::public::CloneComposite, 0>
        {
            PublicCloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_copy() -> InsertCopyStmt {
        InsertCopyStmt("INSERT INTO copy (composite) VALUES ($1)", None)
    }
    pub struct InsertCopyStmt(&'static str, Option<postgres::Statement>);
    impl InsertCopyStmt {
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
            composite: &'a super::super::super::types::public::CopyComposite,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[composite])
        }
    }
    pub fn select_copy() -> SelectCopyStmt {
        SelectCopyStmt("SELECT * FROM copy", None)
    }
    pub struct SelectCopyStmt(&'static str, Option<postgres::Statement>);
    impl SelectCopyStmt {
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
        ) -> PublicCopyCompositeQuery<'a, C, super::super::super::types::public::CopyComposite, 0>
        {
            PublicCopyCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> super::super::super::types::public::CloneCompositeBorrowed,
        mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
        ) -> PublicCloneCompositeQuery<'a, C, R, N> {
            PublicCloneCompositeQuery {
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
                .into_stream()
                .boxed();
            Ok(mapped)
        }
    }
    pub struct PublicCopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::super::super::types::public::CopyComposite,
        mapper: fn(super::super::super::types::public::CopyComposite) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicCopyCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::CopyComposite) -> R,
        ) -> PublicCopyCompositeQuery<'a, C, R, N> {
            PublicCopyCompositeQuery {
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
                .into_stream()
                .boxed();
            Ok(mapped)
        }
    }
    pub fn insert_clone() -> InsertCloneStmt {
        InsertCloneStmt("INSERT INTO clone (composite) VALUES ($1)", None)
    }
    pub struct InsertCloneStmt(&'static str, Option<tokio_postgres::Statement>);
    impl InsertCloneStmt {
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
            composite: &'a super::super::super::types::public::CloneCompositeBorrowed<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[composite]).await
        }
    }
    pub fn select_clone() -> SelectCloneStmt {
        SelectCloneStmt("SELECT * FROM clone", None)
    }
    pub struct SelectCloneStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectCloneStmt {
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
        ) -> PublicCloneCompositeQuery<'a, C, super::super::super::types::public::CloneComposite, 0>
        {
            PublicCloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_copy() -> InsertCopyStmt {
        InsertCopyStmt("INSERT INTO copy (composite) VALUES ($1)", None)
    }
    pub struct InsertCopyStmt(&'static str, Option<tokio_postgres::Statement>);
    impl InsertCopyStmt {
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
            composite: &'a super::super::super::types::public::CopyComposite,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[composite]).await
        }
    }
    pub fn select_copy() -> SelectCopyStmt {
        SelectCopyStmt("SELECT * FROM copy", None)
    }
    pub struct SelectCopyStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectCopyStmt {
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
        ) -> PublicCopyCompositeQuery<'a, C, super::super::super::types::public::CopyComposite, 0>
        {
            PublicCopyCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
}
