use chrono::{TimeZone, Utc};
use crate::controllers::models::Job;

fn setup() -> Job {
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

fn teardown() {
    // This function will be called after each test
}

#[cfg(test)]
mod meta_test {
    use crate::controllers::models::{Job, UserPreferences};
    use crate::meta_test::setup;
    use crate::ranking_algorithms::meta::{calc_score, calc_score_no_pref, employer_rating, salary_range, spontaneity, spontaneity_map, trend_factor};

    const ER_WF: f64 = 0.2;
    const TF_WF: f64 = 0.5;
    const SR_WF: f64 = 0.2;
    const SP_WF: f64 = 0.1;

    // Helper function to assert floating-point equality within a certain tolerance (could be useful for shadowing-tests)
    fn assert_float_eq(outcome: f64, expected: f64, tolerance: f64) {
        println!("outcome: {outcome}, expected: {expected}");
        assert!((outcome - expected).abs() <= tolerance);
    }

    #[test]
    fn test_calc_score() {
        let job = setup();
        let pref = UserPreferences {
            job_types: Default::default(),
            key_skills: None,
            salary_range: Some((50000.0, 80000.0)),
            spontaneity: Some(500.0),
            // Add other required fields for UserPreferences
            num_jobs_done: None,
        };

        let score = calc_score(&job, &pref);

        let employer_rating_score = job.employer_rating.unwrap_or_default() as f64 / 5.0 * ER_WF;
        let trend_factor_score = trend_factor(&job) * TF_WF;
        let salary_range_score = salary_range(&job, &pref) * SR_WF;
        let spontaneity_score = spontaneity(&job, &pref) * SP_WF;

        let expected_score = employer_rating_score + trend_factor_score + salary_range_score + spontaneity_score;

        assert_float_eq(score, expected_score, 0.000001);
    }

    #[test]
    fn test_calc_score_no_pref() {
        let job = setup();

        let score = calc_score_no_pref(&job);

        let employer_rating_score = job.employer_rating.unwrap_or_default() as f64 / 5.0 * ER_WF;
        let trend_factor_score = trend_factor(&job) * TF_WF;
        let expected_score = employer_rating_score + trend_factor_score;

        assert_float_eq(score, expected_score, 0.000001);
    }

    #[test]
    fn test_employer_rating() {
        let job = setup();

        let rating = employer_rating(&job);
        let expected_rating = job.employer_rating.unwrap_or_default() as f64 / 5.0;
        assert_float_eq(rating, expected_rating, 0.000001);
    }

    #[test]
    fn test_trend_factor() {
        let job = setup();

        let factor = trend_factor(&job);
        let expected_factor = 0.7423858694304362;
        assert_float_eq(factor, expected_factor, 0.000001);
    }

    #[test]
    fn test_salary_range() {
        let job = setup();
        let pref = UserPreferences {
            job_types: Default::default(),
            key_skills: None,
            salary_range: Some((50000.0, 80000.0)),
            // Add other required fields for UserPreferences
            spontaneity: None,
            num_jobs_done: None,
        };

        let range = salary_range(&job, &pref);
        let expected_range = 1.0 / 3.0;
        assert_float_eq(salary_range(&job, &pref), expected_range as f64, 0.000001);
    }

    #[test]
    fn test_spontaneity() {
        let job = setup();
        let pref = UserPreferences {
            job_types: Default::default(),
            key_skills: None,
            salary_range: None,
            spontaneity: Some(500.0),
            // Add other required fields for UserPreferences
            num_jobs_done: None,
        };

        let spontaneity = spontaneity(&job, &pref);
        let expected_spontaneity = 2.5;
        assert_float_eq(spontaneity, expected_spontaneity, 0.000001);
    }

    #[test]
    fn test_spontaneity_map() {
        let job = setup();
        let a = 1000.0;
        let b = 500.0;

        let score = spontaneity_map(a, b);
        let expected_score = 2.5;
        assert_float_eq(score, expected_score, 0.000001);
    }
}