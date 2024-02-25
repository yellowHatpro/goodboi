use std::env;
use std::error::Error;
use dotenvy::dotenv;
use i_remember_structs::RememberEntity;
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("Please insert database url in env file");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("{}",url);
    let rememberentity = RememberEntity {
        id: "123".to_string(),
        title: "This new".to_string(),
        cmds: vec![],
        description: "This is the new description".to_string(),
        pwd: "dsdasd".to_string(),
    };
    let query = "INSERT INTO remember_entities(id, title, cmds, description, pwd) VALUES ($1, $2, $3, $4, $5)";
    sqlx::query(query)
        .bind(&rememberentity.id)
        .bind(&rememberentity.title)
        .bind(&rememberentity.cmds)
        .bind(&rememberentity.description)
        .bind(&rememberentity.pwd)
        .execute(&pool)
        .await?;
    Ok(())
}