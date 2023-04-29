use crate::{meta, t_score};
use crate::models::{Job, UserPreferences};

fn sort_jobs_by_relevance(jobs: &mut Vec<Job>, preferences: &mut Option<UserPreferences>) -> Vec<Job> {
    if preferences.is_some() {
        t_score::calc_x_ranking(&mut preferences.as_mut().unwrap());
        for x in &mut *jobs {
            x.relevance_score = Option::from(job_relevance_score(x, &preferences.clone().unwrap()));
        }
    } else {
        for x in &mut *jobs {
            x.relevance_score = Option::from(job_relevance_score_no_pref(x));
        }
    }

    jobs.sort_by(|a, b| {
        let a_score = a.relevance_score.unwrap_or_default();
        let b_score = b.relevance_score.unwrap_or_default();
        b_score.partial_cmp(&a_score).unwrap()
    });
    jobs.clone()
}

fn job_relevance_score(job: &Job, preferences: &UserPreferences) -> f64 {
    let x = 0.3;
    let raw_score = meta::calc_score(job, preferences) * x + t_score::calc_score(job, preferences) * (1.0 - x);
    println!("calc meta score {} calc t_score {}", meta::calc_score(job, preferences), t_score::calc_score(job, preferences));
    println!("Raw {} for {}", raw_score, job.job_id);
    raw_score
}

fn job_relevance_score_no_pref(job: &Job) -> f64 {
    meta::calc_score_no_pref(job)
}

pub async fn generate_job_feed(jobs: Vec<Job>, mut preferences: Option<UserPreferences>) -> Vec<Job> {
    let raw_ranked_slice = sort_jobs_by_relevance(&mut jobs.clone(), &mut preferences);
    raw_ranked_slice
    // TODO: Shadowing ...
}
