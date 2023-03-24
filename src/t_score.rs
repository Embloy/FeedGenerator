
use crate::models::{Job, UserPreferences};
use crate::job_type_matrix::query;

const NUM_JOB_TYPES: i32 = 27; // todo: replace with dynamical value based on matrix

pub(crate) fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    //println!("2. {:?} an 21 {:?}", &pref, &pref.job_type.get(&21));
    //println!("job type value is {} for job {}", &job.job_type_value, job.job_id);
    //todo: bugfix
    //let x_value_rank = pref.job_type.keys().position(|k| k == &job.job_type).unwrap() + 1;
    //let x_value = &pref.job_type.get(&job.job_type_value).unwrap();
    let x_value = pref.job_type.get(&job.job_type_value).map_or(0.0, |score| *score);
    let m_score = calc_m_score(job, pref, 3);
    let t_score = m_score * x_value;
    println!("for job id {} x_value is {} m_score is {} t_score is {}", job.job_id, (pref.job_type.get(&job.job_type_value).unwrap_or(&0.0)), calc_m_score(job, pref, 3),  m_score * x_value);
    t_score
}

fn calc_m_score(job: &Job, pref: &UserPreferences, max_considered_rank: i32) -> f64 { //todo: make more efficient
    let mut max_m_score = 0.0;
    let mut counter = 0;
    for job_type in &pref.job_type {
        if counter < max_considered_rank {
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
    println!("1.");
    let job_types = &mut pref.job_type;
    let num_jobs_done = job_types.values().map(|&val| val as i32).sum();
    for (key, value) in job_types.iter_mut() {
        let x_value = calc_x_value(*value, num_jobs_done);
        *value = x_value;
    }
    let mut job_type_vec: Vec<(i32, f64)> = pref.job_type.iter().map(|(k, v)| (*k, *v)).collect();
    job_type_vec.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    pref.job_type = job_type_vec.into_iter().collect();
}

fn calc_x_value(job_type_pref: f64, num_jobs_done: i32) -> f64 {
    //println!("input {},{}", job_type_pref, num_jobs_done);
    let pref_div_by_num_of_jobs = job_type_pref / num_jobs_done as f64;
    let job_type_weight = 1 as f64 / NUM_JOB_TYPES as f64;
    let x_value = job_type_weight * pref_div_by_num_of_jobs;
    x_value
}


