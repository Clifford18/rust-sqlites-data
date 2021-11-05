use rusqlite::{Connection, ToSql};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod common;

static MIN_BATCH_SIZE: i64 = 50;

enum ParamValues {
    WithArea(Vec<(i32, i8, String, i32, String, i8, i32)>),
}

fn consumer(rx: Receiver<ParamValues>) {
    let mut conn = Connection::open("threaded_batched.db").unwrap();
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
              PRAGMA synchronous = 0;
              PRAGMA cache_size = 1000000;
              PRAGMA locking_mode = EXCLUSIVE;
              PRAGMA temp_store = MEMORY;",
    )
        .expect("PRAGMA");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS testing (
                id INTEGER not null primary key,
                amount INTEGER not null,
                posted INTEGER not null,
                address VARCHAR(40) not null,
                account_number INTEGER not null,
                account_name VARCHAR(40) not null,
                status INTEGER not null,
                phone INTEGER not null)",
        [],
    )
        .unwrap();
    let tx = conn.transaction().unwrap();
    {
        // jeez, refactor this!
        let mut with_area_params = " (NULL, ?, ?, ?, ?, ?, ?, ?),".repeat(MIN_BATCH_SIZE as usize);
        with_area_params.pop();
        let with_area_params = with_area_params.as_str();
        let st1 = format!("INSERT INTO testing VALUES {}", with_area_params);
        let mut stmt_with_area = tx.prepare_cached(st1.as_str()).unwrap();
        for param_values in rx {
            let mut row_values: Vec<&dyn ToSql> = Vec::new();
            match param_values {
                ParamValues::WithArea(values) => {
                    for batch in values.iter() {
                        row_values.push(&batch.0 as &dyn ToSql);
                        row_values.push(&batch.1 as &dyn ToSql);
                        row_values.push(&batch.2 as &dyn ToSql);
                        row_values.push(&batch.3 as &dyn ToSql);
                        row_values.push(&batch.4 as &dyn ToSql);
                        row_values.push(&batch.5 as &dyn ToSql);
                        row_values.push(&batch.6 as &dyn ToSql);
                    }
                    stmt_with_area.execute(&*row_values).unwrap();
                }
            }
        }
    }
    tx.commit().unwrap();
}

fn producer(tx: Sender<ParamValues>, count: i64) {
    if count < MIN_BATCH_SIZE {
        panic!("count cant be less than min batch size");
    }
    for _ in 0..(count / MIN_BATCH_SIZE) {
        let ammount = common::get_random_amount();
        let posted = common::get_random_posted();
        let account_number = common::get_random_account_number();
        let status = common::get_random_status();
        let phone = common::get_randome_phone_number();
        let mut param_values: Vec<_> = Vec::new();

        // lets prepare the batch
        let mut vector = Vec::<(i32, i8, String, i32, String, i8, i32)>::new();
        for _ in 0..MIN_BATCH_SIZE {
            let address = common::get_random_address();
            let account_name = common::get_random_account_name();
            vector.push((
                ammount,
                posted,
                address,
                account_number,
                account_name,
                status,
                phone,
            ));
        }
        for batch in vector.iter() {
            param_values.push(&batch.0 as &dyn ToSql);
            param_values.push(&batch.1 as &dyn ToSql);
            param_values.push(&batch.2 as &dyn ToSql);
            param_values.push(&batch.3 as &dyn ToSql);
            param_values.push(&batch.4 as &dyn ToSql);
            param_values.push(&batch.5 as &dyn ToSql);
            param_values.push(&batch.6 as &dyn ToSql);
        }
        // send the values
        tx.send(ParamValues::WithArea(vector)).unwrap();
    }
}

fn main() {
    // setup the DB and tables
    let (tx, rx): (Sender<ParamValues>, Receiver<ParamValues>) = mpsc::channel();
    // lets launch the consumer
    let consumer_handle = thread::spawn(|| consumer(rx));

    let cpu_count = num_cpus::get();
    // let total_rows = 100_000_000;
    let total_rows = 10_000_000;
    let each_producer_count = (total_rows / cpu_count) as i64;
    let mut handles = Vec::with_capacity(cpu_count);
    for _ in 0..cpu_count {
        let thread_tx = tx.clone();
        handles.push(thread::spawn(move || {
            producer(thread_tx, each_producer_count.clone())
        }))
    }
    for t in handles {
        t.join().unwrap();
    }
    drop(tx);
    // wait till consumer is exited
    consumer_handle.join().unwrap();
}
