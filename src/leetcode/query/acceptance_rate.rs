use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::json;

const ACCEPTANCE_RATE_QUERY: &str = "\
query questionStats($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    stats
  }
}";

#[derive(Debug, Deserialize)]
struct AcceptanceRateWrapper {
    data: QuestionWrapper
}

#[derive(Debug, Deserialize)]
struct QuestionWrapper {
    question: QuestionStatsWrapper
}

#[derive(Debug, Deserialize)]
struct QuestionStatsWrapper {
    stats: String
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct QuestionStats {
    #[serde(rename = "totalAccepted")]
    _totalAccepted: String,
    #[serde(rename = "totalSubmission")]
    _totalSubmission: String,
    #[serde(rename = "totalAcceptedRaw")]
    _totalAcceptedRaw: i32,
    #[serde(rename = "totalSubmissionRaw")]
    _totalSubmissionRaw: i32,
    acRate: String,
}

pub async fn get_acceptance_rate(client: &Client, slug: &String) -> Result<String, Error> {
    let res: AcceptanceRateWrapper = client.post("https://leetcode.com/graphql")
        .json(&json!({
            "query": ACCEPTANCE_RATE_QUERY,
            "variables": {
                "titleSlug": slug
            }
        }))
        .send()
        .await?
        .json()
        .await?;
    
    /*
     * I don't know why the hell this is a thing, but LeetCode returns
     * a JSON as a raw string when you query for stats
     */
    let stats: QuestionStats = serde_json::from_str(res.data.question.stats.as_str()).unwrap();

    Ok(stats.acRate)
}

