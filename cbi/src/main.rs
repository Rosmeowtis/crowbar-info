pub(crate) mod cli;
pub(crate) mod fmt;

use cli::{cli, Cmd};
use std::sync::Arc;

fn main() {
    let (cmd, hosts) = cli();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();
    rt.block_on(run(cmd, hosts));
}

async fn run(cmd: Cmd, hosts: Vec<String>) {
    if hosts.len() == 0 {
        eprintln!("[cbi Erorr] No hosts specified");
        return;
    }
    // 异步查询
    let a2s = a2s::A2SClient::new().await.unwrap();
    let a2s = Arc::new(a2s);
    // let a2s = Arc::new(a2s);
    match cmd {
        Cmd::Info => {
            let mut jobs = vec![];
            let mut results = vec![];
            for host in hosts {
                let a2s = a2s.clone();
                let handle = tokio::spawn(async move {
                    let res = a2s.info(&host).await;
                    res
                });
                jobs.push(handle);
            }
            for job in jobs {
                let res = job.await.unwrap();
                results.push(res);
            }
            fmt::fmt_info(results);
        }
        Cmd::Players => {
            let mut jobs = vec![];
            let mut results = vec![];
            for host in hosts {
                let a2s = a2s.clone();
                let handle = tokio::spawn(async move {
                    let res = a2s.players(&host).await;
                    res
                });
                jobs.push(handle);
            }
            for job in jobs {
                let res = job.await.unwrap();
                results.push(res);
            }
            println!("{:?}", results);
        }
        Cmd::Full => {
            let mut jobs = vec![];
            let mut results = vec![];
            for host in hosts {
                let a2s = a2s.clone();
                let handle = tokio::spawn(async move {
                    let res1 = a2s.info(&host).await;
                    let res2 = a2s.players(&host).await;
                    (res1, res2)
                });
                jobs.push(handle);
            }
            for job in jobs {
                let res = job.await.unwrap();
                results.push(res);
            }
            println!("{:?}", results);
        }
    }
}
