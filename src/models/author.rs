use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::models::book::Book;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub(crate) struct Author {
	pub(crate) id: String,
	pub(crate) name: String,
	pub(crate) books: Vec<Book>,
}
