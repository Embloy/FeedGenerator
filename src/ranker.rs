////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////RANK JOBS//////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::models::{Job, UserPreferences};

// TODO: rank jobs
// fn filter_jobs_by_preferences(jobs: &Vec<Job>, preferences: &UserPreferences) -> Vec<Job> {
//     jobs.iter().filter(|job| {
//         job.job_type == preferences.job_type &&
//             job.key_skills.contains(&preferences.key_skills) &&
//             job.salary >= preferences.salary_range.0 &&
//             job.salary <= preferences.salary_range.1
//     }).cloned().collect()
// }

fn sort_jobs_by_relevance(jobs: &mut Vec<Job>, preferences: &UserPreferences) -> Vec<Job> {
    jobs.sort_by(|a, b| {
        let a_score = job_relevance_score(a, preferences);
        let b_score = job_relevance_score(b, preferences);
        b_score.partial_cmp(&a_score).unwrap()
    });
    jobs.clone()
}

fn job_relevance_score(job: &Job, preferences: &UserPreferences) -> f64 {
    let job_type_score = if job.job_type == preferences.job_type { 1.0 } else { 0.0 };
    let key_skills_score = if job.key_skills.contains(&preferences.key_skills) { 1.0 } else { 0.0 };
    let salary_score = (job.salary - preferences.salary_range.0) / (preferences.salary_range.1 - preferences.salary_range.0);
    job_type_score + key_skills_score + salary_score
}

// pub fn generate_job_feed(jobs: Vec<Job>, preferences: UserPreferences) -> Vec<Job> {
//     let mut filtered_jobs = filter_jobs_by_preferences(&jobs, &preferences);
//     sort_jobs_by_relevance(&mut filtered_jobs, &preferences);
//     filtered_jobs
// }

pub fn generate_job_feed(jobs: Vec<Job>, preferences: UserPreferences) -> Vec<Job> {
    sort_jobs_by_relevance(&mut jobs.clone(), &preferences)
}