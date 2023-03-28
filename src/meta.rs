////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////META-SCORE/////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// ER: [0,1] * 0.5
// TF: [0,10] * 0.2
// SR: [-2,2] * 0.2
// SP: [0,1] * 0.1
// => MIN META-SCORE = -0.4
// => MAX META-SCORE = 3

use chrono::{TimeZone, Utc};

use crate::models::{Job, UserPreferences};

const ER_WF: f64 = 0.2;
const TF_WF: f64 = 0.5;
const SR_WF: f64 = 0.2;
const SP_WF: f64 = 0.1;

pub fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    //println!("for job id {} employer_score is {} trend_factor is {} salaryrange is {} spontaneity is {}", job.job_id, employer_rating(job), trend_factor(job), salary_range_B(job, pref), spontaneity(job, pref));
    employer_rating(job) * ER_WF + trend_factor(job) * TF_WF + salary_range_b(job, pref) * SR_WF + spontaneity(job, pref) * SP_WF
}

pub fn calc_score_no_pref(job: &Job) -> f64 {
    employer_rating(job) * ER_WF + trend_factor(job) * TF_WF
}

fn employer_rating(job: &Job) -> f64 {
    //potential todo: take into account # of reviews
    //so employer_rating of 5(1) < employer_rating of 5(10)
    job.employer_rating.unwrap_or_default() as f64 / 5.0
}

fn trend_factor(job: &Job) -> f64 {
    let applications = job.applications_count as f64;
    let view_weight: f64;
    let views = job.view_count as f64;
    if views <= 100.0 {
        view_weight = 0.3;
    } else if views > 100.0 && views <= 500.0 {
        view_weight = 1.0;
    } else if views != 0.0 {
        view_weight = 1.5 / (1.00001 - applications / views);
    } else {
        view_weight = 0.0;
    }
    let score = (job.applications_count as f64 + 1.0).log10() / (views + 1.0).log10();
    //println!("For {} the views {} and applications {} add up to the non weighted score {} or weighted score {}", job.job_id, views, applications, score, score * view_weight);
    score * view_weight
}


// V1
/*
    salary >> range => 2
    salary > range  => 2
    salary = max    => 1
    salary in range => (0..1)
    salary = min    => 0
    salary < range: =>-1
    salary << range =>-2
 */
fn salary_range_a(job: &Job, pref: &UserPreferences) -> f64 {
    let min: f64 = pref.salary_range.unwrap_or_default().0;
    let max: f64 = pref.salary_range.unwrap_or_default().1;
    let salary: f64 = job.salary.unwrap_or_default();

    if salary > 0.0 && min >= 0.0 && max > min {
        return ((salary - min) / (max - min)).max(-2.0).min(2.0);
    } else { 0.0 }
}

// V2
fn salary_range_b(job: &Job, pref: &UserPreferences) -> f64 {
    let min: f64 = pref.salary_range.unwrap_or_default().0;
    let max: f64 = pref.salary_range.unwrap_or_default().1;
    let salary: f64 = job.euro_salary.unwrap_or_default();
    let mut res: f64 = 0.0;
    if salary > 0.0 && min >= 0.0 && max > min {
        if salary >= min && salary <= max {
            res = 2.0
        } else if salary < min {
            res = 2.0 * (1.0 - (salary - min).powf(2.0) / min);
            if res < -2.0 { res = -2.0 }
        } else if salary > max { //if is unnecessary and only has informational purposes
            res = 2.0 / (salary / max);
            if res < 0.0001 {
                res = 0.0;
            }
        }
        return res;
    } else { 0.0 }
}

fn spontaneity(job: &Job, pref: &UserPreferences) -> f64 {
    // Get spontaneity preference => x value of peak
    let p: f64 = pref.spontaneity.unwrap_or_default();
    // Parse start_slot
    let start_slot = Utc
        .datetime_from_str(&job.start_slot.trim(), "%Y-%m-%dT%H:%M:%S%.3fZ")
        .expect("Failed to parse datetime string");

    spontaneity_map((start_slot.signed_duration_since(Utc::now()).num_seconds()) as f64, p)
}

// a: time from now to start, b: user preference
fn spontaneity_map(a: f64, b: f64) -> f64 {
    let distance = (a - b).abs();
    if distance > 86400.0 { return 0.0; }
    if distance < 1000.0 { return 2.5; }

    // steepness factor => 0.15; should be adapted if score is to impactful
    let fa = 2.5 - 0.15 * distance.ln();
    return fa.max(0.0).min(2.5);
}



