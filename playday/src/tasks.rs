use celery::task::TaskResult;
use chrono::{Duration, TimeZone, Utc};

use crate::db;

#[celery::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    println!("Aala re aala!");
    Ok(x + y)
}

#[celery::task]
pub fn whats_for_tomorrow() -> TaskResult<bool> {
    let db_conn = db::establish_connection();

    // Get all games in wishlist
    let results = db::get_future_wishlist_games(&db_conn).unwrap(); // TODO: Handle error in better way

    let now_dt = Utc::now();
    let release_dt_threshold = now_dt + Duration::days(3); // 3 days from now

    let release_dt_epoch = release_dt_threshold.timestamp();

    // For each game
    for game in results.iter() {
        let release_dt = Utc.timestamp(game.pc_release_date, 0);

        let duration = release_dt.signed_duration_since(now_dt);

        println!("{} releasing in {} days", game.title, duration.num_days())
        // if game.pc_release_date < release_dt_epoch {
        // }
    }
    // is it releasing tomorrow?
    // Send email

    // Get latest info from igdb incase release date is updated
    // update entry in db

    Ok(true)
}
