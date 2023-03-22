use std::collections::HashMap;

use num_cpus::get;

use crate::models::{Job, UserPreferences};
use crate::job_type_matrix::query;

const NUM_JOB_TYPES: i32 = 27; // todo: replace with dynamical value based on matrix

pub(crate) fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    //let x_value_rank = pref.job_type.keys().position(|k| k == &job.job_type).unwrap() + 1;
    let x_value = pref.job_type.get(&job.job_type_value).unwrap_or(&0.0);
    let m_score = calc_m_score(job, pref, 3);
    let t_score = m_score * x_value;
    t_score
}

fn calc_m_score(job: &Job, pref: &UserPreferences, considered_rank: i32) -> f64 { //todo: make more efficient
    let mut max_m_score = 0.0;
    let mut counter = 0;
    for job_type in pref.job_type.iter() {
        if counter < considered_rank {
            let m_score = query(job.job_type_value - 1, *job_type.0 - 1) as f64;
            if m_score > max_m_score {
                max_m_score = m_score;
            }
            counter = counter + 1;
        } else { break; }
    }
    max_m_score
}

pub(crate) fn calc_x_ranking(pref: &mut UserPreferences) {
    let job_types = &mut pref.job_type;
    for (key, value) in job_types.iter_mut() {
        let x_value = calc_x_value(*value, pref.num_jobs_done.unwrap_or_default());
        *value = x_value;
    }
    let mut job_type_vec: Vec<(i32, f64)> = pref.job_type.iter().map(|(k, v)| (*k, *v)).collect();
    job_type_vec.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    pref.job_type = job_type_vec.into_iter().collect();
}

fn calc_x_value(job_type_pref: f64, num_jobs_done: i32) -> f64 {
    let pref_div_by_num_of_jobs = job_type_pref / num_jobs_done as f64;
    let job_type_weight = 1 as f64 / NUM_JOB_TYPES as f64;
    let x_value = job_type_weight * pref_div_by_num_of_jobs;
    x_value
}


