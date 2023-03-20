////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////RANK JOBS//////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{meta, t_score};
use crate::models::{Job, UserPreferences};

fn sort_jobs_by_relevance(jobs: &mut Vec<Job>, preferences: &Option<UserPreferences>) -> Vec<Job> {
    jobs.sort_by(|a, b| {
        let a_score = job_relevance_score(a, preferences);
        let b_score = job_relevance_score(b, preferences);
        b_score.partial_cmp(&a_score).unwrap()
    });
    jobs.clone()
}

fn job_relevance_score(job: &Job, preferences: &Option<UserPreferences>) -> f64 {
    if preferences.is_some() {
        let x = 0.5;
        meta::calc_score(job, &preferences.clone().unwrap()) * x + t_score::calc_score(job, &preferences.clone().unwrap()) * (1.0 - x)
    } else {
        println!("ERROR! [NO PREFERENCES FOUND]");
        meta::calc_score_no_pref(job)
    }
}

pub fn generate_job_feed(jobs: Vec<Job>, preferences: Option<UserPreferences>) -> Vec<Job> {
    sort_jobs_by_relevance(&mut jobs.clone(), &preferences)
    // TODO: Shadowing ...
}
