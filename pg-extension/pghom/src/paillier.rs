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

#[pg_aggregate]
impl Aggregate for PaillierU64Sum {
    const NAME: &'static str = "paillier_u64_sum";
    const INITIAL_CONDITION: Option<&'static str> = Some(r#"{ "sum": null }"#);
    type Args = (name!(ek, &'static [u8]), name!(value, &'static [u8]));

    fn state(
        mut current: Self::State,
        args: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let ek = ciborium::de::from_reader(args.0).unwrap();
        let value = ciborium::de::from_reader(args.1).unwrap();
        current.sum = match current.sum {
            Some(sum) => Some(Paillier::add(&ek, &sum, &value)),
            None => Some(value),
        };

        current
    }

    type Finalize = Option<Vec<u8>>;

    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pgx::pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
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

impl Default for PaillierU64Sum {
    fn default() -> Self {
        Self { sum: None }
    }
}
