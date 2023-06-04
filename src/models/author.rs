use std::sync::Arc;

use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{db::Db, models::book::Book};

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
		let db = ctx
			.data::<Arc<Db>>()
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;

		Ok(self
			.books
			.iter()
			.filter_map(|book| {
				tracing::debug!("Get book with id: {:?}", book.id.clone());
				let book = db.get_book(book.id.clone()).unwrap()?;
				tracing::debug!("Book: {:?}", book);
				Some(book)
			})
			.collect())
	}
}
