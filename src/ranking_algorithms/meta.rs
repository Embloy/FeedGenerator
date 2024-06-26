// ER: [0,1] * 0.5
// TF: [0,10] * 0.2
// SR: [-2,2] * 0.2
// SP: [0,1] * 0.1
// => MIN META-SCORE = -0.4
// => MAX META-SCORE = 3

use chrono::{TimeZone, Utc};

use crate::controllers::models::{Job, UserPreferences};

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

pub fn employer_rating(job: &Job) -> f64 {
    //potential TODO: Take into account # of reviews
    //so employer_rating of 5(1) < employer_rating of 5(10)
    job.employer_rating.unwrap_or_default() as f64 / 5.0
}

pub fn trend_factor(job: &Job) -> f64 {
    let applications = job.applications_count as f64;
    let views = job.view_count as f64;

    if applications > views || (views == 0.0 && applications == 0.0) { return 0.0; }
    let view_weight = if views <= 100.0 {
        0.3
    } else if views <= 500.0 {
        1.0
    } else { 1.5 };

    let score = (applications + 1.000001).log10() / (views + 1.000001).log10();
    score * view_weight
}

// V1
/*
    WITH VALID INPUTS:
       salary >> range => 2
       salary > range  => (1,2)
       salary = max    => 1
       salary in range => [0,1]
       salary = min    => 0
       salary < range: => (-2,0)
       salary << range =>-2
    ELSE => 0

 */
pub fn salary_range(job: &Job, pref: &UserPreferences) -> f64 {
    let min: f64 = pref.salary_range.unwrap_or_default().0;
    let max: f64 = pref.salary_range.unwrap_or_default().1;
    let salary: f64 = job.salary.unwrap_or_default();
    if salary > 0.0 && min >= 0.0 && max > min {
        return ((salary - min) / (max - min)).max(-2.0).min(2.0);
    } else { 0.0 }
}

// V2
pub fn salary_range_b(job: &Job, pref: &UserPreferences) -> f64 {
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

pub fn spontaneity(job: &Job, pref: &UserPreferences) -> f64 {
    // Get spontaneity preference => x value of peak
    let p: f64 = pref.spontaneity.unwrap_or_default();
    // Parse start_slot
    let start_slot = Utc
        .datetime_from_str(&job.start_slot.trim(), "%Y-%m-%dT%H:%M:%S%.3fZ")
        .expect("Failed to parse datetime string");

    return spontaneity_map((start_slot.signed_duration_since(Utc::now()).num_seconds()) as f64, p);
}

// a: time from now to start, b: user preference
pub fn spontaneity_map(a: f64, b: f64) -> f64 {
    let distance = (a - b).abs();
    if distance > 86400.0 { return 0.0; }
    if distance < 1000.0 { return 2.5; }

    // steepness factor => 0.15; should be adapted if score is to impactful
    let fa = 2.5 - 0.15 * distance.ln();
    return fa.max(0.0).min(2.5);
}



