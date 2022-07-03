#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub mod database;