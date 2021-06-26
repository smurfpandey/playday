use anyhow::Result;
use celery::broker::AMQPBroker;
use dotenv::dotenv;

use playday::tasks::add;

const QUEUE_NAME: &str = "playday_celery";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let my_app = celery::app!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [add],
        task_routes = [
            "*" => QUEUE_NAME,
        ],
    ).await?;

    my_app.display_pretty().await;
    my_app.consume_from(&[QUEUE_NAME]).await?;

    Ok(())
}
