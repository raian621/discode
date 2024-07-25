
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

const DAILY_PROBLEM_SLUG_QUERY: &str = "query {
    activeDailyCodingChallengeQuestion {,
        link
    }
}";

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct DailyProblemResponse {
    data: ActiveDailyCodingChallengeQuestion
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ActiveDailyCodingChallengeQuestion {
    activeDailyCodingChallengeQuestion: ProblemLink
}

#[derive(Debug, Deserialize)]
struct ProblemLink {
    link: String
}

pub async fn get_daily_problem_slug(client: &reqwest::Client) -> Result<String, Error> {
    let res: DailyProblemResponse = client.post("https://leetcode.com/graphql")
        .json(&json!({"query": DAILY_PROBLEM_SLUG_QUERY}))
        .send()
        .await?
        .json()
        .await?;

    let problem_path = res.data.activeDailyCodingChallengeQuestion.link;
    let problem_slug = problem_path.split('/').collect::<Vec<&str>>()[2];

    Ok(problem_slug.to_string())
}