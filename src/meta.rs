////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////META-SCORE/////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: @cb

use serde::de::Unexpected::Option;

use crate::models::{Job, UserPreferences};

const ER_WF: f64 = 0.5;
const TF_WF: f64 = 0.2;
const SR_WF: f64 = 0.2;
const SP_WF: f64 = 0.1;

pub fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    println!("\nStarted meta::calc_score for {:?}", job);


    println!("Employer rating = {:?}", employer_rating(job));
    println!("Trend factor = {:?}", trend_factor(job));
    println!("Salary range = {:?}", salary_range(job, pref));
    println!("Spontaneity = {:?}", spontaneity(job, pref));

    let res: f64 = employer_rating(job) * ER_WF + trend_factor(job) * TF_WF + salary_range(job, pref) * SR_WF + spontaneity(job, pref) * SP_WF;

    println!("\tMETA-SCORE = {:?}", res);

    return res;
}

fn employer_rating(job: &Job) -> f64 {
    job.employer_rating as f64
}

fn trend_factor(job: &Job) -> f64 {
    if job.applications_count > 0 && job.view_count > 0 {
        (job.applications_count * 10 / job.view_count) as f64
    } else { 0.0 }
}

fn salary_range(job: &Job, pref: &UserPreferences) -> f64 {
    let min: f64 = pref.salary_range.unwrap_or_default().0;
    let max: f64 = pref.salary_range.unwrap_or_default().1;
    let salary: f64 = job.salary.unwrap_or_default();

    if salary > 0.0 && min >= 0.0 && max > min {
        (salary - min) / (max - min)
    } else { 0.0 }
}

fn spontaneity(job: &Job, pref: &UserPreferences) -> f64 {
    0.0 * SP_WF
}

