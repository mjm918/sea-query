use super::*;

impl QueryBuilder for MssqlQueryBuilder {
	fn values_list_tuple_prefix(&self) -> &str {
		"ROW"
	}

	fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut dyn SqlWriter) {
		match select_distinct {
			SelectDistinct::All => write!(sql, "ALL").unwrap(),
			SelectDistinct::Distinct => write!(sql, "DISTINCT").unwrap(),
			_ => {}
		};
	}

	fn prepare_select_limit_offset(&self, select: &SelectStatement, sql: &mut dyn SqlWriter) {
		for order in &select.orders {
			self.prepare_order(&order, sql);
		}
		if let Some(offset) = &select.offset {
			write!(sql, " OFFSET ").unwrap();
			self.prepare_value(offset, sql);
			write!(sql, " ROWS ").unwrap();
		}
		if let Some(limit) = &select.limit {
			write!(sql, " FETCH NEXT ").unwrap();
			self.prepare_value(limit, sql);
			write!(sql, " ROWS ONLY").unwrap();
		}
	}

	fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
		query.prepare_statement(self, sql);
	}

	fn prepare_with_clause_recursive_options(&self, _: &WithClause, _: &mut dyn SqlWriter) {
		// MySQL doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
	}

	fn prepare_with_query_clause_materialization(
		&self,
		_: &CommonTableExpression,
		_: &mut dyn SqlWriter,
	) {
		// MySQL doesn't support declaring materialization in SQL for with query.
	}

	fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
		match order_expr.nulls {
			None => (),
			Some(NullOrdering::Last) => {
				self.prepare_simple_expr(&order_expr.expr, sql);
				write!(sql, " IS NULL ASC, ").unwrap()
			}
			Some(NullOrdering::First) => {
				self.prepare_simple_expr(&order_expr.expr, sql);
				write!(sql, " IS NULL DESC, ").unwrap()
			}
		}
		if !matches!(order_expr.order, Order::Field(_)) {
			self.prepare_simple_expr(&order_expr.expr, sql);
		}
		self.prepare_order(order_expr, sql);
	}

	fn prepare_value(&self, value: &Value, sql: &mut dyn SqlWriter) {
		sql.push_param(value.clone(), self as _);
	}

	fn prepare_on_conflict_target(&self, _: &Option<OnConflictTarget>, _: &mut dyn SqlWriter) {
		// MSSQL doesn't support declaring ON CONFLICT target.
	}

	fn prepare_on_conflict_keywords(&self, sql: &mut dyn SqlWriter) {
		// MSSQL does not supports on conflict
	}

	fn prepare_on_conflict_do_update_keywords(&self, sql: &mut dyn SqlWriter) {
		write!(sql, " UPDATE ").unwrap();
	}

	fn prepare_on_conflict_excluded_table(&self, col: &DynIden, sql: &mut dyn SqlWriter) {
		write!(sql, "VALUES(").unwrap();
		col.prepare(sql.as_writer(), self.quote());
		write!(sql, ")").unwrap();
	}

	fn prepare_on_conflict_condition(&self, _: &ConditionHolder, _: &mut dyn SqlWriter) {}

	fn prepare_returning(&self, _returning: &Option<ReturningClause>, _sql: &mut dyn SqlWriter) {}

	fn random_function(&self) -> &str {
		"RAND"
	}

	fn insert_default_keyword(&self) -> &str {
		"()"
	}
}