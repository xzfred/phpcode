use mysql as my;
use base::config::Database;

pub struct MyPool(my::Pool);

impl MyPool {
    pub fn new(config: &Database) -> MyPool {
        let mut builder = my::OptsBuilder::default();
        builder.user(Some(config.user.clone()))
            .pass(Some(config.password.clone()))
            .ip_or_hostname(Some(config.host.clone()))
            .tcp_port(config.port.unwrap())
            .db_name(Some(config.db.clone()));
        let pool = my::Pool::new(builder).unwrap();
        MyPool(pool)
    }

    pub fn value(&self) -> my::Pool {
        self.0.clone()
    }

    pub fn new_user(&self) {
        let mut conn = self.0.get_conn().unwrap();
        let q = conn.query(r#"
            insert into usr_info 
                (uname, email, pass, salt) 
            values 
                ('phpcode', 'my@phpcode.com', 'phpcode', 'xxx---uuu')
        "#).unwrap();
    }
}

