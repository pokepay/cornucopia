// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct InsertNightmareDomainParams<
    'a,
    T1: cornucopia_async::StringSql,
    T2: cornucopia_async::JsonSql,
    T3: cornucopia_async::JsonSql,
    T4: cornucopia_async::ArraySql<Item = T3>,
> {
    pub txt: T1,
    pub json: T2,
    pub nb: i32,
    pub arr: T4,
    pub composite: Option<super::super::types::public::DomainCompositeParams<'a>>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct SelectNightmareDomain {
    pub txt: String,
    pub json: serde_json::Value,
    pub nb: i32,
    pub arr: Vec<serde_json::Value>,
}
pub struct SelectNightmareDomainBorrowed<'a> {
    pub txt: &'a str,
    pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub nb: i32,
    pub arr:
        cornucopia_async::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
}
impl<'a> From<SelectNightmareDomainBorrowed<'a>> for SelectNightmareDomain {
    fn from(
        SelectNightmareDomainBorrowed { txt, json, nb, arr }: SelectNightmareDomainBorrowed<'a>,
    ) -> Self {
        Self {
            txt: txt.into(),
            json: serde_json::from_str(json.0.get()).unwrap(),
            nb,
            arr: arr
                .map(|v| serde_json::from_str(v.0.get()).unwrap())
                .collect(),
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct SelectNightmareDomainNull {
    pub txt: Option<String>,
    pub json: Option<serde_json::Value>,
    pub nb: Option<i32>,
    pub arr: Option<Vec<Option<serde_json::Value>>>,
    pub composite: Option<super::super::types::public::DomainComposite>,
}
pub struct SelectNightmareDomainNullBorrowed<'a> {
    pub txt: Option<&'a str>,
    pub json: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub nb: Option<i32>,
    pub arr: Option<
        cornucopia_async::ArrayIterator<
            'a,
            Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
        >,
    >,
    pub composite: Option<super::super::types::public::DomainCompositeBorrowed<'a>>,
}
impl<'a> From<SelectNightmareDomainNullBorrowed<'a>> for SelectNightmareDomainNull {
    fn from(
        SelectNightmareDomainNullBorrowed {
            txt,
            json,
            nb,
            arr,
            composite,
        }: SelectNightmareDomainNullBorrowed<'a>,
    ) -> Self {
        Self {
            txt: txt.map(|v| v.into()),
            json: json.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            nb,
            arr: arr.map(|v| {
                v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                    .collect()
            }),
            composite: composite.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::SelectNightmareDomainBorrowed,
        mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
        ) -> SelectNightmareDomainQuery<'a, C, R, N> {
            SelectNightmareDomainQuery {
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
    pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::SelectNightmareDomainNullBorrowed,
        mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
        ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
            SelectNightmareDomainNullQuery {
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
    pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
        SelectNightmareDomainStmt("SELECT txt, json, nb, arr FROM nightmare_domain", None)
    }
    pub struct SelectNightmareDomainStmt(&'static str, Option<postgres::Statement>);
    impl SelectNightmareDomainStmt {
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
        ) -> SelectNightmareDomainQuery<'a, C, super::SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::SelectNightmareDomainBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                },
                mapper: |it| <super::SelectNightmareDomain>::from(it),
            }
        }
    }
    pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
        InsertNightmareDomainStmt("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)", None)
    }
    pub struct InsertNightmareDomainStmt(&'static str, Option<postgres::Statement>);
    impl InsertNightmareDomainStmt {
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
            T2: cornucopia_sync::JsonSql,
            T3: cornucopia_sync::JsonSql,
            T4: cornucopia_sync::ArraySql<Item = T3>,
        >(
            &'a self,
            client: &'a mut C,
            txt: &'a T1,
            json: &'a T2,
            nb: &'a i32,
            arr: &'a T4,
            composite: &'a Option<super::super::super::types::public::DomainCompositeParams<'a>>,
        ) -> Result<u64, postgres::Error> {
            client.execute(
                self.0,
                &[
                    &cornucopia_sync::private::Domain(txt),
                    &cornucopia_sync::private::Domain(json),
                    &cornucopia_sync::private::Domain(nb),
                    &cornucopia_sync::private::Domain(&cornucopia_sync::private::DomainArray(arr)),
                    composite,
                ],
            )
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::JsonSql,
            T3: cornucopia_sync::JsonSql,
            T4: cornucopia_sync::ArraySql<Item = T3>,
        >
        cornucopia_sync::Params<
            'a,
            super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
            Result<u64, postgres::Error>,
            C,
        > for InsertNightmareDomainStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
        ) -> Result<u64, postgres::Error> {
            self.bind(
                client,
                &params.txt,
                &params.json,
                &params.nb,
                &params.arr,
                &params.composite,
            )
        }
    }
    pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
        SelectNightmareDomainNullStmt("SELECT * FROM nightmare_domain", None)
    }
    pub struct SelectNightmareDomainNullStmt(&'static str, Option<postgres::Statement>);
    impl SelectNightmareDomainNullStmt {
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
        ) -> SelectNightmareDomainNullQuery<'a, C, super::SelectNightmareDomainNull, 0> {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::SelectNightmareDomainNullBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                    composite: row.get(4),
                },
                mapper: |it| <super::SelectNightmareDomainNull>::from(it),
            }
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::SelectNightmareDomainBorrowed,
        mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
        ) -> SelectNightmareDomainQuery<'a, C, R, N> {
            SelectNightmareDomainQuery {
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
    pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::SelectNightmareDomainNullBorrowed,
        mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
        ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
            SelectNightmareDomainNullQuery {
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
    pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
        SelectNightmareDomainStmt("SELECT txt, json, nb, arr FROM nightmare_domain", None)
    }
    pub struct SelectNightmareDomainStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectNightmareDomainStmt {
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
        ) -> SelectNightmareDomainQuery<'a, C, super::SelectNightmareDomain, 0> {
            SelectNightmareDomainQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::SelectNightmareDomainBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                },
                mapper: |it| <super::SelectNightmareDomain>::from(it),
            }
        }
    }
    pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
        InsertNightmareDomainStmt("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)", None)
    }
    pub struct InsertNightmareDomainStmt(&'static str, Option<tokio_postgres::Statement>);
    impl InsertNightmareDomainStmt {
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
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::JsonSql,
            T4: cornucopia_async::ArraySql<Item = T3>,
        >(
            &'a self,
            client: &'a C,
            txt: &'a T1,
            json: &'a T2,
            nb: &'a i32,
            arr: &'a T4,
            composite: &'a Option<super::super::super::types::public::DomainCompositeParams<'a>>,
        ) -> Result<u64, tokio_postgres::Error> {
            client
                .execute(
                    self.0,
                    &[
                        &cornucopia_async::private::Domain(txt),
                        &cornucopia_async::private::Domain(json),
                        &cornucopia_async::private::Domain(nb),
                        &cornucopia_async::private::Domain(
                            &cornucopia_async::private::DomainArray(arr),
                        ),
                        composite,
                    ],
                )
                .await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::JsonSql,
            T4: cornucopia_async::ArraySql<Item = T3>,
        >
        cornucopia_async::Params<
            'a,
            super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for InsertNightmareDomainStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(
                client,
                &params.txt,
                &params.json,
                &params.nb,
                &params.arr,
                &params.composite,
            ))
        }
    }
    pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
        SelectNightmareDomainNullStmt("SELECT * FROM nightmare_domain", None)
    }
    pub struct SelectNightmareDomainNullStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectNightmareDomainNullStmt {
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
        ) -> SelectNightmareDomainNullQuery<'a, C, super::SelectNightmareDomainNull, 0> {
            SelectNightmareDomainNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::SelectNightmareDomainNullBorrowed {
                    txt: row.get(0),
                    json: row.get(1),
                    nb: row.get(2),
                    arr: row.get(3),
                    composite: row.get(4),
                },
                mapper: |it| <super::SelectNightmareDomainNull>::from(it),
            }
        }
    }
}
