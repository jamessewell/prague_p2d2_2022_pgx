use pgx::*;

pg_module_magic!();

// Implement our max_timed aggregate

// Define state for aggregate
pub struct MaxTimedState {
    max: Option<i32>,
    timestamp: Option<pg_sys::TimestampTz>,
}

// Define implementation for our aggregate
#[pg_aggregate]
impl Aggregate for MaxTimedState {
    const NAME: &'static str = "max_timed_pgx";
    type State = Internal;
    type Args = (
        pgx::name!(timestamp, Option<pg_sys::TimestampTz>),
        pgx::name!(max, Option<i32>),
    );
    type Finalize = pgx::JsonB;
    // type OrderBy = i32;
    // type MovingState = i32;

    #[pgx(immutable)]
    fn state(
        mut current: Self::State,
        (timestamp, max): Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let inner = unsafe { current.get_or_insert(Self { max, timestamp }) };
        // implementation starts here!
        if max > inner.max {
            inner.max = max;
            inner.timestamp = timestamp;
        }
        current
    }

    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pgx::pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        let inner = unsafe { current.get::<Self>().unwrap() };
        pgx::JsonB(serde_json::json!({
            "max":inner.max,
            "timestamp": inner.timestamp.map(|t| pgx::TimestampWithTimeZone::from(t))
        }))
    }

    // const PARALLEL: Option<ParallelOption> = Some(ParallelOption::Safe);
    // const FINALIZE_MODIFY: Option<FinalizeModify> = Some(FinalizeModify::ReadWrite);
    // const MOVING_FINALIZE_MODIFY: Option<FinalizeModify> = Some(FinalizeModify::ReadWrite);

    // const SORT_OPERATOR: Option<&'static str> = Some("sortop");
    // const MOVING_INITIAL_CONDITION: Option<&'static str> = Some("1,1");
    // const HYPOTHETICAL: bool = true;

    // You can skip all these:
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

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;
    #[pg_test]
    fn test_integer_avg_state_sql() {
        Spi::run("CREATE TABLE test_table (value INTEGER, ts TIMESTAMPTZ);");
        Spi::run(
            "INSERT INTO test_table (value, ts) VALUES 
          (0, null), 
          (1, '2022-02-02'),
          (2, '2022-01-01'),
          (2, '2022-01-02')
          ",
        );
        let json = Spi::get_one::<JsonB>("SELECT max_timed_pgx(ts, value) FROM test_table;")
            .expect("Error running SQL");
        let correct_json = pgx::JsonB(serde_json::json!({
                "max": 2_i32,
                "timestamp": "2022-01-01T00:00:00-00"
            }
        ));
        assert_eq!(json.0, correct_json.0);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec!["timezone = 'UTC'"]
    }
}
