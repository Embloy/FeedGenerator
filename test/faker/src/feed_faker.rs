////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////FAKER///////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
pub mod feed_faker {
    use backend::controllers::models::{Job, UserPreferences};
    use backend::ranking_algorithms::ranker::generate_job_feed;


    fn generate_slice() -> Vec<Job> {
        return nil;
    }

    fn generate_job() -> Job {
        return nil;
    }

    fn generate_pref() -> Option<UserPreferences> {
        return nil;
    }

    fn write_feed(res: Vec<Job>) {}

    fn generate_feeds() {
        for i in 0..1000 {
            println!("pref #{}", i);
            let pref = generate_pref();

            for j in 0.100 {
                let slice = generate_slice();
                write_feed(generate_job_feed(slice, pref.clone()));
            }
        }
    }
}
