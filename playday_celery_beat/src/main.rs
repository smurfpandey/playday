use anyhow::Result;
use celery::beat::CronSchedule;
use celery::broker::AMQPBroker;
use dotenv::dotenv;

use playday::tasks::add;

const QUEUE_NAME: &str = "playday_celery";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // Build a `Beat` with a default scheduler backend.
    let mut beat = celery::beat!(
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap() },
        tasks = [
            "long_running" => {
                add,
                schedule = CronSchedule::from_string("*/2 * * * *")?,
                args = (2,3),
            }
        ],
        task_routes = [
            "*" => QUEUE_NAME,
        ],
    ).await?;

    beat.start().await?;

    Ok(())
}
