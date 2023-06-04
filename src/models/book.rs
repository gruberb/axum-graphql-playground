use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(InputObject)]
pub(crate) struct NewBook {
	pub(crate) title: String,
	pub(crate) author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub(crate) struct Book {
	pub(crate) id: String,
	pub(crate) title: String,
	pub(crate) author: String,
}
