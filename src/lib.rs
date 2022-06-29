pub mod agent;
pub mod board;
pub mod cell;
pub mod factory;
pub mod params;
pub mod path;
pub mod terrain;
pub mod tool;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
