pub mod test_api;

use barrel::Migration;
use quaint::{prelude::Queryable, single::Quaint};

#[macro_export]
macro_rules! assert_eq_datamodels {
    ($left:expr, $right:expr) => {{
        let parsed_result = datamodel::parse_datamodel($left).unwrap();
        let parsed_expected = datamodel::parse_datamodel($right).unwrap();

        let reformatted_result = datamodel::render_datamodel_to_string(&parsed_result).unwrap();
        let reformatted_expected = datamodel::render_datamodel_to_string(&parsed_expected).unwrap();

        pretty_assertions::assert_eq!(reformatted_result, reformatted_expected);
    }};
}

pub fn custom_assert_with_config(left: &str, right: &str) {
    let parsed_expected_datamodel = datamodel::parse_datamodel(&right).unwrap();
    let parsed_expected_config = datamodel::parse_configuration(&right).unwrap();

    let reformatted_expected =
        datamodel::render_datamodel_and_config_to_string(&parsed_expected_datamodel, &parsed_expected_config)
            .expect("Datamodel rendering failed");

    assert_eq!(left, reformatted_expected);
}

pub fn assert_eq_json(a: &str, b: &str) {
    let json_a: serde_json::Value = serde_json::from_str(a).expect("The String a was not valid JSON.");
    let json_b: serde_json::Value = serde_json::from_str(b).expect("The String b was not valid JSON.");

    assert_eq!(json_a, json_b);
}

pub struct BarrelMigrationExecutor {
    database: Quaint,
    sql_variant: barrel::backend::SqlVariant,
    schema_name: String,
}

impl BarrelMigrationExecutor {
    pub async fn execute<F>(&self, migration_fn: F)
    where
        F: FnOnce(&mut Migration) -> (),
    {
        self.execute_with_schema(migration_fn, &self.schema_name).await
    }

    pub async fn execute_with_schema<F>(&self, migration_fn: F, schema_name: &str)
    where
        F: FnOnce(&mut Migration) -> (),
    {
        let mut migration = Migration::new().schema(dbg!(schema_name));
        migration_fn(&mut migration);
        let full_sql = migration.make_from(self.sql_variant);
        run_full_sql(&self.database, &full_sql).await;
    }
}

async fn run_full_sql(database: &Quaint, full_sql: &str) {
    for sql in full_sql.split(";") {
        if sql != "" {
            database.query_raw(&sql, &[]).await.unwrap();
        }
    }
}
