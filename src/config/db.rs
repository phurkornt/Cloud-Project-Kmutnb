
use mysql::*;
use mysql::prelude::*;
    

pub fn conDB() -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    let url = "mysql://root:@localhost:3306/lottery";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    Ok(conn)
}

