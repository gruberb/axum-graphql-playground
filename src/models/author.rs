use crate::models::book::Book;
use async_graphql::*;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Author {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) books: Vec<Book>,
}

#[Object]
impl Author {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<Book>> {
        // Fetch the books for the author from the database
        let db = ctx
            .data::<Arc<DB>>()
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        Ok(self
            .books
            .iter()
            .filter_map(|book| {
                tracing::debug!("Get book with id: {:?}", book.id.clone());
                let book_data = db.get(book.id.clone()).unwrap()?;
                tracing::debug!("Book data: {:?}", book_data);
                let book: Book = serde_json::from_slice(&book_data).ok()?;
                Some(book)
            })
            .collect())
    }
}
