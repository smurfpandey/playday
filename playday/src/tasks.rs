use celery::task::TaskResult;

#[celery::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    println!("Aala re aala!");
    Ok(x + y)
}
