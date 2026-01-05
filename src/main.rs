use rand::Rng;

use restate_sdk::prelude::*;

#[restate_sdk::object]
trait LargeState {
    async fn state(size: u64) -> Result<String, HandlerError>;
}

struct LargeStateImpl;

impl LargeState for LargeStateImpl {
    async fn state(&self, mut ctx: ObjectContext<'_>, size: u64) -> Result<String, HandlerError> {
        let mut data = Vec::with_capacity(size as usize);

        for _ in 0..size {
            data.push(ctx.rand().random())
        }

        ctx.set("foobar", data);
        // Respond to caller
        Ok("State set".to_owned())
    }
}

#[tokio::main]
async fn main() {
    // To enable logging
    tracing_subscriber::fmt::init();

    HttpServer::new(Endpoint::builder().bind(LargeStateImpl.serve()).build())
        .listen_and_serve("0.0.0.0:9080".parse().unwrap())
        .await;
}
