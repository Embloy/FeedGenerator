use actix_web::body::MessageBody;
use serde::Serialize;

use crate::{logger, meta, t_score};
use crate::models::{FeedLog, Job, UserPreferences};

// fn sort_jobs_by_relevance(jobs: &mut Vec<Job>, preferences: &mut Option<UserPreferences>) -> Vec<Job> {
//     if preferences.is_some() {
//         //println!("test {:?}", preferences.clone().unwrap());
//         t_score::calc_x_ranking(&mut preferences.as_mut().unwrap());
//         //println!("test 2 {:?}", preferences.clone().unwrap());
//
//         jobs.sort_by(|a, b| {
//             let a_score = job_relevance_score(a, &preferences.clone().unwrap());
//             let b_score = job_relevance_score(b, &preferences.clone().unwrap());
//             b_score.partial_cmp(&a_score).unwrap()
//         });
//     } else {
//         jobs.sort_by(|a, b| {
//             let a_score = job_relevance_score_no_pref(a);
//             let b_score = job_relevance_score_no_pref(b);
//             b_score.partial_cmp(&a_score).unwrap()
//         });
//     }
//     jobs.clone()
// }

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
    //println!("calc meta score {} calc t_score {}", meta::calc_score(job, preferences), t_score::calc_score(job, preferences));
    //println!("Raw {} for {}", raw_score, job.job_id);
    raw_score
}

fn job_relevance_score_no_pref(job: &Job) -> f64 {
    meta::calc_score_no_pref(job)
}

pub fn generate_job_feed(jobs: Vec<Job>, mut preferences: Option<UserPreferences>) -> Vec<Job> {
    // Deprecated
    let raw_ranked_slice = sort_jobs_by_relevance(&mut jobs.clone(), &mut preferences);
    let log = FeedLog { pref: preferences.clone(), sorted_slice: raw_ranked_slice.clone() };
    logger::add_to_feed_log(log, "raw_ranking", 200);
    raw_ranked_slice
    // TODO: Shadowing ...
}
