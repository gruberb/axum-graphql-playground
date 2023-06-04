use std::sync::Arc;

use anyhow::Result;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use author::Author;
use book::{Book, NewBook};
use uuid::Uuid;

use crate::db::Db;

pub(crate) mod author;
pub(crate) mod book;

pub(crate) struct QueryRoot;
pub(crate) struct MutationRoot;

pub(crate) type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[Object]
impl QueryRoot {
	async fn author(&self, ctx: &Context<'_>, id: String) -> Result<Option<Author>> {
		let db = ctx
			.data::<Arc<Db>>()
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;

		let author = db
			.get_author(id)
			.ok()
			.ok_or(anyhow::anyhow!("Author not found"))?;

		Ok(author)
	}

	async fn book(&self, ctx: &Context<'_>, id: String) -> Result<Option<Book>> {
		let db = ctx
			.data::<Arc<Db>>()
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;

		let book = db
			.get_book(id)
			.ok()
			.ok_or(anyhow::anyhow!("Author not found"))?;

		Ok(book)
	}
}

#[Object]
impl MutationRoot {
	async fn add_book(&self, ctx: &Context<'_>, book_input: NewBook) -> Result<Book> {
		let db = ctx
			.data::<Arc<Db>>()
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;

		// Create a new author
		let mut author = Author {
			id: Uuid::new_v4().to_string(),
			name: book_input.author.clone(),
			books: vec![],
		};

		// Add the author to the database
		db.update_author(&author)?;

		// Create a new book with the provided details
		let book = Book {
			id: Uuid::new_v4().to_string(),
			title: book_input.title.clone(),
			author: author.id.clone(),
		};

		// Update the author's books
		author.books.push(book.clone());
		db.update_author(&author)?;

		// Add the book to the database
		db.update_book(&book)?;

		Ok(book)
	}
}
