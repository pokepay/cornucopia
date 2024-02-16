use std::future::Future;
use tokio_postgres::{
    types::{BorrowToSql, ToSql},
    Client, Error, Row, RowStream, Statement, ToStatement, Transaction,
};

/// Abstraction over multiple types of asynchronous clients.
/// This allows you to use tokio_postgres clients and transactions interchangeably.
///
/// In addition, when the `deadpool` feature is enabled (default), this trait also
/// abstracts over deadpool clients and transactions
pub trait GenericClient: Send + Sync {
    fn prepare(&self, query: &str) -> impl Future<Output = Result<Statement, Error>> + Send;

    fn stmt_cache() -> bool {
        false
    }

    fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<u64, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;

    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<Row, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;

    fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<Option<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;

    fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<Vec<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;

    fn query_raw<T, P, I>(
        &self,
        statement: &T,
        params: I,
    ) -> impl Future<Output = Result<RowStream, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator;
}

impl GenericClient for Transaction<'_> {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        Transaction::prepare(self, query).await
    }

    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Transaction::execute(self, query, params).await
    }

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Transaction::query_one(self, statement, params).await
    }

    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Transaction::query_opt(self, statement, params).await
    }

    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Transaction::query(self, query, params).await
    }

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        Transaction::query_raw(self, statement, params).await
    }
}

impl GenericClient for Client {
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        Client::prepare(self, query).await
    }

    async fn execute<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Client::execute(self, query, params).await
    }

    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Client::query_one(self, statement, params).await
    }

    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Client::query_opt(self, statement, params).await
    }

    async fn query<T>(&self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        Client::query(self, query, params).await
    }

    async fn query_raw<T, P, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        P: BorrowToSql,
        I: IntoIterator<Item = P> + Sync + Send,
        I::IntoIter: ExactSizeIterator,
    {
        Client::query_raw(self, statement, params).await
    }
}
