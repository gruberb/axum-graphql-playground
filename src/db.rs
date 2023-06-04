use std::sync::Arc;

use anyhow::Result;
use rocksdb::{ColumnFamily, ColumnFamilyDescriptor, Options, DB};

use crate::models::{author::Author, book::Book};

pub(crate) struct Db {
	pub(crate) db: DB,
}

impl Db {
	fn build() -> Self {
		let mut db_options = Options::default();
		db_options.create_missing_column_families(true);

		let column_families = vec![
			ColumnFamilyDescriptor::new("author", Options::default()),
			ColumnFamilyDescriptor::new("book", Options::default()),
		];

		let db = DB::open_cf_descriptors(&db_options, "./db", column_families)
			.expect("Could not open DB");

		Self { db }
	}

	pub fn bootstrap() -> Arc<Self> {
		Arc::new(Self::build())
	}

	pub fn update_book(&self, book: &Book) -> Result<()> {
		let book_json = serde_json::to_string(&book)?;
		let book_cf: &ColumnFamily = self.db.cf_handle("book").unwrap();
		Ok(self.db.put_cf(book_cf, book.id.clone(), book_json)?)
	}

	pub fn get_book(&self, id: String) -> Result<Option<Book>> {
		let book_cf: &ColumnFamily = self.db.cf_handle("book").unwrap();
		let data = self
			.db
			.get_cf(book_cf, &id)
			.ok()
			.ok_or(anyhow::anyhow!("Book not found"))?;

		match &data {
			Some(data) => {
				let book: Book = serde_json::from_slice(&data[..])?;
				Ok(Some(book))
			}
			None => Ok(None),
		}
	}

	pub fn update_author(&self, author: &Author) -> Result<()> {
		let author_json = serde_json::to_string(&author)?;
		let author_cf: &ColumnFamily = self.db.cf_handle("author").unwrap();
		Ok(self.db.put_cf(author_cf, author.id.clone(), author_json)?)
	}

	pub fn get_author(&self, id: String) -> Result<Option<Author>> {
		let author_cf: &ColumnFamily = self.db.cf_handle("author").unwrap();
		let data = self
			.db
			.get_cf(author_cf, &id)
			.ok()
			.ok_or(anyhow::anyhow!("Author not found"))?;

		match &data {
			Some(data) => {
				let author: Author = serde_json::from_slice(&data[..])?;
				Ok(Some(author))
			}
			None => Ok(None),
		}
	}
}
