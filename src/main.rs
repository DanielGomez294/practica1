use axum::{
    extract::{Path, Query},
    http::{
        header::{ACCEPT, CONTENT_TYPE, ORIGIN},
        Method, StatusCode,
    },
    routing::{get, post},
    Json, Router,
};

use tower_http::cors::CorsLayer;

use serde::{Deserialize, Serialize};
mod database;

use database::connection::DB;
use sqlx::types::Uuid;

//struct select

struct SelectTshirt {
    id: Uuid,
    tshirt: String,
    price: i64,
}
#[derive(Serialize)]
struct Response {
    status: String,
    data: Vec<SelectToString>,
    desc: String,
}

#[derive(Serialize)]
struct SelectToString {
    Uuid: String,
    tshirt: String,
    price: i64,
} //end struct select

//struc delete
#[derive(Deserialize, Serialize)]
struct UuidLibro {
    id: String,
}

#[derive(Serialize)]

struct UpResponse {
    status: String,
    rows_affected: bool,
    description: String,
}// end struct delete

//struc update

#[derive(Deserialize, Serialize)]
struct updateLibros{
    id: String,
    tshirt: String,
    price: i64
}

#[derive(Serialize)]
struct UpdateResponse{
    status: String,
    rows_affected: bool,
    description: String
}//end struct update

//struc for id
#[derive(Serialize)]
struct selectuuid{
    status: String,
    data: updateLibros,
    desc: String,
}
#[tokio::main]

async fn main() {
    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://localhost:3000/".parse().unwrap(),
    ];

    let app = Router::new()
        .route("/select", get(select))
        .route("/delete", post(eliminar))
        .route("/update", post(actualizar))
        .route("/select/one", post(SelectId))
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([ORIGIN, ACCEPT, CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );

    axum::Server::bind(&"0.0.0.0:9000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn select() -> Json<Response> {
    let pool = DB::connection().await;

    let sql = sqlx::query_as!(
            SelectTshirt,
        "
        SELECT * from styled
        "
    )
    .fetch_all(&pool)
    .await;

    let response = match sql {
        Ok(data) => Response {
            status: "200 OK".to_string(),
            data: data
                .into_iter()
                .map(|x| SelectToString {
                    Uuid: x.id.to_string(),
                    tshirt: x.tshirt,
                    price: x.price,
                })
                .collect(),
                desc: "success".to_string(),
          
        },
        Err(_err) => Response {
            status: "500 Internal Server Error".to_string(),
            data: vec![],
            desc: "error".to_string(),
            
        },
    };

    Json(response)
}

async fn selectt() -> Json<Response> {
    let db = DB::connection().await;

    let sql = sqlx::query_as!(
        SelectTshirt,
        "
         SELECT * FROM styled 
         "
    )
    .fetch_all(&db)
    .await;

    let response = match sql {
        Ok(data) => Response {
            status: "200 OK".to_string(),
            data: data
                .into_iter()
                .map(|x| SelectToString {
                    Uuid: x.id.to_string(),
                    tshirt: x.tshirt,
                    price: x.price,
                })
                .collect(),
            desc: "All data".to_string(),
        },
        Err(_err) => Response {
            status: StatusCode::CONFLICT.to_string(),
            data: vec![],
            desc: "No data".to_string(),
        },
    };

    Json(response)
}

async fn eliminar(Json(payload): Json<UuidLibro>) -> Json<UpResponse> {
    let db = DB::connection().await;
    let uuid = Uuid::parse_str(&payload.id).expect("error al transformar uuid");

    let sql = sqlx::query!(
        "
        DELETE FROM styled
        WHERE id = $1",
        uuid
    )
    .execute(&db)
    .await
    .expect("ERROR AL ELIMINAR TSHIRT")
    .rows_affected();

    let response = if sql > 0 {
        UpResponse {
            status: "200 OK".to_string(),
            rows_affected: true,
            description: "Eliminado".to_string(),
        }
    } else {
        UpResponse {
            status:  StatusCode::CONFLICT.to_string(),
            rows_affected: false,
            description: "No data".to_string(),
        }
    };
    Json(response)
}


async fn actualizar(Json(payload): Json<updateLibros>) -> Json<UpdateResponse>{
    let db = DB::connection().await;

    let uuid = Uuid::parse_str(&payload.id).expect("error al transformar uuid");
    
    let sql = sqlx::query!(
            r#"
            UPDATE styled
            SET tshirt = $1, price = $2
            WHERE id = $3
            "#,
            payload.tshirt,
            payload.price,
            uuid
    )
    .execute(&db)
    .await
    .expect("error al transformar uuid")
    .rows_affected();

let response = if sql > 0{
    UpdateResponse {
        status: "200 OK".to_string(),
        rows_affected: true,
        description: "Actualizado".to_string(),
    }
}else{
    UpdateResponse {
        status:  StatusCode::CONFLICT.to_string(),
        rows_affected: false,
        description: "No data".to_string(),
    }
};

Json(response)
}


async fn SelectId(Json(payload): Json<UuidLibro>) -> Json<selectuuid> {
    let db = DB::connection().await;

    let uuid = Uuid::parse_str(&payload.id).expect("error al transformar uuid");


let sql = sqlx::query!(
        "
        SELECT * FROM styled
        WHERE id = $1",
        uuid
    )
    .fetch_all(&db)
    .await
    .expect("error al transformar uuid");


    let response = selectuuid{
        status: "200 OK".to_string(),
        data: updateLibros{

            id: sql[0].id.to_string(),
            tshirt: sql[0].tshirt.to_string(),
            price: sql[0].price,
        },
        desc: "All data".to_string(),
    };

    Json(response)
}