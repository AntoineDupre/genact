use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::{ANSIBLEPLAYS_LIST, ANSIBLETASKS_LIST, HOSTS_LIST};
use rand::prelude::*;
use rand::seq::SliceRandom;
use yansi::Paint;

fn ok(host: &str) {
    println!("{}", Paint::green(format!("ok: [{}]", host)));
}

fn skipping(host: &str) {
    println!("{}", Paint::cyan(format!("skipping: [{}]", host)));
}

fn changed(host: &str) {
    println!("{}", Paint::yellow(format!("changed: [{}]", host)));
}

fn failed(host: &str) {
    println!("{}", Paint::red(format!("failed: [{}]", host)));
}

fn task(task: &str) {
    let msg = format!("TASK [{}] ", task);
    let msgline = format!("{:*<92}", msg);
    println!("{}", msgline);
}

fn recap(nbrtasks: usize) {
    let mut rng = thread_rng();
    println!("\nPLAY RECAP *********************************************************************************");
    let nbrhosts = rng.gen_range(1, 21);
    let hosts: Vec<_> = HOSTS_LIST.choose_multiple(&mut rng, nbrhosts).collect();
    for h in hosts.into_iter() {
        let failed = rng.gen_range(0, nbrtasks / 8);
        let changed = rng.gen_range(0, (nbrtasks - failed) / 4);
        let unreachable = rng.gen_range(0, (nbrtasks - failed - changed) / 8);
        let ok = nbrtasks - failed - changed - unreachable;
        if (failed > 0) | (unreachable > 0) {
            print!("{}", Paint::red(format!("{:<25}: ", h)));
        } else {
            print!("{}", Paint::green(format!("{:<25}: ", h)));
        }
        print!("{}", Paint::green(format!("ok={:<5}", ok)));
        if changed > 0 {
            print!("{}", Paint::yellow(format!("changed={:<5}", changed)));
        } else {
            print!("{}", Paint::white(format!("changed={:<5}", 0)));
        }
        if unreachable > 0 {
            print!("{}", Paint::red(format!("unreachable={:<5}", unreachable)));
        } else {
            print!("{}", Paint::white(format!("unreachable={:<5}", 0)));
        }
        if failed > 0 {
            println!("{}", Paint::red(format!("failed={:<5}", failed)));
        } else {
            println!("{}", Paint::white(format!("failed={:<5}", 0)));
        }
    }
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    let nbr_tasks = rng.gen_range(5, 23);
    //for t in ANSIBLETASKS_LIST.clone().into_iter() {
    for _ in 0..nbr_tasks {
        let play = ANSIBLEPLAYS_LIST.choose(&mut rng).unwrap().to_string();
        let desc = ANSIBLETASKS_LIST.choose(&mut rng).unwrap().to_string();
        let tk = format!("{} : {}", play, desc);
        println!("");
        //let tk = t.to_string();
        task(&tk);
        if tk == "Gathering Facts" {
            csleep(rng.gen_range(500, 3000));
        } else {
            let nbr_hosts = rng.gen_range(1, HOSTS_LIST.len());
            for _ in 0..nbr_hosts {
                if appconfig.should_exit() {
                    break;
                }
                let host_name = HOSTS_LIST.choose(&mut rng);
                let host_name = host_name.as_deref().unwrap_or(&"g-v-ec-0");
                match rng.gen_range(1, 20) {
                    1 | 2 | 3 | 4 | 5 => {
                        ok(host_name);
                        csleep(rng.gen_range(50, 1000));
                    }
                    6 | 7 | 8 => skipping(host_name),
                    9 => {
                        changed(host_name);
                        csleep(rng.gen_range(50, 1000));
                    }
                    10 => {
                        failed(host_name);
                        csleep(rng.gen_range(50, 1000));
                    }
                    _ => {}
                }
                csleep(rng.gen_range(50, 1000));
            }
        }
        csleep(1000);
        if appconfig.should_exit() {
            break;
        }
    }
    recap(20);
}
