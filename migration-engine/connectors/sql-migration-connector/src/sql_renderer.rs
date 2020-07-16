pub(crate) mod rendered_step;

mod common;
mod mysql_renderer;
mod postgres_renderer;
mod sqlite_renderer;

pub(crate) use common::{IteratorJoin, Quoted, QuotedWithSchema};
pub(crate) use mysql_renderer::render_column_type as mysql_render_column_type;
pub(crate) use postgres_renderer::render_column_type as postgres_render_column_type;

use crate::sql_schema_helpers::ColumnRef;
use sql_schema_describer::*;
use std::borrow::Cow;

pub(crate) trait SqlRenderer {
    fn quote<'a>(&self, name: &'a str) -> Quoted<&'a str>;

    fn quote_with_schema<'a, 'b>(&'a self, schema_name: &'a str, name: &'b str) -> QuotedWithSchema<'a, &'b str> {
        QuotedWithSchema {
            schema_name,
            name: self.quote(name),
        }
    }

    fn render_column(&self, schema_name: &str, column: ColumnRef<'_>, add_fk_prefix: bool) -> String;

    fn render_references(&self, schema_name: &str, foreign_key: &ForeignKey) -> String;

    fn render_default<'a>(&self, default: &'a DefaultValue, family: &ColumnTypeFamily) -> Cow<'a, str>;
}