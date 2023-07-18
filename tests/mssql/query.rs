use super::*;
use pretty_assertions::assert_eq;

#[test]
fn select_1() {
	assert_eq!(
		Query::select()
			.columns([Char::Character, Char::SizeW, Char::SizeH])
			.from(Char::Table)
			.order_by(Char::FontId, Order::None)
			.limit(10)
			.offset(100)
			.to_string(MssqlQueryBuilder),
		"SELECT [character], [size_w], [size_h] FROM [character] ORDER BY [font_id] OFFSET 100 ROWS  FETCH NEXT 10 ROWS ONLY"
	);
}