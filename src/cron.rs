use tokio_cron_scheduler::{JobScheduler, Job};

pub async fn init() {
    let sched = JobScheduler::new().await.unwrap();
    let _r = sched.add(
        Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds. {}", _uuid);
        })
        .unwrap()
    ).await.unwrap();
    println!("Scheduled job: {}", _r);

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();
    /*
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
          println!("Shut down done");
        })
      }));
*/
    let _r = sched.start().await;
    //tokio::time::sleep(core::time::Duration::from_secs(100)).await;
}
