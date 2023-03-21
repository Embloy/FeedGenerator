//todo: add type

use std::collections::HashMap;

const NUM_JOB_TYPES: f32 = 29; // just preliminary! todo: remove this and substitute with dynamic value based on actual matrix





fn calc_X_Ranking(user_preferences: &UserPreferences) -> Vec<i32>{
    let mut X_Ranking = HashMap::new();
    for job_type_pref in user_preferences.job_type_prefs{

        X_Ranking.insert(job_type_pref.key(), calc_X_Value(job_type_pref.value(), user_preferences.num_jobs_done))
    }
    let mut sort_X_Ranking: Vec<(&i32, &f32)> = X_Ranking.iter().collect();
    sort_X_Ranking.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    sort_X_Ranking.iter().map(|(job_type, _)| **job_type).collect()
}



fn calc_X_Value(job_type_pref: f32, num_jobs_done: f32)  -> f32{
    let pref_div_by_num_of_jobs = job_type_pref / num_jobs_done; // num of jbos for each type / num of jobs done totally
    let job_type_weight = 1 as f32 / NUM_JOB_TYPES;
    X_Value = job_type_weight * pref_div_by_num_of_jobs // can be understood as expectation value
}




