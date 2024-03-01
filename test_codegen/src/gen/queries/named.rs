// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct NamedParams<T1: cornucopia_async::StringSql> {
    pub name: T1,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct NamedComplexParams<'a> {
    pub named: super::super::types::public::NamedCompositeBorrowed<'a>,
    pub named_with_dot: Option<super::super::types::public::NamedCompositeWithDot>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Copy)]
pub struct Id {
    pub id: i32,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Named {
    pub id: i32,
    pub name: String,
    pub price: Option<f64>,
    pub show: bool,
}
pub struct NamedBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub price: Option<f64>,
    pub show: bool,
}
impl<'a> From<NamedBorrowed<'a>> for Named {
    fn from(
        NamedBorrowed {
            id,
            name,
            price,
            show,
        }: NamedBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            price,
            show,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct NamedComplex {
    pub named: super::super::types::public::NamedComposite,
    pub named_with_dot: Option<super::super::types::public::NamedCompositeWithDot>,
}
pub struct NamedComplexBorrowed<'a> {
    pub named: super::super::types::public::NamedCompositeBorrowed<'a>,
    pub named_with_dot: Option<super::super::types::public::NamedCompositeWithDot>,
}
impl<'a> From<NamedComplexBorrowed<'a>> for NamedComplex {
    fn from(
        NamedComplexBorrowed {
            named,
            named_with_dot,
        }: NamedComplexBorrowed<'a>,
    ) -> Self {
        Self {
            named: named.into(),
            named_with_dot,
        }
    }
}
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::Id,
        mapper: fn(super::Id) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'a, C, R, N> {
            IdQuery {
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
    pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::NamedBorrowed,
        mapper: fn(super::NamedBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NamedBorrowed) -> R) -> NamedQuery<'a, C, R, N> {
            NamedQuery {
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
    pub struct NamedComplexQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::NamedComplexBorrowed,
        mapper: fn(super::NamedComplexBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedComplexQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedComplexBorrowed) -> R,
        ) -> NamedComplexQuery<'a, C, R, N> {
            NamedComplexQuery {
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
    pub fn new_named_visible() -> NewNamedVisibleStmt {
        NewNamedVisibleStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
            None,
        )
    }
    pub struct NewNamedVisibleStmt(&'static str, Option<postgres::Statement>);
    impl NewNamedVisibleStmt {
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
            price: &'a Option<f64>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
        cornucopia_sync::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedVisibleStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn new_named_hidden() -> NewNamedHiddenStmt {
        NewNamedHiddenStmt(
            "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct NewNamedHiddenStmt(&'static str, Option<postgres::Statement>);
    impl NewNamedHiddenStmt {
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
            price: &'a Option<f64>,
            name: &'a T1,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [price, name],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
        cornucopia_sync::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedHiddenStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.price, &params.name)
        }
    }
    pub fn named() -> NamedStmt {
        NamedStmt("SELECT * FROM named", None)
    }
    pub struct NamedStmt(&'static str, Option<postgres::Statement>);
    impl NamedStmt {
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
        ) -> NamedQuery<'a, C, super::Named, 0> {
            NamedQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn named_by_id() -> NamedByIdStmt {
        NamedByIdStmt("SELECT * FROM named WHERE id = $1", None)
    }
    pub struct NamedByIdStmt(&'static str, Option<postgres::Statement>);
    impl NamedByIdStmt {
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
            id: &'a i32,
        ) -> NamedQuery<'a, C, super::Named, 1> {
            NamedQuery {
                client,
                params: [id],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn new_named_complex() -> NewNamedComplexStmt {
        NewNamedComplexStmt(
            "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
            None,
        )
    }
    pub struct NewNamedComplexStmt(&'static str, Option<postgres::Statement>);
    impl NewNamedComplexStmt {
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
            named: &'a super::super::super::types::public::NamedCompositeBorrowed<'a>,
            named_with_dot: &'a Option<super::super::super::types::public::NamedCompositeWithDot>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[named, named_with_dot])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::NamedComplexParams<'a>, Result<u64, postgres::Error>, C>
        for NewNamedComplexStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::NamedComplexParams<'a>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.named, &params.named_with_dot)
        }
    }
    pub fn named_complex() -> NamedComplexStmt {
        NamedComplexStmt("SELECT * FROM named_complex", None)
    }
    pub struct NamedComplexStmt(&'static str, Option<postgres::Statement>);
    impl NamedComplexStmt {
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
        ) -> NamedComplexQuery<'a, C, super::NamedComplex, 0> {
            NamedComplexQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NamedComplexBorrowed {
                    named: row.get(0),
                    named_with_dot: row.get(1),
                },
                mapper: |it| <super::NamedComplex>::from(it),
            }
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::Id,
        mapper: fn(super::Id) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'a, C, R, N> {
            IdQuery {
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
    pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::NamedBorrowed,
        mapper: fn(super::NamedBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NamedBorrowed) -> R) -> NamedQuery<'a, C, R, N> {
            NamedQuery {
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
    pub struct NamedComplexQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::NamedComplexBorrowed,
        mapper: fn(super::NamedComplexBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NamedComplexQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::NamedComplexBorrowed) -> R,
        ) -> NamedComplexQuery<'a, C, R, N> {
            NamedComplexQuery {
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
    pub fn new_named_visible() -> NewNamedVisibleStmt {
        NewNamedVisibleStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
            None,
        )
    }
    pub struct NewNamedVisibleStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NewNamedVisibleStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
            &'a self,
            client: &'a C,
            name: &'a T1,
            price: &'a Option<f64>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
        cornucopia_async::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedVisibleStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn new_named_hidden() -> NewNamedHiddenStmt {
        NewNamedHiddenStmt(
            "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct NewNamedHiddenStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NewNamedHiddenStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
            &'a self,
            client: &'a C,
            price: &'a Option<f64>,
            name: &'a T1,
        ) -> IdQuery<'a, C, super::Id, 2> {
            IdQuery {
                client,
                params: [price, name],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::Id { id: row.get(0) },
                mapper: |it| <super::Id>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
        cornucopia_async::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
        for NewNamedHiddenStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::NamedParams<T1>,
        ) -> IdQuery<'a, C, super::Id, 2> {
            self.bind(client, &params.price, &params.name)
        }
    }
    pub fn named() -> NamedStmt {
        NamedStmt("SELECT * FROM named", None)
    }
    pub struct NamedStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NamedStmt {
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
        ) -> NamedQuery<'a, C, super::Named, 0> {
            NamedQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn named_by_id() -> NamedByIdStmt {
        NamedByIdStmt("SELECT * FROM named WHERE id = $1", None)
    }
    pub struct NamedByIdStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NamedByIdStmt {
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
            id: &'a i32,
        ) -> NamedQuery<'a, C, super::Named, 1> {
            NamedQuery {
                client,
                params: [id],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NamedBorrowed {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    show: row.get(3),
                },
                mapper: |it| <super::Named>::from(it),
            }
        }
    }
    pub fn new_named_complex() -> NewNamedComplexStmt {
        NewNamedComplexStmt(
            "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
            None,
        )
    }
    pub struct NewNamedComplexStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NewNamedComplexStmt {
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
            named: &'a super::super::super::types::public::NamedCompositeBorrowed<'a>,
            named_with_dot: &'a Option<super::super::super::types::public::NamedCompositeWithDot>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[named, named_with_dot]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::NamedComplexParams<'a>,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for NewNamedComplexStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::NamedComplexParams<'a>,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.named, &params.named_with_dot))
        }
    }
    pub fn named_complex() -> NamedComplexStmt {
        NamedComplexStmt("SELECT * FROM named_complex", None)
    }
    pub struct NamedComplexStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NamedComplexStmt {
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
        ) -> NamedComplexQuery<'a, C, super::NamedComplex, 0> {
            NamedComplexQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NamedComplexBorrowed {
                    named: row.get(0),
                    named_with_dot: row.get(1),
                },
                mapper: |it| <super::NamedComplex>::from(it),
            }
        }
    }
}
