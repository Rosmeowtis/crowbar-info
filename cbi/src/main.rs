pub(crate) mod cli;
pub(crate) mod fmt;
pub(crate) mod json_loop;

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
    if hosts.len() == 0 && cmd != Cmd::Json {
        eprintln!("[cbi Erorr] No hosts specified");
        return;
    }
    // 异步查询
    let a2s = a2s::A2SClient::new().await.unwrap();
    let a2s = Arc::new(a2s);
    // let a2s = Arc::new(a2s);
    match cmd {
        Cmd::Info => run_info(hosts, &a2s).await,
        Cmd::Players => run_players(hosts, &a2s).await,
        Cmd::Full => run_full(hosts, &a2s).await,
        Cmd::Json => {
            json_loop::main_loop(a2s.clone()).await;
        }
    }
}

async fn run_info(hosts: Vec<String>, a2s: &Arc<a2s::A2SClient>) {
    let hosts = parse_hosts(hosts);
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

async fn run_players(hosts: Vec<String>, a2s: &Arc<a2s::A2SClient>) {
    let hosts = parse_hosts(hosts);
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
    fmt::fmt_players(results);
}

async fn run_full(hosts: Vec<String>, a2s: &Arc<a2s::A2SClient>) {
    let hosts = parse_hosts(hosts);
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
    fmt::fmt_fulls(results);
}

/// 解析hosts参数，如果为 @ 开头则从文件读取，每行一个，忽略空行。
/// 最后将所有 hosts 合并为一个 Vec<String>
fn parse_hosts(hosts: Vec<String>) -> Vec<String> {
    let mut res = vec![];
    for host in hosts {
        if host.starts_with("@") {
            let path = host.trim_start_matches("@");
            let file = std::fs::read_to_string(path).unwrap();
            for line in file.lines() {
                let line = line.trim();
                if line.len() > 0 {
                    res.push(line.to_string());
                }
            }
        } else {
            res.push(host);
        }
    }
    res
}
