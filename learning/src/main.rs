use std::fmt::Display;

struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }

    fn update_title(&self, new_title: String) -> Self {
        Question::new(self.id, new_title, self.content, self.tags)
    }
}

fn main() {
    let question = Question::new(
        QuestionId::from("1"),
        "First Question".to_string(),
        "Content of question".to_string(),
        ["faq"],
    );
    println!("{}", question);
}
