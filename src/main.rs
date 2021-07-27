use sqlx::prelude::*;
use sqlx::Pool;
use sqlx::postgres::{PgConnection,PgCursor};
use std::env;


#[async_std::main]
async fn main() -> anyhow::Result<()> {

    // 单个连接
    let mut pool = Pool::<PgConnection>::new(&env::var("DATABASE_URL")?).await?;

    // 可以对连接池配置一些参数
    /*
    let pool = Pool::<PgConnection>::builder()
    .max_size(5)
    .build(&env::var("DATABASE_URL")?).await?;
    */

    /*
    let sql="INSERT INTO student (id,name,age,hobby)VALUES ( $1,$2,$3,$4 )";
    let count=sqlx::query(sql).bind(4).bind("熊大").bind(7).bind("跑步").execute(&pool).await?;
    println!("{}",count);
    */

    /*
    let sql=  "DELETE FROM student WHERE  hobby = '踢足球'";
    let count=sqlx::query(sql).bind(18).execute(&pool).await?;
    println!("{}",count);
    */

    /*
    let sql=  "UPDATE student SET hobby = '打羽毛球' WHERE id = 1 ";
    let count=sqlx::query(sql).bind(1).execute(&pool).await?;
    println!("{}",count);
    */

    let sql = "SELECT * FROM student ORDER BY id";
    let mut cursor:PgCursor = sqlx::query(sql).fetch(&pool);
    while let Some(row) = cursor.next().await? {

        let id: i32 = row.get(0);
        let name: String = row.get("name");
        let age: i32 = row.get(2);
        let hobby: String = row.get(3);
        println!("{}, {}，{}, {}",id,name,age,hobby);
    }
    Ok(())
}

