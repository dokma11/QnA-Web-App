use std::collections::HashMap;
use handle_errors::Error;
use warp::{http::StatusCode, path::param};
use tracing::{instrument, info, event, Level};

use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{Question, QuestionId, NewQuestion};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
        info!(pagination = false);
        let converted_pagination_limit: Option<i32> = pagination.limit.map(|limit| limit as i32);
        let res: Vec<Question> = match store
            .get_questions(converted_pagination_limit, pagination.offset as i32)
            .await {
            Ok(res) => res,
            Err(_) => {
                return Err(warp::reject::custom(
                    Error::DatabaseQueryError
                ))
            },
        };

    Ok(warp::reply::json(&res))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(_) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(_) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    }

    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(_) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
