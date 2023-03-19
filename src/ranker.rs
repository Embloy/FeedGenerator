////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////RANK JOBS//////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: rank jobs
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use crate::models::{Job, UserPreferences};

fn filter_jobs_by_preferences(jobs: &Vec<Job>, preferences: &UserPreferences, sender: Sender<Job>) {
    for job in jobs.iter() {
        if job.job_type == preferences.job_type &&
            job.key_skills.contains(&preferences.key_skills) &&
            job.salary >= preferences.salary_range.0  &&
            job.salary <= preferences.salary_range.1
        {
            sender.send(job.clone()).unwrap();
        }
    }
}

fn sort_jobs_by_relevance(jobs: &mut Vec<Job>, preferences: &UserPreferences) {
    jobs.sort_by(|a, b| {
        let a_score = job_relevance_score(a, preferences);
        let b_score = job_relevance_score(b, preferences);
        b_score.partial_cmp(&a_score).unwrap()
    });
}

fn job_relevance_score(job: &Job, preferences: &UserPreferences) -> f64 {
    let job_type_score = if job.job_type == preferences.job_type { 1.0 } else { 0.0 };
    let key_skills_score = if job.key_skills.contains(&preferences.key_skills) { 1.0 } else { 0.0 };
    let salary_score = (job.salary - preferences.salary_range.0) / (preferences.salary_range.1 - preferences.salary_range.0);
    job_type_score + key_skills_score + salary_score
}

fn generate_job_feed(jobs: Vec<Job>, preferences: UserPreferences) -> Vec<Job> {
    let (sender, receiver): (Sender<Job>, Receiver<Job>) = channel();

    let mut job_threads = Vec::new();
    for chunk in jobs.chunks(jobs.len() / num_cpus::get()) {
        let preferences_clone = preferences.clone();
        let sender_clone = sender.clone();

        let thread = thread::spawn(move || {
            filter_jobs_by_preferences(&chunk.to_vec(), &preferences_clone, sender_clone);
        });

        job_threads.push(thread);
    }

    drop(sender);

    let mut filtered_jobs = Vec::new();
    for job in receiver.iter() {
        filtered_jobs.push(job);
    }

    for thread in job_threads {
        thread.join().unwrap();
    }

    sort_jobs_by_relevance(&mut filtered_jobs, &preferences);
    filtered_jobs
}
