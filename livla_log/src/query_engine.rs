
use datafusion::arrow::array::{ArrayRef, Float32Array};
use datafusion::arrow::datatypes::DataType;
use datafusion::common::cast::{as_float32_array, as_int32_array};
use datafusion::error::DataFusionError;
use datafusion::logical_expr::{ColumnarValue, Volatility};
use datafusion::prelude::*;
use std::sync::Arc;

fn epley_rep_max(w: f32, r: i32) -> f32 {
    if r > 1 {
        w * (1. + (r as f32) / 30.)
    } else {
        w
    }
}


pub fn get_context() -> SessionContext {
    let ctx = SessionContext::new();
    let repmax = Arc::new(|args: &[ColumnarValue]| {
        let args = args.to_vec();

        let weight = args
            .first()
            .expect("no weight")
            .clone()
            .into_array(0)
            .unwrap();
        let w_arr = as_float32_array(&weight).unwrap();
        let reps = args
            .last()
            .expect("No reps given")
            .clone()
            .into_array(0)
            .unwrap();
        let reps = as_int32_array(&reps).expect("Could not create array");
        let arr = w_arr
            .iter()
            .zip(reps.iter())
            .map(|(weight, reps)| match (weight, reps) {
                (Some(weight), Some(reps)) => Some(epley_rep_max(weight, reps)),
                _ => None,
            })
            .collect::<Float32Array>();

        Ok::<ColumnarValue, DataFusionError>(datafusion::logical_expr::ColumnarValue::Array(
            Arc::new(arr) as ArrayRef,
        ))
    });

    let max_rep_udf = create_udf(
        "epley",
        vec![DataType::Float32, DataType::Int32],
        Arc::new(DataType::Float32),
        Volatility::Immutable,
        repmax,
    );
    ctx.register_udf(max_rep_udf);
    ctx
}