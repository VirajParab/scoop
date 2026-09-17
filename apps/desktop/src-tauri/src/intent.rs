use crate::classify::ContentType;

pub fn rank_actions(content_type: ContentType) -> Vec<String> {
    let mut actions = match content_type {
        ContentType::Math => vec![
            "calculate".into(),
            "ask_ai".into(),
            "search".into(),
            "save_library".into(),
            "save_note".into(),
            "copy".into(),
        ],
        ContentType::Code => vec![
            "ask_ai".into(),
            "search".into(),
            "save_library".into(),
            "save_note".into(),
            "copy".into(),
        ],
        ContentType::Table => vec![
            "ask_ai".into(),
            "calculate".into(),
            "search".into(),
            "save_library".into(),
            "copy".into(),
        ],
        ContentType::UiScreenshot => vec![
            "ask_ai".into(),
            "save_library".into(),
            "save_note".into(),
            "copy".into(),
            "search".into(),
        ],
        _ => vec![
            "ask_ai".into(),
            "search".into(),
            "save_library".into(),
            "save_note".into(),
            "copy".into(),
        ],
    };

    // Ensure universal actions present
    for a in ["ask_ai", "search", "copy", "save_library", "save_note"] {
        if !actions.iter().any(|x| x == a) {
            actions.push(a.into());
        }
    }
    actions
}
