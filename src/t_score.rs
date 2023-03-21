use std::collections::HashMap;
use crate::models::{Job, UserPreferences};

const NUM_JOB_TYPES: i32 = 29;


/*

pub(crate) fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    let mut rank = None;
    for x_ranking in pref.x_ranking.iter() {
        for (i, (jt, _)) in x_ranking.iter().enumerate() {
            if *jt == job.job_type {
                rank = Some(i);
                break;
            }
        }
        if rank.is_some() {
            break;
        }
    }
    let rank = rank.map(|r| r + 1).unwrap_or(0); // add 1 to the index to get the actual rank
    let t_score = rank as f64 * calc_m_score(job, pref);
    t_score
}

fn calc_m_score(job: &Job, pref: &UserPreferences) -> f64 {
    let mut max_m_score = 0.0;
    for job_type in pref.job_type.iter() {
        let m_score = 0.0; // todo: implement matrix
        if m_score > max_m_score {
            max_m_score = m_score;
        }
    }
    max_m_score
}
*/
pub(crate) fn calc_x_ranking(pref: &mut UserPreferences) {
    let job_types = &mut pref.job_type;
    for (key, value) in job_types.iter_mut() {
        let x_value = calc_x_value(*value, pref.num_jobs_done);
        *value = x_value;
    }
    let mut job_type_vec: Vec<(i32, f64)> = pref.job_type.iter().map(|(k, v)| (*k, *v)).collect();
    job_type_vec.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    pref.job_type = job_type_vec.into_iter().collect();

}


fn calc_x_value(job_type_pref: f64, num_jobs_done: i32) -> f64 {
    let pref_div_by_num_of_jobs = job_type_pref/ num_jobs_done as f64;
    let job_type_weight = 1 as f64 / NUM_JOB_TYPES as f64;
    let x_value = job_type_weight * pref_div_by_num_of_jobs;
    x_value
}
























/*
use std::collections::HashMap;
use crate::models::{Job, UserPreferences};

const NUM_JOB_TYPES: i32 = 29; // just preliminary! todo: remove this and substitute with dynamic value based on actual matrix


pub(crate) fn calc_score(job: &Job, pref: &UserPreferences) -> f64 {
    let mut x_value_rank = None;
    for (i, (jt, _)) in x_ranking.iter().enumerate() {
        if *jt == job_type {
            x_value_ran = Some(i);
            break;
        }
    }
    x_value_rank.map(|r| r + 1); // add 1 to the index to get the actual rank
    let t_score = rank as f64 * calc_m_score(job, pref);
    t_score
}

fn calc_m_score(job: &Job, pref: &UserPreferences) -> f64 {
    // todo: implement matrix
    let mut max_m_score = 0.0;
    for job_type in &pref.x_ranking{
        // match *job_type with matrix //OLD:calc_m_score(job.job_type, *job_type );
        let m_score = 0.0;
        if m_score > max_m_score {max_m_score = m_score;}
    }
    max_m_score
}

pub(crate) fn calc_x_ranking(pref: &mut UserPreferences) {
    let mut x_ranking = HashMap::new();
    for job_type_pref in &pref.job_type {
        //x_ranking.insert(job_type_pref.keys(), calc_x_value(job_type_pref.1, pref.num_jobs_done as f64));
        let x_value = calc_x_value(job_type_pref.1, pref.num_jobs_done as f64);
        x_ranking.push((*job_type, x_value));
    }
    let mut sort_x_ranking: Vec<(&i32, &f64)> = x_ranking.iter().collect();
    sort_x_ranking.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let top_x_ranking = sort_x_ranking.iter()
        .rev() // reverse the order so that we start with the highest ranked elements
        .map(|(job_type, x_value)| (**job_type, *x_value))
        .collect::<Vec<(i32, f64)>>();
    pref.x_ranking = Some(top_x_ranking);
}

fn calc_x_value(job_type_pref: &i32, num_jobs_done: f64) -> f64 {
    let pref_div_by_num_of_jobs = *job_type_pref as f64 / num_jobs_done; // num of jobs for each type / num of jobs done totally
    let job_type_weight = 1 as f64 / NUM_JOB_TYPES as f64;
    let x_value = job_type_weight * pref_div_by_num_of_jobs; // can be understood as expectation value
    x_value
}

*/


