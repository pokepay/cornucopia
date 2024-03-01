// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct NullityParams<
    'a,
    T1: cornucopia_async::StringSql,
    T2: cornucopia_async::ArraySql<Item = Option<T1>>,
    T3: cornucopia_async::StringSql,
> {
    pub texts: T2,
    pub name: T3,
    pub composite: Option<super::super::types::public::NullityCompositeParams<'a>>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Nullity {
    pub texts: Vec<Option<String>>,
    pub name: String,
    pub composite: Option<super::super::types::public::NullityComposite>,
}
pub struct NullityBorrowed<'a> {
    pub texts: cornucopia_async::ArrayIterator<'a, Option<&'a str>>,
    pub name: &'a str,
    pub composite: Option<super::super::types::public::NullityCompositeBorrowed<'a>>,
}
impl<'a> From<NullityBorrowed<'a>> for Nullity {
    fn from(
        NullityBorrowed {
            texts,
            name,
            composite,
        }: NullityBorrowed<'a>,
    ) -> Self {
        Self {
            texts: texts.map(|v| v.map(|v| v.into())).collect(),
            name: name.into(),
            composite: composite.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::NullityBorrowed,
        mapper: fn(super::NullityBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NullityBorrowed) -> R) -> NullityQuery<'a, C, R, N> {
            NullityQuery {
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
    pub fn new_nullity() -> NewNullityStmt {
        NewNullityStmt(
            "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
            None,
        )
    }
    pub struct NewNullityStmt(&'static str, Option<postgres::Statement>);
    impl NewNullityStmt {
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
            T2: cornucopia_sync::ArraySql<Item = Option<T1>>,
            T3: cornucopia_sync::StringSql,
        >(
            &'a self,
            client: &'a mut C,
            texts: &'a T2,
            name: &'a T3,
            composite: &'a Option<super::super::super::types::public::NullityCompositeParams<'a>>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[texts, name, composite])
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::ArraySql<Item = Option<T1>>,
            T3: cornucopia_sync::StringSql,
        >
        cornucopia_sync::Params<
            'a,
            super::NullityParams<'a, T1, T2, T3>,
            Result<u64, postgres::Error>,
            C,
        > for NewNullityStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::NullityParams<'a, T1, T2, T3>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.texts, &params.name, &params.composite)
        }
    }
    pub fn nullity() -> NullityStmt {
        NullityStmt("SELECT * FROM nullity", None)
    }
    pub struct NullityStmt(&'static str, Option<postgres::Statement>);
    impl NullityStmt {
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
        ) -> NullityQuery<'a, C, super::Nullity, 0> {
            NullityQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NullityBorrowed {
                    texts: row.get(0),
                    name: row.get(1),
                    composite: row.get(2),
                },
                mapper: |it| <super::Nullity>::from(it),
            }
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::NullityBorrowed,
        mapper: fn(super::NullityBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::NullityBorrowed) -> R) -> NullityQuery<'a, C, R, N> {
            NullityQuery {
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
    pub fn new_nullity() -> NewNullityStmt {
        NewNullityStmt(
            "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
            None,
        )
    }
    pub struct NewNullityStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NewNullityStmt {
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
            T2: cornucopia_async::ArraySql<Item = Option<T1>>,
            T3: cornucopia_async::StringSql,
        >(
            &'a self,
            client: &'a C,
            texts: &'a T2,
            name: &'a T3,
            composite: &'a Option<super::super::super::types::public::NullityCompositeParams<'a>>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[texts, name, composite]).await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::ArraySql<Item = Option<T1>>,
            T3: cornucopia_async::StringSql,
        >
        cornucopia_async::Params<
            'a,
            super::NullityParams<'a, T1, T2, T3>,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for NewNullityStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::NullityParams<'a, T1, T2, T3>,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.texts, &params.name, &params.composite))
        }
    }
    pub fn nullity() -> NullityStmt {
        NullityStmt("SELECT * FROM nullity", None)
    }
    pub struct NullityStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NullityStmt {
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
        ) -> NullityQuery<'a, C, super::Nullity, 0> {
            NullityQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::NullityBorrowed {
                    texts: row.get(0),
                    name: row.get(1),
                    composite: row.get(2),
                },
                mapper: |it| <super::Nullity>::from(it),
            }
        }
    }
}
