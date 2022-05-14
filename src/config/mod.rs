pub mod database;
pub mod mybatis_config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conn() {
        database::init();
    }
}