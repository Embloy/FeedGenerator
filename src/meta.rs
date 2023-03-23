////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////META-SCORE/////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// ER: [0,1] * 0.5
// TF: [0,10] * 0.2
// SR: [-2,2] * 0.2
// SP: [0,1] * 0.1
// => MIN META-SCORE = -0.4
// => MAX META-SCORE = 3

use std::any::Any;
use chrono::{DateTime, Utc};

use crate::models::{Job, UserPreferences};

const ER_WF: f64 = 0.2;
const TF_WF: f64 = 0.5;
const SR_WF: f64 = 0.2;
const SP_WF: f64 = 0.1;

pub fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    employer_rating(job) * ER_WF + trend_factor(job) * TF_WF + salary_range(job, pref) * SR_WF + spontaneity(job, pref) * SP_WF
}

pub fn calc_score_no_pref(job: &Job) -> f64 {

    employer_rating(job) * ER_WF + trend_factor(job) * TF_WF
}


fn employer_rating(job: &Job) -> f64 { job.employer_rating.unwrap_or_default() as f64 / 5.0 }

fn trend_factor(job: &Job) -> f64 {

    let applications = job.applications_count as f64;
    let view_weight:f64;
    let views = job.view_count as f64;
    if views <= 100.0 {
        view_weight = 0.3;
    }else if views > 100.0 && views <= 500.0 {
        view_weight = 1.0;
    }else {
        view_weight = 1.5/(1.00001-applications/views);
    }

    let score = (job.applications_count as f64 + 1.0).log10() / (views + 1.0).log10();
    println!("For {} the views {} and applications {} add up to the non weighted score {} or weighted score {}", job.job_id, views, applications, score, score * view_weight);
    score * view_weight
    /*
      if job.applications_count > 0 && job.view_count > 0 {
          (job.applications_count * 10 / job.view_count) as f64
      } else { 0.0 }
    */

}

fn salary_range(job: &Job, pref: &UserPreferences) -> f64 {
    let min: f64 = pref.salary_range.unwrap_or_default().0;
    let max: f64 = pref.salary_range.unwrap_or_default().1;
    let salary: f64 = job.salary.unwrap_or_default();
    // Return salary-boost and set lower and upper bounds to +2.0/-2.0
    if salary > 0.0 && min >= 0.0 && max > min {
        let res: f64 = (salary - min) / (max - min);
        if res < -2.0 { return -2.0; };
        if res > 2.0 { return 2.0; };
        return res;
    } else { 0.0 }
}

fn spontaneity(job: &Job, pref: &UserPreferences) -> f64 {
    // Get spontaneity preference
    let p: f64 = pref.spontaneity.unwrap_or_default();
    // Parse start_slot
    let start_slot = DateTime::<Utc>::from_utc(
        chrono::DateTime::parse_from_rfc3339(&job.start_slot)
            .unwrap()
            .naive_utc(),
        Utc,
    );
    // Return ratio of preference to time-delta
    p / (start_slot.signed_duration_since(Utc::now()).num_days() as f64)
}

