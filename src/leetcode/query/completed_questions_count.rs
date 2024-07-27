use serde::Deserialize;
use serde_json::json;

use super::error::QueryError;

const COMPLETED_QUESTIONS_COUNT_QUERY: &str = "\
query userProfileUserQuestionProgressV2($userSlug: String!) {
    userProfileUserQuestionProgressV2(userSlug: $userSlug) {
        numAcceptedQuestions {
            count
            difficulty
        }
    }
}";

pub struct CompletedQuestionsCount {
    pub easy: i32,
    pub medium: i32,
    pub hard: i32
}

#[derive(Deserialize)]
struct CompletedQuestionsCountWrapper {
    data: UserProfileUserQuestionProgressV2Wrapper
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UserProfileUserQuestionProgressV2Wrapper {
    userProfileUserQuestionProgressV2: NumAcceptedQuestionsWrapper
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct NumAcceptedQuestionsWrapper {
    numAcceptedQuestions: Vec<AcceptedQuestionCount>
}

#[derive(Deserialize)]
struct AcceptedQuestionCount {
    count: i32,
    difficulty: String
}

impl CompletedQuestionsCount {
    fn new() -> Self {
        Self { easy: 0, medium: 0, hard: 0 }
    }

    pub fn score(&self) -> i32 {
        self.easy + 3 * self.medium + 5 * self.hard
    }
}

impl From<CompletedQuestionsCountWrapper> for CompletedQuestionsCount {
    fn from(wrapper: CompletedQuestionsCountWrapper) -> Self {
        let mut count = Self::new();

        for accepted_questions in wrapper.extract().iter() {
            match accepted_questions.difficulty.as_str() {
                "EASY" => count.easy = accepted_questions.count,
                "MEDIUM" => count.medium = accepted_questions.count,
                "HARD" => count.hard = accepted_questions.count,
                _ => ()
            }
        }

        count        
    }
}

impl CompletedQuestionsCountWrapper {
    fn extract(&self) -> &Vec<AcceptedQuestionCount> {
        self.data.userProfileUserQuestionProgressV2.numAcceptedQuestions.as_ref()
    }
}

pub async fn get_completed_questions_count(client: &reqwest::Client, username: String) -> Result<CompletedQuestionsCount, QueryError> {
    let response_result = client.post("https://leetcode.com/graphql")
        .json(&json!({
            "query": COMPLETED_QUESTIONS_COUNT_QUERY,
            "variables": {
                "userSlug": username
            }
        })).send().await;

    let response = match response_result {
        Ok(response) => response,
        Err(why) => return Err(QueryError::Reqwest(why))
    };

    let wrapper: CompletedQuestionsCountWrapper = match response.json().await {
        Ok(count) => count,
        Err(why) => return Err(QueryError::Reqwest(why))
    };

    Ok(CompletedQuestionsCount::from(wrapper))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_completed_questions_count_init() {
        let (easy_count, medium_count, hard_count) = (196, 167, 25);

        let wrapper = CompletedQuestionsCountWrapper {
            data: UserProfileUserQuestionProgressV2Wrapper {
                userProfileUserQuestionProgressV2: NumAcceptedQuestionsWrapper {
                    numAcceptedQuestions: vec![
                        AcceptedQuestionCount {
                            count: easy_count,
                            difficulty: "EASY".to_string()
                        },
                        AcceptedQuestionCount {
                            count: medium_count,
                            difficulty: "MEDIUM".to_string()
                        },
                        AcceptedQuestionCount {
                            count: hard_count,
                            difficulty: "HARD".to_string()
                        },
                    ]
                }
            }
        };

        let completed_questions_count = CompletedQuestionsCount::from(wrapper);
        assert_eq!(completed_questions_count.easy, easy_count);
        assert_eq!(completed_questions_count.medium, medium_count);
        assert_eq!(completed_questions_count.hard, hard_count);
    }

    #[test]
    fn test_completed_questions_score() {
        let completed_questions_count = CompletedQuestionsCount {
            easy: 196,
            medium: 167,
            hard: 25
        };

        let score = completed_questions_count.score();
        assert_eq!(score, 822);
    }
}