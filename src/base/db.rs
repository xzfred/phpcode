use mysql as my;
use mysql::prelude::FromValue;
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
        {
            let _q = conn.query(r#" delete from usr_info where email='my@phpcode.com' "#).unwrap();
        }
        {
            let _q = conn.query(r#"
            insert into usr_info 
                (uname, email, pass, salt) 
            values 
                ('phpcode智', 'my@phpcode.com', 'phpcode', 'xxx---uuu')
        "#).unwrap();
        }

        let q = conn.query(r#"select uname,email from usr_info"#).unwrap();

        dbg!(q.info());
        dbg!(q.column_indexes());
        // dbg!(q);
        for row in q {
            let columns = row.unwrap().unwrap();
            for val in columns {
                dbg!(String::from_value(val));
            }
        }
    }
}

