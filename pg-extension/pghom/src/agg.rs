/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>
All rights reserved.
Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use core::ffi::CStr;
use pgx::aggregate::*;
use pgx::prelude::*;
use pgx::{pgx, PgVarlena, PgVarlenaInOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Copy, Clone, PostgresType, Serialize, Deserialize)]
#[pgvarlena_inoutfuncs]
pub struct IntegerAvgState {
    sum: i32,
    n: i32,
}

impl IntegerAvgState {
    #[inline(always)]
    fn state(
        mut current: <Self as Aggregate>::State,
        arg: Option<i32>,
    ) -> <Self as Aggregate>::State {
        arg.map(|a| {
            current.sum += a;
            current.n += 1;
        });
        current
    }

    #[inline(always)]
    fn finalize(current: <Self as Aggregate>::State) -> <Self as Aggregate>::Finalize {
        current.sum / current.n
    }
}

impl PgVarlenaInOutFuncs for IntegerAvgState {
    fn input(input: &CStr) -> PgVarlena<Self> {
        let mut result = PgVarlena::<Self>::new();

        let mut split = input.to_bytes().split(|b| *b == b',');
        let sum = split
            .next()
            .map(|value| {
                i32::from_str(unsafe { std::str::from_utf8_unchecked(value) }).expect("invalid i32")
            })
            .expect("expected sum");
        let n = split
            .next()
            .map(|value| {
                i32::from_str(unsafe { std::str::from_utf8_unchecked(value) }).expect("invalid i32")
            })
            .expect("expected n");

        result.sum = sum;
        result.n = n;

        result
    }
    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{},{}", self.sum, self.n));
    }
}

// In order to improve the testability of your code, it's encouraged to make this implementation
// call to your own functions which don't require a PostgreSQL made [`pgx::pg_sys::FunctionCallInfo`].
#[pg_aggregate]
impl Aggregate for IntegerAvgState {
    type State = PgVarlena<Self>;
    type Args = (
        pgx::name!(value, Option<i32>),
        pgx::name!(value2, Option<i32>),
    );
    const NAME: &'static str = "DEMOAVG";

    const INITIAL_CONDITION: Option<&'static str> = Some("0,0");

    #[pgx(parallel_safe, immutable)]
    fn state(
        current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        Self::state(current, arg.0)
    }

    // You can skip all these:
    type Finalize = i32;
    // type OrderBy = i32;
    // type MovingState = i32;

    // const PARALLEL: Option<ParallelOption> = Some(ParallelOption::Safe);
    // const FINALIZE_MODIFY: Option<FinalizeModify> = Some(FinalizeModify::ReadWrite);
    // const MOVING_FINALIZE_MODIFY: Option<FinalizeModify> = Some(FinalizeModify::ReadWrite);

    // const SORT_OPERATOR: Option<&'static str> = Some("sortop");
    // const MOVING_INITIAL_CONDITION: Option<&'static str> = Some("1,1");
    // const HYPOTHETICAL: bool = true;

    // You can skip all these:
    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pgx::pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        Self::finalize(current)
    }

    // fn combine(current: Self::State, _other: Self::State, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Self::State {
    //     unimplemented!()
    // }

    // fn serial(current: Self::State, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Vec<u8> {
    //     unimplemented!()
    // }

    // fn deserial(current: Self::State, _buf: Vec<u8>, _internal: PgBox<Self::State>, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> PgBox<Self::State> {
    //     unimplemented!()
    // }

    // fn moving_state(_mstate: Self::MovingState, _v: Self::Args, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Self::MovingState {
    //     unimplemented!()
    // }

    // fn moving_state_inverse(_mstate: Self::MovingState, _v: Self::Args, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Self::MovingState {
    //     unimplemented!()
    // }

    // fn moving_finalize(_mstate: Self::MovingState, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Self::Finalize {
    //     unimplemented!()
    // }
}

impl Default for IntegerAvgState {
    fn default() -> Self {
        Self { sum: 0, n: 0 }
    }
}
