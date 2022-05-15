#[macro_use]
extern crate mybatis;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate async_trait;

mod config;
mod domain;
mod dao;
mod service;
mod api;
mod common;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4+4, 8);
    }
}
