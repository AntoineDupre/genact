use rand::prelude::*;
use yansi::Paint;
extern crate chrono;
use chrono::{DateTime, Utc};

use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::BOOTLOG_LIST;
use crate::EXCEPTIONS_LIST;
use crate::EX_LIST;
use crate::MOTOR_LIST;
use crate::TIMER_LIST;

fn get_one_entry<'a>(source: Vec<&'static str>, default: &'a str) -> &'a str {
    let mut rng = thread_rng();
    let entry_name = source.choose(&mut rng);
    let value = entry_name.as_deref().unwrap_or(&default);
    value
}

fn scan_input(
    ipython_number: i32,
    scan_type: &str,
    motor_name: &str,
    start: i32,
    end: i32,
    step: i32,
    ct: i32,
) {
    println!(
        "{ipython}: {scan_type} {motor_name} {motstart} {motend} {step} {ct}",
        ipython = Paint::green(format!(
            "[{ipython_number}]",
            ipython_number = ipython_number,
        ))
        .bold(),
        motor_name = motor_name,
        scan_type = scan_type,
        motstart = start,
        motend = end,
        step = step,
        ct = ct,
    );
}

pub fn run(appconfig: &AppConfig) {
    let mut rng = thread_rng();
    let mut ipython_number: i32 = rng.gen_range(30, 200);
    let mut scan_id: i32 = rng.gen_range(100, 300);
    for _ in 0..30 {
        oneloop(appconfig, ipython_number, scan_id);
        ipython_number += 1;
        scan_id += 1;
        csleep(1000);
    }
}

fn oneloop(appconfig: &AppConfig, ipython_number: i32, scan_id: i32) {
    let mut rng = thread_rng();
    let ct = rng.gen_range(1, 3);
    let start = rng.gen_range(100, 500);
    let end = rng.gen_range(550, 1000);
    let step = rng.gen_range(20, 100);
    let nbr_chan = rng.gen_range(2, 6);
    let mut count = 0;
    let motor_name = get_one_entry(MOTOR_LIST.to_vec(), &"mono_energy");
    let timer_name = get_one_entry(TIMER_LIST.to_vec(), &"vodka_ct");
    let scan_type = get_one_entry(["fscan", "ascan", "mesh"].to_vec(), &"ascan");
    let mut exp_chan = Vec::new();
    let mut exp_chan_ranges = Vec::new();
    for n in 0..nbr_chan {
        let ct = get_one_entry(EX_LIST.to_vec(), &"em01_ch01");
        exp_chan.push(ct.clone());
        exp_chan_ranges.push(rng.gen_range(50, 5000));
    }
    let exp_chan: Vec<_> = EX_LIST.choose_multiple(&mut rng, 4).collect();
    println!(
        "{ipython}: {scan_type} {motor_name} {motstart} {motend} {step} {ct}",
        ipython = Paint::green(format!(
            "[{ipython_number}]",
            ipython_number = ipython_number,
        ))
        .bold(),
        motor_name = motor_name,
        scan_type = scan_type,
        motstart = start,
        motend = end,
        step = step,
        ct = ct,
    );
    //scan_input(ipython_number, scan_type, motor_name, start, end, step, ct);
    csleep(1000);
    println!(
            "{message}",
            message = Paint::blue("Operation will be saved in /data/master/kits/scan.h5 (HDF5::NXscan from NXscanH5_FileRecoder)")
            );
    let now: DateTime<Utc> = Utc::now();
    println!(
        "{message}",
        message = Paint::blue(format!(
            "Scan #{scan_id} started at {start_t}",
            scan_id = scan_id,
            start_t = now.to_rfc2822()
        )),
    );

    print!(
        "{pt:^15} {motor_name:^15} {counter_name:^15}",
        pt = "#Pt No",
        motor_name = motor_name,
        counter_name = timer_name,
    );
    for name in exp_chan {
        print!("{meas_name:^15}", meas_name = name,);
    }
    println!("{dt:^15}", dt = "dt",);
    csleep(500);
    let mut t = 0;
    for n in (start..end).step_by((end - start) / step) {
        t += ct;
        let mut pulses = Vec::<f64>::new();
        for r in &exp_chan_ranges {
            let nbr = rng.gen_range(1, r) as f64;
            pulses.push(nbr);
        }
        let mut pulse: f64 = rng.gen_range(1, 40) as f64;
        print!(
            " {count:^15} {n:^15} {ct:^15}",
            count = count,
            n = n,
            ct = ct,
        );
        for p in &pulses {
            print!("{pls:^15}", pls = format!("{a:.6}", a = p / pulse),);
        }
        println!("{dt:^15}", dt = t,);
        count += 1;
        csleep(ct * 100);
        if appconfig.should_exit() {
            return;
        }
    }
    csleep(500);
    println!(
        "{message}",
        message = Paint::blue("Operation saved in /data/master/kits/scan.h5")
    );
    let now: DateTime<Utc> = Utc::now();
    println!(
        "{message}",
        message = Paint::blue(format!(
            "Scan #{scan_id} ended at {end_t}",
            scan_id = scan_id,
            end_t = now.to_rfc2822()
        )),
    );
    if rng.gen_range(1, 3) == 1 {
        println!(
                "{message}",
                message = Paint::red(format!("An error occurred while running macro '{scan_type}({motor_name}, {motstart}, {motend}, {step}, {ct}) -> {id}':",
                    motor_name = motor_name,
                    scan_type = scan_type,
                    motstart = start,
                    motend = end,
                    step = step,
                    ct = ct,
                    id = "353eb23a-42fe-b06a-bc56876a6e6f0",
                ))
            );
        let exc = EXCEPTIONS_LIST.choose(&mut rng);
        let exc = exc.as_deref().unwrap_or(&"ValueError");
        let desc = BOOTLOG_LIST.choose(&mut rng);
        let desc = desc.as_deref().unwrap_or(&"Unknown error");
        println!(
            "{message}",
            message = Paint::red(format!(
                "{exception}: {description}",
                exception = exc,
                description = desc,
            ))
        );
    }
}
