use std::collections::LinkedList;

use crate::job_type_matrix::query;
use crate::models::{Job, UserPreferences};

const NUM_JOB_TYPES: i32 = 27; // Todo: replace with dynamical value based on matrix

pub(crate) fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    let x_value = &pref.job_types.iter().find(|&&(job_types, _)| job_types == job.job_type_value)
        .map_or(0.0, |&(_, score)| score);
    let m_score = calc_m_score(job, pref, 3);
    let t_score = m_score * x_value; // Todo: or m_score + x_value??
    println!("for job id {} x_value is {} m_score is {} t_score is {}", job.job_id, x_value,calc_m_score(job, pref, 3),  m_score * x_value);
    t_score
}

fn calc_m_score(job: &Job, pref: &UserPreferences, max_considered_rank: i32) -> f64 { //Todo: make more efficient
    let mut max_m_score = 0.0;
    let mut counter = 0;
    for job_type in &pref.job_types {
        if counter < max_considered_rank {
            let m_score = query(job.job_type_value as f64 - 1.0, job_type.0 as f64 - 1.0);
            if counter == 0 {
                max_m_score = m_score;
            } else if m_score > max_m_score {
                max_m_score = m_score;
            }
            counter = counter + 1;
        } else { break; }
    }
    max_m_score
}

pub(crate) fn calc_x_ranking(pref: &mut UserPreferences) {
    let job_types = &mut pref.job_types;
    let num_jobs_done = job_types.iter().map(|&(_, val)| val).sum::<f64>() as i32;
    for (_key, value) in job_types.iter_mut() {
        let x_value = calc_x_value(*value, num_jobs_done);
        *value = x_value;
    }
    let mut sorted_vec: Vec<_> = job_types.iter().collect();
    sorted_vec.sort_by(|&a, &b| b.1.total_cmp(&a.1));
    let mut sorted_list = LinkedList::new();
    for value in sorted_vec {
        sorted_list.push_back(*value);
    }
    pref.job_types = sorted_list;
}

fn calc_x_value(job_type_pref: f64, num_jobs_done: i32) -> f64 {
    let pref_div_by_num_of_jobs = job_type_pref / num_jobs_done as f64;
    let job_type_weight = 1 as f64 / NUM_JOB_TYPES as f64;
    let x_value = job_type_weight * pref_div_by_num_of_jobs;
    x_value
}
