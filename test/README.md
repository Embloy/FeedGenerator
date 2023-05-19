# Unit Tests for Feed-Generator Backend

This directory contains unit tests that cover various scenarios for feed requests. The tests are designed to parse JSON
data containing job and preference information and ensure the expected behavior of the system.

## Test Scenarios

The unit tests are organized into three main scenarios:

> 1. **Valid Normal Scenario**: This scenario represents a typical feed request with valid job and preference data.

> 2. **Edge-case Scenario**: This scenario tests the system's behavior when it encounters edge-case data or unusual
     input.

> 3. **Invalid Scenario**: This scenario focuses on handling invalid data and ensuring proper error handling and
     validation.

## Feed Requests

For each scenario, there are four feed requests. Each feed request is expected to process three jobs. Therefore, there
are
a total of 12 job JSONs per scenario.

## JSON Data

The [data](../test/unit/data) directory contains JSON files for both jobs and preferences. Here's the breakdown:

- Job JSONs (serialized `Job` object):
    - Valid Normal Scenario: 10 job JSONs
    - Edge-case Scenario: 10 job JSONs
    - Invalid Scenario: 10 job JSONs

- Preference JSONs (serialized `UserPreferences` object):
    - Valid Normal Scenario: 4 preference JSONs
    - Edge-case Scenario: 4 preference JSONs
    - Invalid Scenario: 4 preference JSONs

- Result JSONs (serialized `Res` object):
    - Valid Normal Scenario: 40 result JSONs
    - Edge-case Scenario: 40 result JSONs
    - Invalid Scenario: 40 result JSONs

Each scenario covers 4 feed requests (4 different preferences) with 10 jobs each (jobs remain equal for each request).
So to summarize: There are 40 ranked-jobs per scenario 120 ranked-jobs in total.

## Running the Tests

To run the unit tests, follow these steps:

1. Ensure that you have the necessary dependencies and test framework set up.

2. Navigate to the [test directory](../test/unit) in your command line or terminal (`cd test/unit`).

3. Run `cargo test` (or `cd test/unit; cargo test`).

4. The tests will execute, and you will see the test results and any failures or errors encountered.

## TODO

- Unit
    - [ ] Write 10/10 edge-case job JSONs
    - [ ] Write 4/4 edge-case pref JSONs
    - [ ] Write 40/40 edge-case res JSONs
    - [ ] Write 10/10 invalid job JSONs
    - [ ] Write 4/4 invalid pref JSONs
    - [ ] Write 40/40 invalid res JSONs
- Faker
    - [ ] Implement functionality to store test results (or generally ranked feed) in .csv (or similar) file to analyze
      further (e.g., use "Faker" to generate 10.000 jobs & 100 preferences and have resulting feeds stored in Excel file
      to analyze in R)

---
© Carlo Bortolan, Jan Hummel

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [@carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [@bortolanoffice@embloy.com](bortolanoffice@embloy.com)
>
> Jan Hummel &nbsp;&middot;&nbsp;
> GitHub [@github4touchdouble](https://github.com/github4touchdouble) &nbsp;&middot;&nbsp;
> contact via [@hummeloffice@embloy.com](hummeloffice@embloy.com)



