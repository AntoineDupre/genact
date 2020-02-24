use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::{HOSTS_LIST, ANSIBLETASKS_LIST};
use rand::prelude::*;
use rand::seq::SliceRandom;
use regex::Regex;
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
    //b320b-a101930-cab05-ctl-ioc-01 : ok=0    changed=0    unreachable=1    failed=0   
    //g-controlroom-cc-0         : ok=17   changed=6    unreachable=0    failed=0  
    for h in HOSTS_LIST.clone().into_iter() {
        //let failed = rng.
        print!("{}", Paint::green(format!("{:<25}: ", h)));
        print!("{}", Paint::green(format!("ok={:<5}", nbrtasks)));
        print!("{}", Paint::white(format!("changed={:<5}", 3)));
        print!("{}", Paint::white(format!("unreachable={:<5}", 0)));
        println!("{}", Paint::white(format!("failed={:<5}", 0)));
    }
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();

    for t in ANSIBLETASKS_LIST.clone().into_iter() {
        println!("");
        let tk = t.to_string();
        task(&tk);
        if tk == "Gathering Facts" {
            csleep(rng.gen_range(500, 3000));
        }
        else {
            let nbr_hosts = rng.gen_range(1, HOSTS_LIST.len());
            for n in 0..nbr_hosts {
                if appconfig.should_exit() {
                    break;
                }
                let host_name = HOSTS_LIST.choose(&mut rng);
                let host_name = host_name.as_deref().unwrap_or(&"g-v-ec-0");
                match rng.gen_range(1,20) {
                    1|2|3|4|5 => {
                        ok(host_name);
                        csleep(rng.gen_range(50, 1000));
                    }
                    6|7|8 => skipping(host_name),
                    9 => {
                        changed(host_name);
                        csleep(rng.gen_range(50, 1000));
                    }
                    10 => {
                        failed(host_name);
                        csleep(rng.gen_range(50, 1000));
                    }
                    _ => {},
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
