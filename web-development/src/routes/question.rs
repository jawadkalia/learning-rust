use std::collections::HashMap;

use warp::hyper::StatusCode;

use crate::store::Store;
use crate::types::pagination::{self, extract_pagination};
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        match res.get(pagination.start..pagination.end) {
            Some(value) => Ok(warp::reply::json(&value)),
            None => Err(warp::reject::custom(Error::QuestionNotFound)),
        }
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&question.id.clone()) {
        Some(q) => {
            let updated_id = q.id.0.clone();
            *q = question;
            Ok(warp::reply::with_status(
                format!("Question id {} updated", { updated_id }),
                StatusCode::OK,
            ))
        }
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .questions
        .write()
        .await
        .remove(&QuestionId(id.clone()))
    {
        Some(_) => Ok(warp::reply::with_status(
            format!("Question with id {} removed", id),
            StatusCode::OK,
        )),
        None => todo!(),
    }
}
