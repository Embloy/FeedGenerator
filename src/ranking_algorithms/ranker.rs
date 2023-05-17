use crate::controllers::models::{Job, UserPreferences};
use crate::ranking_algorithms::{meta, t_score};

const META_WEIGHT: f64 = 0.3; // Todo: replace with dynamical value based on matrix

// Generate feed based on jobs and user's preferences
pub async fn generate_job_feed(jobs: Vec<Job>, mut preferences: Option<UserPreferences>) -> Vec<Job> {
    // 1. Matching/Scoring
    let mut raw_ranked_slice = rank_jobs(&mut jobs.clone(), &mut preferences);

    /* TODO: 2. Shadowing ...
     network.feed_forward(...);
    */

    // 3. Sorting by relevance score
    sort_by_relevance(&mut raw_ranked_slice)
}


// Assign relevance_score to each job
fn rank_jobs(jobs: &mut Vec<Job>, preferences: &mut Option<UserPreferences>) -> Vec<Job> {
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
    jobs.clone()
}

// Calculate relevance_score for one job
fn job_relevance_score(job: &Job, preferences: &UserPreferences) -> f64 {
    let raw_score = meta::calc_score(job, preferences) * META_WEIGHT + t_score::calc_score(job, preferences) * (1.0 - META_WEIGHT);
    // println!("calc meta score {} calc t_score {}", meta::calc_score(job, preferences), t_score::calc_score(job, preferences));
    // println!("Raw {} for {}", raw_score, job.job_id);
    raw_score
}

// Calculate relevance_score for one job without preferences
fn job_relevance_score_no_pref(job: &Job) -> f64 {
    meta::calc_score_no_pref(job)
}

// Sort all jobs by relevance_score
fn sort_by_relevance(raw_ranked_slice: &mut Vec<Job>) -> Vec<Job> {
    raw_ranked_slice.sort_by(|a, b| {
        let a_score = a.relevance_score.unwrap_or_default();
        let b_score = b.relevance_score.unwrap_or_default();
        b_score.partial_cmp(&a_score).unwrap()
    });
    raw_ranked_slice.clone()
}


