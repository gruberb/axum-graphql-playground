use anyhow::Result;
use async_graphql::EmptySubscription;
use async_graphql::{Context, Object, Schema};
use author::Author;
use book::{Book, NewBook};
use rocksdb::DB;
use std::sync::Arc;
use uuid::Uuid;

mod author;
mod book;

pub(crate) struct QueryRoot;
pub(crate) struct MutationRoot;

pub(crate) type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[Object]
impl QueryRoot {
    async fn author(&self, ctx: &Context<'_>, id: String) -> Result<Author> {
        let db = ctx
            .data::<Arc<DB>>()
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;
        tracing::debug!("Get author with id: {:?}", id);
        let author_data = db
            .get(&id)
            .ok()
            .ok_or(anyhow::anyhow!("Author not found"))?;

        let author: author::Author = match &author_data {
            Some(data) => {
                let author: Author = serde_json::from_slice(&data[..])?;
                tracing::debug!("a: {:?}", author);
                author
            }
            None => return Err(anyhow::anyhow!("Author data not found")),
        };

        Ok(author)
    }
}

#[Object]
impl MutationRoot {
    async fn add_book(&self, ctx: &Context<'_>, book_input: NewBook) -> Result<Book> {
        let db = ctx
            .data::<Arc<DB>>()
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        // Create a new author
        let mut author = Author {
            id: Uuid::new_v4().to_string(),
            name: book_input.author.clone(),
            books: vec![],
        };

        // Convert the author to JSON
        let author_json = serde_json::to_string(&author)?;

        tracing::info!("AuthorID: {:?}", author.id.clone());
        // Add the author to the database
        db.put(author.id.clone(), author_json)?;

        // Create a new book with the provided details
        let book = Book {
            id: Uuid::new_v4().to_string(),
            title: book_input.title.clone(),
            author: author.id.clone(),
        };

        // Update the author's books
        author.books.push(book.clone());
        let author_json = serde_json::to_string(&author)?;
        db.put(author.id.clone(), author_json)?;

        // Convert the book to JSON
        let book_json = serde_json::to_string(&book)?;

        // Add the book to the database
        db.put(book.id.clone(), book_json)?;

        Ok(book)
    }
}
