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
    async fn server() -> mockito::ServerGuard {
        mockito::Server::new_async().await
    }

    #[fixture]
    async fn server_url(#[future] server: mockito::ServerGuard) -> String {
        server.await.url()
    }

    #[fixture]
    async fn my_struct(#[future] server_url: String) -> MyStruct {
        return MyStruct {
            url: server_url.await,
        };
    }

    #[rstest]
    #[tokio::test]
    async fn test_it(#[future] server: mockito::ServerGuard, #[future] my_struct: MyStruct) {
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
