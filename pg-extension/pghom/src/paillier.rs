use kzen_paillier::{Add, EncodedCiphertext, EncryptionKey, Paillier};
use pgx::prelude::*;
use serde::{Deserialize, Serialize};

#[pg_extern]
fn paillier_u64_add(ek: &[u8], c1: &[u8], c2: &[u8]) -> Vec<u8> {
    let ek: EncryptionKey = ciborium::de::from_reader(ek).unwrap();
    let c1: EncodedCiphertext<u64> = ciborium::de::from_reader(c1).unwrap();
    let c2: EncodedCiphertext<u64> = ciborium::de::from_reader(c2).unwrap();

    let sum = Paillier::add(&ek, &c1, &c2);
    let mut result = Vec::new();
    ciborium::ser::into_writer(&sum, &mut result).unwrap();

    result
}

#[derive(Clone, PostgresType, Serialize, Deserialize)]
pub struct PaillierU64Sum {
    sum: Option<EncodedCiphertext<u64>>,
}

impl PaillierU64Sum {
    fn state(
        mut current: <Self as Aggregate>::State,
        args: (&[u8], &[u8]),
    ) -> <Self as Aggregate>::State {
        let ek = ciborium::de::from_reader(args.0).unwrap();
        let value = ciborium::de::from_reader(args.1).unwrap();
        current.sum = match current.sum {
            Some(sum) => Some(Paillier::add(&ek, &sum, &value)),
            None => Some(value),
        };

        current
    }

    fn finalize(current: <Self as Aggregate>::State) -> <Self as Aggregate>::Finalize {
        match current.sum {
            Some(current) => {
                let mut result = Vec::new();
                ciborium::ser::into_writer(&current, &mut result).unwrap();

                Some(result)
            }
            None => None,
        }
    }
}

#[pg_aggregate]
impl Aggregate for PaillierU64Sum {
    const NAME: &'static str = "paillier_u64_sum";
    const INITIAL_CONDITION: Option<&'static str> = Some(r#"{ "sum": null }"#);
    type Args = (name!(ek, &'static [u8]), name!(value, &'static [u8]));

    fn state(
        current: Self::State,
        args: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        Self::state(current, args)
    }

    type Finalize = Option<Vec<u8>>;

    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pgx::pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        Self::finalize(current)
    }
}

impl Default for PaillierU64Sum {
    fn default() -> Self {
        Self { sum: None }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use std::vec;

    use pghom_client::paillier::{decrypt_u64, encrypt_u64, keypair};
    use pgx::prelude::*;

    #[pg_test]
    fn test_paillier_u64_add_sql() {
        let (ek, dk) = keypair();
        let c1 = encrypt_u64(&ek, 1);
        let c2 = encrypt_u64(&ek, 2);

        let result: Vec<u8> = Spi::get_one_with_args(
            "SELECT paillier_u64_add($1, $2, $3);",
            vec![
                (PgBuiltInOids::BYTEAOID.oid(), ek.clone().into_datum()),
                (PgBuiltInOids::BYTEAOID.oid(), c1.clone().into_datum()),
                (PgBuiltInOids::BYTEAOID.oid(), c2.clone().into_datum()),
            ],
        )
        .unwrap()
        .unwrap();

        assert_eq!(3, decrypt_u64(&dk, &result));
    }

    #[pg_test]
    fn test_paillier_u64_sum_sql() {
        let (ek, dk) = keypair();
        let c1 = encrypt_u64(&ek, 1);
        let c2 = encrypt_u64(&ek, 2);
        let c3 = encrypt_u64(&ek, 3);

        Spi::run("CREATE TABLE test_paillier_u64_sum_sql_table (value BYTEA);").unwrap();
        Spi::run_with_args(
            "INSERT INTO test_paillier_u64_sum_sql_table VALUES ($1), ($2), ($3);",
            Some(vec![
                (PgBuiltInOids::BYTEAOID.oid(), c1.clone().into_datum()),
                (PgBuiltInOids::BYTEAOID.oid(), c2.clone().into_datum()),
                (PgBuiltInOids::BYTEAOID.oid(), c3.clone().into_datum()),
            ]),
        )
        .unwrap();

        let result: Vec<u8> = Spi::get_one_with_args(
            "SELECT paillier_u64_sum($1, value) FROM test_paillier_u64_sum_sql_table;",
            vec![(PgBuiltInOids::BYTEAOID.oid(), ek.clone().into_datum())],
        )
        .unwrap()
        .unwrap();

        assert_eq!(6, decrypt_u64(&dk, &result));
    }
}

/// This module is required by `cargo pgx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    #[allow(dead_code)]
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[allow(dead_code)]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
