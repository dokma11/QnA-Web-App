use sqlx::{Row};
use sqlx::postgres::{PgPoolOptions, PgRow, PgPool};
use crate::types::question::{NewQuestion, Question, QuestionId};
use crate::types::answer::{NewAnswer, Answer, AnswerId};
use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e)
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32
    ) -> Result<Vec<Question>, Error> { // moram samo proveriti da li mora da bude Sqlx error ili moze ovako
        match sqlx::query("SELECT * FROM QUESTION LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags")
            })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError)
                }
        }
    }

    pub async fn add_question(
        &self,
        new_question: NewQuestion
    ) -> Result<Question, Error> { // i ovde proveriti samo za povratnu vrednost greske da li valja
        match sqlx::query("INSERT INTO QUESTIONS (title, content, tags)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .map(|row: PgRow| Question{
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await {
            Ok(question) => Ok(question),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32
    ) -> Result<Question, Error> {
        match sqlx::query("UPDATE QUESTIONS
        SET title = $1, content = $2, tags = $3
        WHERE id = $4
        RETURNING id, title, content, tags")
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .bind(question_id)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await {
            Ok(question) => Ok(question),
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }

    pub async fn delete_question(
        &self,
        question_id: i32
    ) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM QUESTIONS
        WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }

    pub async fn add_answer(
        &self,
        new_answer: NewAnswer
    ) -> Result<Answer, Error> {
        match sqlx::query("INSERT INTO ANSWERS (content, question_id)
        VALUES ($1, $2)
        RETURNING id, content, question_id")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer{
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await {
            Ok(question) => Ok(question),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }
}
