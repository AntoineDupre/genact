use rand::distributions::{ChiSquared, Distribution, Exp};
/// Module that pretends to run cargo to install rust packages.
use rand::prelude::*;
use std::time::Instant;
use yansi::Paint;

use crate::parse_args::AppConfig;
use crate::utils::csleep;
use crate::EX_LIST;
use crate::MOTOR_LIST;
use crate::TIMER_LIST;

pub fn run(appconfig: &AppConfig) {
    unsafe { _run(appconfig) }
}

unsafe fn _run(appconfig: &AppConfig) {
    static mut ipython_number: i32 = 0;
    static mut scan_id: i32 = 523;
    let mut rng = thread_rng();
    let ct = rng.gen_range(1, 3);
    let start = rng.gen_range(100, 500);
    let end = rng.gen_range(550, 1000);
    let step = rng.gen_range(20, 100);
    let mut count = 0;
    let mot_name = MOTOR_LIST.choose(&mut rng);
    let motor_name = mot_name.as_deref().unwrap_or(&"mono_energy");
    let tim_name = TIMER_LIST.choose(&mut rng);
    let timer_name = tim_name.as_deref().unwrap_or(&"vodka_ct");
    let scan_t = ["fscan", "ascan", "mesh"].choose(&mut rng);
    let scan_type = scan_t.as_deref().unwrap_or(&"ascan");
    let exp_chan: Vec<_> = EX_LIST.choose_multiple(&mut rng, 4).collect();
    let memas_1 = exp_chan[0];
    let memas_2 = exp_chan[1];
    let memas_3 = exp_chan[2];
    let memas_4 = exp_chan[3];
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
    csleep(1000);
    println!(
        "{message}",
        message = Paint::blue("Operation will be saved in /data/master/kits/scan.h5 (HDF5::NXscan from NXscanH5_FileRecoder)")
        );
    println!(
        "{message}",
        message = Paint::blue(format!("Scan #{scan_id} Started", scan_id = scan_id)),
    );

    println!(
        "{pt:^15} {motor_name:^15} {counter_name:^15} {meas_name1:^15} {meas_name2:^15} {meas_name3:^15} {meas_name4:^15} {dt:^15}",
        pt = "#Pt No",
        motor_name = motor_name,
        counter_name = timer_name,
        meas_name1 = memas_1,
        meas_name2 = memas_2,
        meas_name3 = memas_3,
        meas_name4 = memas_4,
        dt = "dt",
    );
    let mut pulse1: f64 = rng.gen_range(1, 203) as f64;
    let mut pulse2: f64 = rng.gen_range(2, 207) as f64;
    let mut pulse3: f64 = rng.gen_range(3, 215) as f64;
    let mut pulse4: f64 = rng.gen_range(4, 226) as f64;
    let mut pulse: f64 = rng.gen_range(1, 40) as f64;
    csleep(500);
    let mut t = 0;
    for n in (start..end).step_by((end - start) / step) {
        t += ct;
        println!(
            " {count:^15} {n:^15} {ct:^15} {pulse1:^15} {pulse2:^15} {pulse3:^15} {pulse4:^15} {dt:^15}",
            count = count,
            n = n,
            ct = ct,
            pulse1 = format!("{a:.6}", a=pulse1 / pulse),
            pulse2 = format!("{a:.6}", a= pulse2 / pulse),
            pulse3 = format!("{a:.6}", a=pulse3 / pulse),
            pulse4 = format!("{a:.6}", a=pulse4 / pulse),
            dt = t,
        );
        pulse1 = rng.gen_range(1, 200) as f64;
        pulse2 = rng.gen_range(1, 200) as f64;
        pulse3 = rng.gen_range(1, 200) as f64;
        pulse4 = rng.gen_range(1, 200) as f64;
        pulse = rng.gen_range(1, 40) as f64;
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
    println!(
        "{message}",
        message = Paint::blue(format!("Scan #{scan_id} ended", scan_id = scan_id)),
    );
    ipython_number += 1;
    scan_id += 1;
    csleep(1000);
}
