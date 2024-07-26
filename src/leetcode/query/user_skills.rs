use serde::Deserialize;
use serde_json::json;

use super::error::QueryError;

const GET_USER_SKILLS_QUERY: &str = "\
query userPublicProfile($username: String!) {
    matchedUser(username: $username) {
        profile {
            skillTags
        }
    }
}";

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UserSkillsWrapper {
    data: MatchedUserWrapper
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct MatchedUserWrapper {
    matchedUser: ProfileWrapper
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ProfileWrapper {
    profile: SkillTagsWrapper
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SkillTagsWrapper {
    skillTags: Vec<String>
}

pub async fn get_user_skills(client: &reqwest::Client, username: &String) -> Result<Vec<String>, QueryError> {
    let result = client.post("https://leetcode.com/graphql")
        .json(&json!({
            "query": GET_USER_SKILLS_QUERY,
            "variables": { "username": username }
        })).send().await;
    let response = match result {
        Ok(response) => response,
        Err(why) => return Err(QueryError::Reqwest(why))
    };

    let response: Result<UserSkillsWrapper, reqwest::Error> = response.json().await;
    let user_skills_wrapper = match response {
        Ok(user_skills_wrapper) => user_skills_wrapper,
        Err(why) => return Err(QueryError::Reqwest(why))
    };
    
    Ok(user_skills_wrapper.data.matchedUser.profile.skillTags)
}