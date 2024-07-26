use serenity::all::{Color, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

use super::models::ProblemDescription;

pub fn problem_view(title: Option<&str>, question_info: Option<ProblemDescription>) -> CreateInteractionResponse {
    let data = match question_info {
        Some(info) => {
            let mut problem_embed = CreateEmbed::new()
                .description(problem_description_markdown(info))
                .color(Color::new(0xffa116));
        
            if let Some(title) = title {
                problem_embed = problem_embed.title(title);
            }
        
            CreateInteractionResponseMessage::new().add_embed(problem_embed)
        },
        None => {
            CreateInteractionResponseMessage::new().content("Problem not found").ephemeral(true)
        }
    };
    
    CreateInteractionResponse::Message(data)
}

fn problem_description_markdown(info: ProblemDescription) -> String {
    format!(
        "# [{}. {}]({})\n**Difficulty**: {}\n**Acceptance Rate**: {}\n\n{}",
        info.question_id,
        info.title,
        info.link,
        info.difficulty,
        info.acceptance_rate,
        info.description
    )
}

#[cfg(test)]
mod tests {
    use crate::leetcode::models::ProblemDescription;

    use super::problem_description_markdown;

    #[test]
    fn test_problem_description_md() {
        let info = ProblemDescription {
            question_id: "55".to_string(),
            title: "Jump Game".to_string(),
            link: "https://leetcode.com/problems/jump-game".to_string(),
            acceptance_rate: "55%".to_string(),
            difficulty: "Easy".to_string(),
            description: "# Jump Game description\nThis is the jump game description".to_string()
        };

        let markdown = problem_description_markdown(info);
        assert_eq!(markdown,
r#"# [55. Jump Game](https://leetcode.com/problems/jump-game)
**Difficulty**: Easy
**Acceptance Rate**: 55%

# Jump Game description
This is the jump game description"#.to_string()
        )
    }
}
