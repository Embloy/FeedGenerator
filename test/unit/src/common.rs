#[cfg(test)]
pub mod test_setup {
    use chrono::Utc;
    use backend::controllers::models::{Job, UserPreferences};

    // This is only a very basic test setup to check whether the tests run as expected.
    pub fn setup_job_basic() -> Job {
        let job = Job {
            job_id: 0,
            job_type_value: 0,
            job_type: "".to_string(),
            job_status: 0,
            status: "".to_string(),
            user_id: 0,
            duration: 0,
            code_lang: None,
            title: "".to_string(),
            position: None,
            description: "".to_string(),
            key_skills: None,
            salary: Some(60000.0),
            currency: None,
            euro_salary: None,
            image_url: None,
            start_slot: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            longitude: 0.0,
            latitude: 0.0,
            country_code: None,
            postal_code: None,
            city: None,
            address: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            applications_count: 100,
            view_count: 500,
            job_notifications: None,
            employer_rating: Some(4),
            boost: None,
            relevance_score: None,
        };
        job
    }

    pub fn setup_pref_basic() -> UserPreferences {
        let pref = UserPreferences {
            job_types: Default::default(),
            key_skills: None,
            salary_range: Some((50000.0, 80000.0)),
            spontaneity: Some(500.0),
            num_jobs_done: None,
        };
        pref
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
    // pub fn setup_jobs_valid() -> vec<vec<Job>> {
        // parse jobs_valid.json
        // return vector of slices (e.g, 4 * 10 jobs)
    // }

    // pub fn setup_pref_valid() -> vec<UserPreferences> {
        // parse pref_valid.json
        // return vector of preferences (e.g, 4 preferences)
    // }

    // TODO: Valid edge-case input
    // pub fn setup_jobs_edge_case() -> vec<vec<Job>> {}

    // pub fn setup_pref_edge_case() -> vec<UserPreferences> {}

    // TODO: Invalid input
    // pub fn setup_jobs_invalid() -> vec<vec<Job>> {}

    // pub fn setup_pref_invalid() -> vec<UserPreferences> {}

    pub fn _teardown() {
        // This function will be called after each test
    }
}
