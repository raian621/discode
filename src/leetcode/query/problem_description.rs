use std::collections::HashMap;

use html2md::TagHandlerFactory;
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

use crate::leetcode::{img_tag_handler::ImgTagHandlerFactory, models::ProblemDescription, query::get_acceptance_rate};

const PROBLEM_INFO_QUERY: &str = "query questionTitle($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        questionFrontendId
        title
        difficulty
    }
}";

const PROBLEM_DESCRIPTION_QUERY: &str = "query questionContent($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
    }
}";

#[derive(Debug, Deserialize)]
struct ProblemInfoResponse {
    data: QuestionInfoResponse
}

#[derive(Debug, Deserialize)]
struct QuestionInfoResponse {
    question: QuestionInfo
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct QuestionInfo {
    questionFrontendId: String,
    title: String,
    difficulty: String,
}

#[derive(Debug, Deserialize)]
struct ProblemDescriptionResponse {
    data: QuestionDescriptionResponse
}

#[derive(Debug, Deserialize)]
struct QuestionDescriptionResponse {
    question: QuestionDescription
}

#[derive(Debug, Deserialize)]
struct QuestionDescription {
    content: String
}

pub async fn get_problem_description(slug: String) -> Result<ProblemDescription, Error> {
    tracing::info!("getting problem description for slug {slug}");

    let client = reqwest::Client::new();
    let res: ProblemInfoResponse = client.post("https://leetcode.com/graphql")
        .json(&json!({
            "query": PROBLEM_INFO_QUERY,
            "variables": {
                "titleSlug": slug
            }
        })).send().await?.json().await?;
    let question_info = res.data.question;

    let res: ProblemDescriptionResponse = client.post("https://leetcode.com/graphql")
        .json(&json!({
            "query": PROBLEM_DESCRIPTION_QUERY,
            "variables": {
                "titleSlug": slug
            }
        })).send().await?.json().await?;
    let description_html = res.data.question.content;
    let description_md = html2md::parse_html_custom(
        description_html.as_ref(),
        &HashMap::from([
            ("img".to_string(), Box::new(ImgTagHandlerFactory{}) as Box<dyn TagHandlerFactory>)
        ])
    );

    let acceptance_rate = get_acceptance_rate(client, &slug).await?;

    Ok(ProblemDescription {
        title: question_info.title,
        link: format!("https://leetcode.com/problems/{}", slug),
        difficulty: question_info.difficulty,
        description: description_md,
        question_id: question_info.questionFrontendId,
        acceptance_rate,
    })
}