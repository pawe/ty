use warp::{Filter, Reply, Rejection};
use std::convert::Infallible;
use http::StatusCode;
use dotenv::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use comrak::{markdown_to_html, ComrakOptions};

use ty_lib::ThankYouMessage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let log = warp::log::custom(|info| {
        if let Some(remote_addr) = info.remote_addr() {
            print!("{}: ", remote_addr);
        }

        println!(
            "{} {} {} ({}ms)",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed().as_millis(),
        );
    });

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL expected in environment"))
        .await
        .expect("Cannot connect to postgres");

    setup_database(db_pool.clone()).await?;

    let index = warp::path::end()
        .map(|| 
            warp::reply::html(markdown_to_html(include_str!("../../README.md"), &ComrakOptions::default())));

    let ty = warp::path("v0")
        .and(warp::path("note"))
        .and(warp::post()
            .and(warp::body::content_length_limit(4096))
            .and(with_db(db_pool.clone()))
            .and(warp::body::json())
            .and_then(handle_post_ty_note)
        );

    let count = warp::path!("v0" / "tool" / String)
        .and(with_db(db_pool.clone()))
        .and_then(handle_count);

    
    let port: u16 =  env::var("PORT")
        .unwrap_or("8901".to_string())
        .parse()
        .expect("coudln't parse PORT into u16");
    
    warp::serve(warp::any().and(index.or(ty).or(count)).with(log))
        .run(([0, 0, 0, 0], port))
        .await;

    Ok(())
}

fn with_db(db_pool: Pool<Postgres>) -> impl Filter<Extract = (Pool<Postgres>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

async fn handle_post_ty_note(pool: Pool<Postgres>, ty_message: ThankYouMessage) -> Result<impl Reply, Rejection> {
    use validator::Validate;
    let validation_result = ty_message.validate();
    if validation_result.is_err() {
        let json = warp::reply::json(&validation_result.err().unwrap());
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    match sqlx::query!(
        r#"
        INSERT INTO ty (program, note)
        VALUES ($1, $2)
        "#,
        ty_message.program,
        ty_message.note
    )
    .execute(&pool)
    .await {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(&""), StatusCode::CREATED)),
        Err(_) => Ok(warp::reply::with_status(warp::reply::json(&""), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn handle_count(tool: String, pool: Pool<Postgres>) -> Result<impl Reply, Rejection> {
    let res = sqlx::query!(r#"SELECT COUNT(*) as "count!" FROM ty WHERE program = $1"#, tool)
        .fetch_one(&pool)
        .await;

    if let Ok(rec) = res {
        Ok(rec.count.to_string())
    } else {
        Ok(0.to_string())
    }   
}


async fn setup_database(pool: Pool<Postgres>) -> anyhow::Result<(), sqlx::Error> {
    sqlx::query!(r#"
        CREATE TABLE IF NOT EXISTS ty (
            id BIGSERIAL PRIMARY KEY,
            program TEXT NOT NULL,
            note TEXT,
            created TIMESTAMP DEFAULT now()
        );
    "#)
    .execute(&pool)
    .await?;

    Ok(())
}