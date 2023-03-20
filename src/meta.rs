////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////META-SCORE/////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: @cb

use crate::models::{Job, UserPreferences};

const ER_WF: f64 = 0.5;
const TF_WF: f64 = 0.2;
const SR_WF: f64 = 0.2;
const SP_WF: f64 = 0.1;

pub fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    employer_rating(job) * ER_WF + trend_factor(job) * TF_WF + salary_range(job, pref) * SR_WF + spontaneity(job, pref) * SP_WF
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
    if job.salary > 0.0 && pref.salary_range.0 > 0.0 && pref.salary_range.1 > 0.0 && pref.salary_range.1 > pref.salary_range.0 {
        (job.salary - pref.salary_range.0) / (pref.salary_range.1 - pref.salary_range.0)
    } else { 0.0 }
}

fn spontaneity(job: &Job, pref: &UserPreferences) -> f64 {
    0.0 * SP_WF
}

