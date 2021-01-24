use comrak::{markdown_to_html, ComrakOptions};
use dotenv::dotenv;
use http::StatusCode;
use serde::de::DeserializeOwned;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::convert::Infallible;
use std::env;
use validator::Validate;
use warp::{Filter, Rejection, Reply};
use urlencoding::decode;

use ty_lib::{ThankYouMessage, ThankYouStats, ThankYouDetail};

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

    setup_database(db_pool.clone())
        .await
        .expect("seting up database failed");

    let spa = warp::path("spa")
        .and(warp::fs::dir("/home/pawe/Projects/ty-thank-you/ty-spa/static"));

    let index = warp::path::end().map(|| {
        warp::reply::html(markdown_to_html(
            include_str!("../../README.md"),
            &ComrakOptions::default(),
        ))
    });

    let ty = warp::path("v0").and(warp::path("note")).and(
        warp::post()
            .and(warp::body::content_length_limit(4096))
            .and(with_db(db_pool.clone()))
            .and(validated_from_json())
            .and_then(handle_post_ty_note)
            .recover(handle_rejection),
    );

    let info = warp::path("v0").and(
        warp::get()
            .and(warp::path::end()
                .and(with_db(db_pool.clone()))
                .and_then(handle_info)
        )
    );

    let count = warp::path!("v0" / "tool" / String)
        .and(with_db(db_pool.clone()))
        .and_then(handle_count);

    let detail = warp::path!("v0" / "tool" / String / "detail")
        .and(with_db(db_pool.clone()))
        .and_then(handle_detail);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8901".to_string())
        .parse()
        .expect("coudln't parse PORT into u16");

    warp::serve( warp::any()
            .and(
                index
                    .or(spa)
                    .or(ty)
                    .or(info)
                    .or(count)
                    .or(detail)
            )
                .with(log)
        )
        .run(([0, 0, 0, 0], port))
        .await;

    Ok(())
}

fn with_db(
    db_pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn validated_from_json<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy
where
    T: DeserializeOwned + Validate + Send,
{
    warp::body::json().and_then(|json: T| async move {
        let validation_result = json.validate();
        if validation_result.is_err() {
            Err(warp::reject::custom(TYValidationError::new(
                validation_result.err().unwrap(),
            )))
        } else {
            Ok(json)
        }
    })
}

#[derive(Debug)]
struct TYValidationError {
    errors: validator::ValidationErrors,
}

impl TYValidationError {
    fn new(errors: validator::ValidationErrors) -> Self {
        TYValidationError { errors }
    }
}

impl warp::reject::Reject for TYValidationError {}


#[derive(Debug)]
struct TYDatabaseError {}

impl warp::reject::Reject for TYDatabaseError {}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = err.find::<TYValidationError>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&e.errors),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Err(err)
    }
}

async fn handle_post_ty_note(
    pool: Pool<Postgres>,
    ty_message: ThankYouMessage,
) -> Result<impl Reply, Rejection> {
    match sqlx::query!(
        r#"
            INSERT INTO ty (program, note)
            VALUES ($1, $2)
        "#,
        ty_message.program,
        ty_message.note
    )
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&""),
            StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&""),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

async fn handle_count(program: String, pool: Pool<Postgres>) -> Result<impl Reply, Rejection> {
    let program = match decode(&program) {
        Ok(p) => p,
        Err(err) => {
            dbg!(err);
            return Err(warp::reject::reject());
        }
    };
    
    let res = sqlx::query!(
        r#"SELECT COUNT(*) as "count!" FROM ty WHERE program = $1"#,
        program
    )
    .fetch_one(&pool)
    .await;

    if let Ok(rec) = res {
        Ok(rec.count.to_string())
    } else {
        // that is not a proper way to handle errors!
        Ok(0.to_string())
    }
}

async fn handle_info(pool: Pool<Postgres>) -> Result<impl Reply, Rejection> {
    let res = sqlx::query_as!(ThankYouStats,
        r#"
            select 
                ty."program", 
                count(*) as "count!", 
                count(ty.note) as "note_count!"
            from public.ty 
            group by ty."program"
            order by "count!" desc
            limit 200;
        "#
    ).fetch_all(&pool)
    .await;

    if let Ok(stats) = res {
        Ok(warp::reply::with_status(warp::reply::json(&stats), StatusCode::OK))
    } else {
        Err(warp::reject::custom(TYDatabaseError {}))
    }
    
}

async fn handle_detail(program: String, pool: Pool<Postgres>) -> Result<impl Reply, Rejection> {
    let program = match decode(&program) {
        Ok(p) => p,
        Err(err) => {
            dbg!(err);
            return Err(warp::reject::reject());
        }
    };

    let res = sqlx::query!(
        r#"
            select ty.note as "note!"
            from public.ty 
            where ty.note is not null
                and ty.program = $1
            limit 200;
        "#,
        program
    ).fetch_all(&pool)
    .await;

    if let Ok(records) = res {
        let notes = records.iter().map(|row| row.note.to_string()).collect();

        let detail = ThankYouDetail {
            program,
            notes,
        };

        Ok(warp::reply::with_status(warp::reply::json(&detail), StatusCode::OK))
    } else {
        Err(warp::reject::reject())
        // Ok(warp::reply::with_status(warp::reply::json(&""), StatusCode::INTERNAL_SERVER_ERROR))
    }
    
}

async fn setup_database(pool: Pool<Postgres>) -> anyhow::Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS ty (
            id BIGSERIAL PRIMARY KEY,
            program VARCHAR(50) NOT NULL,
            note VARCHAR(2048),
            created TIMESTAMP DEFAULT now()
        );
    "#
    )
    .execute(&pool)
    .await?;

    Ok(())
}
