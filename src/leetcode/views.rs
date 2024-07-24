use serenity::all::{Color, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

use super::models::ProblemDescription;

pub fn problem_view(title: Option<&str>, question_info: Option<ProblemDescription>) -> CreateInteractionResponse{
    let data = match question_info {
        Some(info) => {
            let mut problem_embed = CreateEmbed::new()
                .description(format!(
                    "# [{}. {}]({})\n**Difficulty**: {}\n**Acceptance Rate**: {}\n\n{}",
                    info.question_id,
                    info.title,
                    info.link,
                    info.difficulty,
                    info.acceptance_rate,
                    info.description
                ))
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