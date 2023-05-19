#[cfg(test)]
pub mod test_setup {
    use std::fs::File;
    use std::io::Read;
    use chrono::Utc;
    use backend::controllers::models::{Job, UserPreferences};
    use serde_json::Value;
    use serde_json::from_value;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Res {
        pub employer_rating: f64,
        pub trend_factor: f64,
        pub salary_range: f64,
        pub spontaneity: f64,
    }

    // Read JSON test data
    fn read_test_data(path: String) -> Value {
        // Ignore
        // if let Ok(current_dir) = env::current_dir() {
        //     println!("Current working directory: {}", current_dir.display());
        // } else {
        //     println!("Failed to retrieve the current working directory.");
        // }
        let mut file = File::open(path).expect("Failed to open the file");
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).expect("Failed to read the file");
        serde_json::from_str(&json_str).unwrap()
    }

    // This is only a very basic test setup to check whether the tests run as expected.
    pub fn setup_job_basic() -> Job {
        let mut job: Job = from_value(read_test_data(String::from("data/jobs_basic.json"))).unwrap();
        job.start_slot = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        job
    }

    pub fn setup_pref_basic() -> UserPreferences {
        let pref: UserPreferences = from_value(read_test_data(String::from("data/pref_basic.json"))).unwrap();
        pref
    }

    pub fn setup_res_basic() -> Res {
        let res: Res = from_value(read_test_data(String::from("data/res_basic.json"))).unwrap();
        res
    }

    /* TODO:
        Write Unit tests that cover 4 feed requests with 10 jobs each.
        Define scenarios and parse JSON containing jobs & preferences:
            => 3 test-scenarios (valid normal, edge-case, invalid)
            => 4 feed requests per scenario
            => 40 job & 4 pref JSONs per scenario
            => 120 job & 12 pref JSOBs in total
    */

    // TODO: Valid normal input
    pub fn setup_jobs(test_scenario: &str) -> Vec<Vec<Job>> {
        let mut jobs: Vec<Vec<Job>> = from_value(read_test_data("data/jobs_".to_owned() + test_scenario + ".json")).unwrap();
        for slice in jobs.iter_mut() {
            for job in slice.iter_mut() {
                job.start_slot = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            }
        }
        jobs
    }

    pub fn setup_pref(test_scenario: &str) -> Vec<UserPreferences> {
        let preferences: Vec<UserPreferences> = from_value(read_test_data("data/pref_".to_owned() + test_scenario + ".json")).unwrap();
        preferences
    }

    pub fn _teardown() {
        // This function will be called after each test
    }
}
