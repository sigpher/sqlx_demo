use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect("test.db")
        .await?;



    let sql = "INSERT INTO COMPANY VALUES (7, 'James', 'Houston', 10000.00 )";
    let ret = sqlx::query(sql).execute(&pool).await?;
    println!("{:?}", ret);


    let emploees: Vec<(u32, String, String, f32)> = sqlx::query_as("SELECT * FROM COMPANY")
        .fetch_all(&pool)
        .await?;

    for emp in emploees {
        println!("{}=>{}", emp.0, emp.1)
    }

    // let sql = "DELETE FROM COMPANY WHERE id = ?";
    // let ret = sqlx::query(sql).bind(7).execute(&pool).await?;
    // println!("{:?}", ret);

    // let sql = "UPDATE COMPANY SET NAME = ? WHERE NAME = ?";
    // let ret = sqlx::query(sql)
    //     .bind("Choi")
    //     .bind("Kim")
    //     .execute(&pool)
    //     .await?;
    // println!("{}", ret.last_insert_rowid());

    Ok(())
}
