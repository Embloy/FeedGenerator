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
    use crate::common::test_setup::{setup_job_basic, setup_pref_basic, setup_res_basic};
    use crate::meta_test::{assert_float_eq, ER_WF, SP_WF, SR_WF, TF_WF};

    #[test]
    fn calc_score_basic() {
        let job = setup_job_basic();
        let pref = setup_pref_basic();

        let score = calc_score(&job, &pref);

        let employer_rating_score = employer_rating(&job) * ER_WF;
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

        let employer_rating_score = employer_rating(&job) * ER_WF;
        let trend_factor_score = trend_factor(&job) * TF_WF;
        let expected_score = employer_rating_score + trend_factor_score;

        assert_float_eq(score, expected_score, 0.000001);
    }

    #[test]
    fn employer_rating_basic() {
        let job = setup_job_basic();

        let rating = employer_rating(&job);
        let expected_rating = setup_res_basic().employer_rating;
        assert_float_eq(rating, expected_rating, 0.000001);
    }

    #[test]
    fn trend_factor_basic() {
        let job = setup_job_basic();

        let factor = trend_factor(&job);
        let expected_factor = setup_res_basic().trend_factor;

        assert_float_eq(factor, expected_factor, 0.000001);
    }

    #[test]
    fn salary_range_basic() {
        let job = setup_job_basic();
        let pref = setup_pref_basic();

        let range = salary_range(&job, &pref);
        let expected_range = setup_res_basic().salary_range;
        assert_float_eq(range, expected_range as f64, 0.000001);
    }

    #[test]
    fn spontaneity_basic() {
        let job = setup_job_basic();
        let pref = setup_pref_basic();

        let spontaneity = spontaneity(&job, &pref);
        let expected_spontaneity = setup_res_basic().spontaneity;
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
    use backend::ranking_algorithms::meta::{calc_score, calc_score_no_pref, employer_rating, salary_range, spontaneity, trend_factor};
    use crate::common::test_setup::{setup_jobs, setup_pref, setup_res};
    use crate::meta_test::{assert_float_eq, ER_WF, SP_WF, SR_WF, TF_WF};

    #[test]
    fn calc_score_valid() {
        let slice = setup_jobs("valid");
        let preferences = setup_pref("valid");

        for pref in &preferences {
            for job in &slice {
                let score = calc_score(&job, &pref);

                let employer_rating_score = employer_rating(&job) * ER_WF;
                let trend_factor_score = trend_factor(&job) * TF_WF;
                let salary_range_score = salary_range(&job, &pref) * SR_WF;
                let spontaneity_score = spontaneity(&job, &pref) * SP_WF;
                let expected_score = employer_rating_score + trend_factor_score + salary_range_score + spontaneity_score;

                println!("for job #{}: employer_score is {} | trend_factor is {} | salary_range is {} | spontaneity is {} | score is {}", job.job_id, employer_rating_score, trend_factor_score, salary_range_score, spontaneity_score, score);
                assert_float_eq(score, expected_score, 0.000001);
            }
        }
    }

    #[test]
    fn calc_score_no_pref_valid() {
        let slice = setup_jobs("valid");

        for job in &slice {
            let score = calc_score_no_pref(&job);

            let employer_rating_score = employer_rating(&job) * ER_WF;
            let trend_factor_score = trend_factor(&job) * TF_WF;
            let expected_score = employer_rating_score + trend_factor_score;

            println!("for job #{}:\t employer_score is {} | trend_factor is {} | score is {}", job.job_id, employer_rating_score, trend_factor_score, score);
            assert_float_eq(score, expected_score, 0.000001);
        }
    }

    #[test]
    fn employer_rating_valid() {
        let slice = setup_jobs("valid");
        let res = setup_res("valid");

        for (i, job) in slice.iter().enumerate() {
            let rating = employer_rating(&job);
            let expected_rating = res[0][i].employer_rating;
            println!("For job #{} with {} stars, the rating is {}", job.job_id, job.employer_rating.unwrap(), rating);
            assert_float_eq(rating, expected_rating, 0.000001);
        }
    }

    #[test]
    fn trend_factor_valid() {
        let slice = setup_jobs("valid");
        let res = setup_res("valid");

        for (i, job) in slice.iter().enumerate() {
            let factor = trend_factor(&job);
            let expected_factor = res[0][i].trend_factor;

            println!("For job #{} the views {} and applications {} add up to the non weighted factor {}", job.job_id, job.view_count, job.applications_count, factor);
            assert_float_eq(factor, expected_factor, 0.000001);
        }
    }

    #[test]
    fn salary_range_valid() {
        let slice = setup_jobs("valid");
        let preferences = setup_pref("valid");
        let res = setup_res("valid");

        for (i, pref) in preferences.iter().enumerate() {
            for (j, job) in slice.iter().enumerate() {
                let range = salary_range(&job, &pref);
                let expected_range = res[i][j].salary_range;

                println!("For job #{} with salary {} and pref #{} with range [{},{}] salary_range is {}", job.job_id, job.salary.unwrap(), pref.id.unwrap(), pref.salary_range.unwrap().0, pref.salary_range.unwrap().1, range);
                assert_float_eq(range, expected_range as f64, 0.000001);
            }
        }
    }

    #[test]
    fn spontaneity_valid() {
        let slice = setup_jobs("valid");
        let preferences = setup_pref("valid");
        let res = setup_res("valid");

        for (i, pref) in preferences.iter().enumerate() {
            for (j, job) in slice.iter().enumerate() {
                let spontaneity = spontaneity(&job, &pref);
                let expected_spontaneity = res[i][j].spontaneity;
                println!("For job #{} with start_slot {} and pref #{} with spontaneity {} factor is {}", job.job_id, job.start_slot, pref.id.unwrap(), pref.spontaneity.unwrap(), spontaneity);
                assert_float_eq(spontaneity, expected_spontaneity, 0.000001);
            }
        }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////EDGE-CASE-TEST///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
// TODO: Edge-case tests
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
// TODO: Invalid tests
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
