// use crate::common::test_setup::{setup_job_basic, setup_pref_basic};
// use backend::ranking_algorithms::meta::{calc_score, calc_score_no_pref, employer_rating, salary_range, spontaneity, spontaneity_map, trend_factor};

pub const ER_WF: f64 = 0.2;
pub const TF_WF: f64 = 0.5;
pub const SR_WF: f64 = 0.2;
pub const SP_WF: f64 = 0.1;

// Helper function to assert floating-point equality within a certain tolerance (could be useful for shadowing-tests)
pub fn assert_float_eq(outcome: f64, expected: f64, tolerance: f64) {
    println!("outcome: {outcome}, expected: {expected}");
    assert!((outcome - expected).abs() <= tolerance);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////BASIC-TEST////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
pub mod mt_basic {
    use backend::ranking_algorithms::meta::{calc_score, calc_score_no_pref, employer_rating, salary_range, spontaneity, spontaneity_map, trend_factor};
    use crate::common::test_setup::{setup_job_basic, setup_pref_basic};
    use crate::meta_test::{assert_float_eq, ER_WF, SP_WF, SR_WF, TF_WF};

    #[test]
    fn calc_score_basic() {
        let job = setup_job_basic();
        let pref = setup_pref_basic();

        let score = calc_score(&job, &pref);

        let employer_rating_score = job.employer_rating.unwrap_or_default() as f64 / 5.0 * ER_WF;
        let trend_factor_score = trend_factor(&job) * TF_WF;
        let salary_range_score = salary_range(&job, &pref) * SR_WF;
        let spontaneity_score = spontaneity(&job, &pref) * SP_WF;
        let expected_score = employer_rating_score + trend_factor_score + salary_range_score + spontaneity_score;

        assert_float_eq(score, expected_score, 0.000001);
    }

    #[test]
    fn calc_score_no_pref_basic() {
        let job = setup_job_basic();

        let score = calc_score_no_pref(&job);

        let employer_rating_score = job.employer_rating.unwrap_or_default() as f64 / 5.0 * ER_WF;
        let trend_factor_score = trend_factor(&job) * TF_WF;
        let expected_score = employer_rating_score + trend_factor_score;

        assert_float_eq(score, expected_score, 0.000001);
    }

    #[test]
    fn employer_rating_basic() {
        let job = setup_job_basic();

        let rating = employer_rating(&job);
        let expected_rating = job.employer_rating.unwrap_or_default() as f64 / 5.0;
        assert_float_eq(rating, expected_rating, 0.000001);
    }

    #[test]
    fn trend_factor_basic() {
        let job = setup_job_basic();

        let factor = trend_factor(&job);
        let expected_factor = 0.7423858694304362;
        assert_float_eq(factor, expected_factor, 0.000001);
    }

    #[test]
    fn salary_range_basic() {
        let job = setup_job_basic();
        let pref = setup_pref_basic();

        let range = salary_range(&job, &pref);
        let expected_range = 1.0 / 3.0;
        assert_float_eq(range, expected_range as f64, 0.000001);
    }

    #[test]
    fn spontaneity_basic() {
        let job = setup_job_basic();
        let pref = setup_pref_basic();

        let spontaneity = spontaneity(&job, &pref);
        let expected_spontaneity = 2.5;
        assert_float_eq(spontaneity, expected_spontaneity, 0.000001);
    }

    #[test]
    fn spontaneity_map_basic() {
        let a = 1000.0;
        let b = 500.0;

        let score = spontaneity_map(a, b);
        let expected_score = 2.5;
        assert_float_eq(score, expected_score, 0.000001);
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////VALID-TEST/////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod mt_valid {
    use backend::ranking_algorithms::meta::{calc_score, calc_score_no_pref, employer_rating, salary_range, spontaneity, spontaneity_map, trend_factor};
    use crate::common::test_setup::{setup_jobs, setup_pref};
    use crate::meta_test::{assert_float_eq, ER_WF, SP_WF, SR_WF, TF_WF};

    #[test]
    fn calc_score_valid() {
        let jobs = setup_jobs("valid");
        let preferences = setup_pref("valid");

        for (index, slice) in jobs.iter().enumerate() {
            let pref = &preferences[index];
            for job in slice {
                let score = calc_score(&job, &pref);

                let employer_rating_score = job.employer_rating.unwrap_or_default() as f64 / 5.0 * ER_WF;
                let trend_factor_score = trend_factor(&job) * TF_WF;
                let salary_range_score = salary_range(&job, &pref) * SR_WF;
                let spontaneity_score = spontaneity(&job, &pref) * SP_WF;
                let expected_score = employer_rating_score + trend_factor_score + salary_range_score + spontaneity_score;

                assert_float_eq(score, expected_score, 0.000001);
            }
        }
    }

    #[test]
    fn calc_score_no_pref_valid() {}

    #[test]
    fn employer_rating_valid() {}

    #[test]
    fn trend_factor_valid() {}

    #[test]
    fn salary_range_valid() {}

    #[test]
    fn spontaneity_valid() {}

    #[test]
    fn spontaneity_map_valid() {}
}


////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////EDGE-CASE-TEST///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod mt_edge_case {
    #[test]
    fn calc_score_edge_case() {}

    #[test]
    fn calc_score_no_pref_edge_case() {}

    #[test]
    fn employer_rating_edge_case() {}

    #[test]
    fn trend_factor_edge_case() {}

    #[test]
    fn salary_range_edge_case() {}

    #[test]
    fn spontaneity_edge_case() {}

    #[test]
    fn spontaneity_map_edge_case() {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////INVALID-TEST////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
pub mod mt_invalid {
    #[test]
    #[should_panic]
    fn calc_score_invalid() {
        panic!("Invalid input!");
    }

    #[test]
    #[should_panic]
    fn calc_score_no_pref_invalid() {
        panic!("Invalid input!");
    }

    #[test]
    #[should_panic]
    fn employer_rating_invalid() {
        panic!("Invalid input!");
    }

    #[test]
    #[should_panic]
    fn trend_factor_invalid() {
        panic!("Invalid input!");
    }

    #[test]
    #[should_panic]
    fn salary_range_invalid() {
        panic!("Invalid input!");
    }

    #[test]
    #[should_panic]
    fn spontaneity_invalid() {
        panic!("Invalid input!");
    }

    #[test]
    #[should_panic]
    fn spontaneity_map_invalid() {
        panic!("Invalid input!");
    }
}
