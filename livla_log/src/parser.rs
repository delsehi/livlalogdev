use chrono::NaiveDate;
use datafusion::arrow::array::{BooleanArray, Date64Array, Float32Array, Int32Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use pest::Parser;
use pest_derive::Parser;
use std::sync::Arc;
use std::vec;
use anyhow::Result;

const MILLI_SECONDS: i64 = 1_000;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SLParser;

pub fn input_to_record_batch(input: &str) -> Result<RecordBatch> {
    let mut date_col: Vec<i64> = vec![];
    let mut lift_col: Vec<String> = vec![];
    let mut weight_col: Vec<Option<f32>> = vec![];
    let mut weight_unit_col: Vec<Option<String>> = vec![];
    let mut rep_col: Vec<Option<i32>> = vec![];
    let mut max_col: Vec<Option<bool>> = vec![];
    let mut distance_col: Vec<Option<f32>> = vec![];
    let mut distance_unit_col: Vec<Option<String>> = vec![];
    let mut time_col: Vec<Option<f32>> = vec![];
    let mut time_unit_col: Vec<Option<String>> = vec![];
    let mut rpe_col: Vec<Option<i32>> = vec![];

    let mut push_record = |date: i64,
                           l: String,
                           w: Option<f32>,
                           wu: Option<String>,
                           r: Option<i32>,
                           rpe: Option<i32>,
                           m: Option<bool>,
                           dist: Option<f32>,
                           distu: Option<String>,
                           t: Option<f32>,
                           tu: Option<String>| {
        date_col.push(date);
        lift_col.push(l);
        weight_col.push(w);
        weight_unit_col.push(wu);
        rep_col.push(r);
        rpe_col.push(rpe);
        max_col.push(m);
        distance_col.push(dist);
        distance_unit_col.push(distu);
        time_col.push(t);
        time_unit_col.push(tu);
    };

    let file = SLParser::parse(Rule::file, input)
        .expect("Could not parse file")
        .next()
        .unwrap();
    //TODO: Convert to Options
    let mut current_date: i64 = 0;
    let mut current_lift = "";
    let mut current_weight = "";
    let mut current_weight_unit = "";
    let mut current_reps = "";
    let mut current_max_rep = false;

    for session in file.into_inner() {
        match session.as_rule() {
            Rule::session => {
                let mut inner_rules = session.into_inner();
                let test = inner_rules.next().unwrap();
                let date = match test.as_rule() {
                    Rule::date => {
                        let mut date_inner = test.into_inner();
                        let the_day = date_inner.next().unwrap().as_str();
                        let the_month = date_inner.next().unwrap().as_str();
                        let the_year = date_inner.next().unwrap().as_str();
                        
                        NaiveDate::from_ymd_opt(
                            the_year.parse::<i32>().unwrap(),
                            the_month.parse::<u32>().unwrap(),
                            the_day.parse::<u32>().unwrap(),
                        )
                        .unwrap()
                    }
                    Rule::human_date => {
                        let mut date_inner = test.into_inner();
                        let the_day = date_inner.next().unwrap().as_str();
                        let the_month = date_inner.next().unwrap().as_str();
                        let the_year = date_inner.next().unwrap().as_str();
                        let month = convert_human_date(the_month).unwrap();
                        
                        NaiveDate::from_ymd_opt(
                            the_year.parse::<i32>().unwrap(),
                            month,
                            the_day.parse::<u32>().unwrap(),
                        )
                        .unwrap()
                    }
                    _ => unreachable!(),
                };
                let date = date.and_hms_opt(0, 0, 0).unwrap().timestamp();
                current_date = date * MILLI_SECONDS;

                for set in inner_rules {
                    match set.as_rule() {
                        Rule::set => {
                            let mut inner_rules = set.into_inner();
                            let lift = inner_rules.next().unwrap().as_str();
                            current_lift = lift;
                            for oneset in inner_rules {
                                current_weight = "";
                                current_weight_unit = "";
                                for single_set in oneset.into_inner() {
                                    // println!("SINGLE SET= {:#?}", single_set);
                                    match single_set.as_rule() {
                                        Rule::weight => {
                                            let mut inner_rules = single_set.into_inner();
                                            let weight = inner_rules.next().unwrap().as_str();
                                            let weight_unit = inner_rules.next().unwrap().as_str();
                                            current_weight = weight;
                                            current_weight_unit = weight_unit;
                                        }
                                        Rule::cardio_rep => {
                                            let mut inner_rules = single_set.into_inner();
                                            let mut distance_rules =
                                                inner_rules.next().unwrap().into_inner();
                                            let distance = distance_rules.next().unwrap().as_str();
                                            let distance_unit =
                                                distance_rules.next().unwrap().as_str();
                                            let mut time_inner_rules =
                                                inner_rules.next().unwrap().into_inner();
                                            let time = time_inner_rules.next().unwrap().as_str();
                                            let time_unit =
                                                time_inner_rules.next().unwrap().as_str();

                                            push_record(
                                                current_date,
                                                current_lift.to_string(),
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                Some(distance.parse::<f32>().unwrap()),
                                                Some(distance_unit.to_string()),
                                                Some(time.parse::<f32>().unwrap()),
                                                Some(time_unit.to_string()),
                                            );
                                        }
                                        Rule::max_rep => {
                                            current_max_rep = true;
                                        }
                                        Rule::rep => {
                                            // println!("the rule: {:#?}", single_set);
                                            let mut inner_rules = single_set.into_inner();
                                            // println!("Inner: {:#?}", inner_rules);
                                            let reps = inner_rules.next().unwrap().as_str();
                                            let rpe = inner_rules.next();
                                            let rpe = match rpe {
                                                None => None,
                                                Some(rpe) => {
                                                    let a = rpe.into_inner().next().unwrap().as_str().parse::<i32>();
                                                    a.ok()
                                                },
                                            };
                                
                                            current_reps = reps;
                                            // let d = convert_date_to_ms(current_date);
                                            let we = if current_weight.is_empty() {
                                                None
                                            } else {
                                                Some(current_weight.parse::<f32>().unwrap())
                                            };
                                            push_record(
                                                current_date,
                                                current_lift.to_string(),
                                                we,
                                                Some(current_weight_unit.to_string()),
                                                Some(current_reps.parse::<i32>().unwrap()),
                                                rpe,
                                                Some(current_max_rep),
                                                None,
                                                None,
                                                None,
                                                None,
                                            );
                                            current_max_rep = false;
                                        }
                                        _ => unreachable!("All tokens should have implementations"),
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("date", DataType::Date64, true),
        Field::new("lift", DataType::Utf8, true),
        Field::new("weight", DataType::Float32, true),
        Field::new("weight_unit", DataType::Utf8, true),
        Field::new("reps", DataType::Int32, true),
        Field::new("rpe", DataType::Int32, true),
        Field::new("max_rep", DataType::Boolean, true),
        Field::new("distance", DataType::Float32, true),
        Field::new("distance_unit", DataType::Utf8, true),
        Field::new("time", DataType::Float32, true),
        Field::new("time_unit", DataType::Utf8, true),
    ]));

    // date,lift,weight,weight_unit,reps,max_rep,distance,distance_unit,time,time_unit
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Date64Array::from(date_col)),
            Arc::new(StringArray::from(lift_col)),
            Arc::new(Float32Array::from(weight_col)),
            Arc::new(StringArray::from(weight_unit_col)),
            Arc::new(Int32Array::from(rep_col)),
            Arc::new(Int32Array::from(rpe_col)),
            Arc::new(BooleanArray::from(max_col)),
            Arc::new(Float32Array::from(distance_col)),
            Arc::new(StringArray::from(distance_unit_col)),
            Arc::new(Float32Array::from(time_col)),
            Arc::new(StringArray::from(time_unit_col)),
        ],
    )
    .unwrap();
    Ok(batch)
}

fn convert_human_date(human_date: &str) -> Option<u32> {
    if human_date.contains("jan") {
        Some(1)
    } else if human_date.contains("feb") {
        Some(2)
    } else if human_date.contains("mar") {
        Some(3)
    } else if human_date.contains("ap") {
        Some(4)
    } else if human_date.contains("maj") || human_date.contains("may") {
        Some(5)
    } else if human_date.contains("jun") {
        Some(6)
    } else if human_date.contains("jul") {
        Some(7)
    } else if human_date.contains("aug") {
        Some(8)
    } else if human_date.contains("ept") {
        Some(9)
    } else if human_date.contains("okt") || human_date.contains("oct") {
        Some(10)
    } else if human_date.contains("nov") {
        Some(11)
    } else if human_date.contains("dec") {
        Some(12)
    } else {
        None
    }
}
