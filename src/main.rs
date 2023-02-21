fn main() {
    println!("Hello, world!");
}

struct MyStruct {
    pub url: String,
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    async fn server() -> mockito::Server {
        mockito::Server::new_with_port_async(0).await
    }

    #[fixture]
    async fn my_struct(#[future] server: mockito::Server) -> MyStruct {
        return MyStruct {
            url: server.await.url(),
        };
    }

    #[rstest]
    #[tokio::test]
    async fn test_it(#[future] server: mockito::Server, #[future] my_struct: MyStruct) {
        let mut server = server.await;
        let my_struct = my_struct.await;

        assert_eq!(server.url(), my_struct.url);

        let _m = server
            .mock("GET", "/foo")
            .with_status(200)
            .with_body("foo bar stroke:#000 other:#000")
            .create_async()
            .await;

        let resp = reqwest::get(format!("{}/foo", my_struct.url))
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
    }
}
