pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// MSSQL query builder.
#[derive(Default, Debug)]
pub struct MssqlQueryBuilder;

const QUOTE: Quote = Quote(b'[', b']');

pub type MSSqlQueryBuilder = MssqlQueryBuilder;

impl GenericBuilder for MssqlQueryBuilder {}

impl SchemaBuilder for MssqlQueryBuilder {}

impl QuotedBuilder for MssqlQueryBuilder {
	fn quote(&self) -> Quote {
		QUOTE
	}
}

impl EscapeBuilder for MssqlQueryBuilder {}

impl TableRefBuilder for MssqlQueryBuilder {}