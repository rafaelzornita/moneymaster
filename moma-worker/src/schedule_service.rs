use chrono::{Days, Local};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::services::{periodic_report, timed_out_routines::remove_timed_out_routines};

pub struct ScheduledService {
    scheduler: JobScheduler,    
}

impl ScheduledService {
    pub async fn stop(&mut self) {
        match self.scheduler.shutdown().await{
            Ok(_) => println!("Scheduler stopped"),
            Err(e) => println!("Scheduler error: {:?}", e.to_string())
        }
    }
}
pub async fn initialize() -> Result<ScheduledService, String> {
    
    let scheduler = match JobScheduler::new().await {
        Ok(obj) => obj,
        Err(e) => {
            println!("Scheduler can't be loaded. Error: {:?}", e.to_string());
            return Err(e.to_string());
        }
    };

    //Monday at 7:00 AM
    let job: Result<Job, tokio_cron_scheduler::JobSchedulerError> =Job::new_cron_job_async("0 0 10 * * MON", |uuid, mut l| {
        Box::pin(async move {
            let start_date = Local::now().naive_local().date().checked_sub_days(Days::new(4)).unwrap();
            
            periodic_report::send_tenants_periodic_report(start_date).await; 

            let next_tick = l.next_tick_for_job(uuid).await;
            match next_tick {
                Ok(Some(ts)) => println!("Next time for monday's tenants periodic report job is {:?}", ts),
                _ => (),
            }
        })
    });

    if let Ok(j) = job {
        match scheduler.add(j).await{
            Ok(_) => println!("Monday Job added to scheduler"),
            Err(e) => {
                println!("Job can't be added to scheduler. Error: {:?}", e.to_string());
                return Err(e.to_string());
            }
        }
    }       

    //Thursday at 7:00 AM
    let job: Result<Job, tokio_cron_scheduler::JobSchedulerError> =Job::new_cron_job_async("0 0 10 * * THU", |uuid, mut l| {
        Box::pin(async move {
            let start_date = Local::now().naive_local().date().checked_sub_days(Days::new(3)).unwrap();
            
            periodic_report::send_tenants_periodic_report(start_date).await; 

            let next_tick = l.next_tick_for_job(uuid).await;
            match next_tick {
                Ok(Some(ts)) => println!("Next time for monday's tenants periodic report job is {:?}", ts),
                _ => (),
            }
        })
    });
    
    if let Ok(j) = job {
        match scheduler.add(j).await{
            Ok(_) => println!("Thursday Job added to scheduler"),
            Err(e) => {
                println!("Job can't be added to scheduler. Error: {:?}", e.to_string());
                return Err(e.to_string());
            }
        }
    }

    //Every 20 seconds, cancel timed out routines
    let job=Job::new_cron_job_async("1/20 * * * * *", |_, _| {
        Box::pin(async move {            
            remove_timed_out_routines().await;
        })
    });
    
    if let Ok(j) = job {
        match scheduler.add(j).await{
            Ok(_) => println!("Timed out routines Job added to scheduler"),
            Err(e) => {
                println!("Job can't be added to scheduler. Error: {:?}", e.to_string());
                return Err(e.to_string());
            }
        }
    }
    
    match scheduler.start().await{
        Ok(_) => println!("Scheduler started"),
        Err(e) => println!("Scheduler error: {:?}", e.to_string())
    }

    Ok(ScheduledService {
        scheduler: scheduler
    })
    
}


