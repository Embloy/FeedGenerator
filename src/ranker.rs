////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////RANK JOBS//////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{meta, t_score};
use crate::models::{Job, UserPreferences};

fn sort_jobs_by_relevance(jobs: &mut Vec<Job>, preferences: &mut Option<UserPreferences>) -> Vec<Job> {
    if preferences.is_some() {
        //println!("test {:?}", preferences.clone().unwrap());
        t_score::calc_x_ranking(&mut preferences.as_mut().unwrap());
        //println!("test 2 {:?}", preferences.clone().unwrap());

        jobs.sort_by(|a, b| {
            let a_score = job_relevance_score(a, &preferences.clone().unwrap());
            let b_score = job_relevance_score(b, &preferences.clone().unwrap());
            b_score.partial_cmp(&a_score).unwrap()
        });
    } else {
        jobs.sort_by(|a, b| {
            let a_score = job_relevance_score_no_pref(a);
            let b_score = job_relevance_score_no_pref(b);
            b_score.partial_cmp(&a_score).unwrap()
        });
    }
    jobs.clone()
}

fn job_relevance_score(job: &Job, preferences: &UserPreferences) -> f64 {
    let x = 0.3;
    //println!("calc meta score {} calc t_score {}", meta::calc_score(job, preferences), t_score::calc_score(job, preferences));
    meta::calc_score(job, preferences) * x + t_score::calc_score(job, preferences) * (1.0 - x)
}

fn job_relevance_score_no_pref(job: &Job) -> f64 {
    meta::calc_score_no_pref(job)
}

pub fn generate_job_feed(jobs: Vec<Job>, mut preferences: Option<UserPreferences>) -> Vec<Job> {
    sort_jobs_by_relevance(&mut jobs.clone(), &mut preferences)
    // TODO: Shadowing ...
}
