use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

const PROBLEM_SLUG_BY_ID_QUERY: &str = "query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
    problemsetQuestionList: questionList(
        categorySlug: $categorySlug
        limit: $limit
        skip: $skip
        filters: $filters
    ) {
        questions: data {
            titleSlug
        }
    }
}";

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ProblemSlugByIdResponseWrapper {
    data: ProblemsetQuestionListWrapper,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ProblemsetQuestionListWrapper {
    problemsetQuestionList: ProblemsetQuestionWrapper,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ProblemsetQuestionWrapper {
    questions: Vec<ProblemsetQuestion>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ProblemsetQuestion {
    titleSlug: String,
}

pub async fn get_problem_slug_by_id(id: i32) -> Result<String, Error> {
    tracing::info!("searching for problem by id `{id}`");

    let client = reqwest::Client::new();
    let res: ProblemSlugByIdResponseWrapper = client.post("https://leetcode.com/graphql")
        .json(&json!({
            "query": PROBLEM_SLUG_BY_ID_QUERY, 
            "variables": {
                "categorySlug": "all-code-essentials",
                "filters": {},
                "limit": 1,
                "skip": id
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res.data.problemsetQuestionList.questions[0].titleSlug.clone())
}
