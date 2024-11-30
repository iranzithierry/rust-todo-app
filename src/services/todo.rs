use uuid::Uuid as UuidGen;
use actix_web::{web, HttpResponse};
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::entities::todo::{self, Entity as Todo};

pub async fn get_todos(db: web::Data<DatabaseConnection>) -> HttpResponse {
    let todos = Todo::find().all(db.get_ref()).await;
    match todos {
        Ok(todos) => HttpResponse::Ok()
            .json(serde_json::json!({ "data": todos, "message": "Todos retrieved successfully" })),
        Err(err) => HttpResponse::InternalServerError().json(
            serde_json::json!({ "error": "Internal Server Error", "message": err.to_string() }),
        ),
    }
}

pub async fn get_todo(path: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> HttpResponse {
    let todo = Todo::find_by_id(path.into_inner()).one(db.get_ref()).await;
    match todo {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound()
            .json(serde_json::json!({ "error": "Not Found", "message": "Todo not found" })),
        Err(err) => HttpResponse::InternalServerError().json(
            serde_json::json!({ "error": "Internal Server Error", "message": err.to_string() }),
        ),
    }
}

pub async fn create_todo(
    todo: web::Json<todo::CreateTodo>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let new_todo = todo::ActiveModel {
        id: Set(UuidGen::new_v4()),
        title: Set(todo.title.clone()),
        completed: Set(false),
        ..Default::default()
    };

    match new_todo.insert(db.get_ref()).await {
        Ok(res) => HttpResponse::Created()
            .json(serde_json::json!({ "data": res, "message": "Todo created successfully" })),
        Err(err) => HttpResponse::InternalServerError().json(
            serde_json::json!({ "error": "Internal Server Error", "message": err.to_string() }),
        ),
    }
}

pub async fn update_todo(
    path: web::Path<Uuid>,
    todo: web::Json<todo::UpdateTodo>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let todo_id = path.into_inner();
    let todo_res = Todo::find_by_id(todo_id).one(db.get_ref()).await;

    match todo_res {
        Ok(Some(t)) => {
            let mut todo_model: todo::ActiveModel = t.into();
            if let Some(title) = &todo.title {
                todo_model.title = Set(title.clone());
            }
            if let Some(completed) = todo.completed {
                todo_model.completed = Set(completed);
            }

            match todo_model.update(db.get_ref()).await {
                Ok(updated_todo) => HttpResponse::Ok().json(updated_todo),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Ok(None) => HttpResponse::NotFound()
            .json(serde_json::json!({ "error": "Not Found", "message": "Todo not found" })),
        Err(err) => HttpResponse::InternalServerError().json(
            serde_json::json!({ "error": "Internal Server Error", "message": err.to_string() }),
        ),
    }
}

pub async fn delete_todo(path: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> HttpResponse {
    match Todo::delete_by_id(path.into_inner())
        .exec(db.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent()
            .json(serde_json::json!({ "message": "Todo deleted successfully" })),
        Err(_) => HttpResponse::NotFound()
            .json(serde_json::json!({ "error": "Not Found", "message": "Todo not found" })),
    }
}
