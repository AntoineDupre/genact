use crate::parse_args::AppConfig;
use crate::syntect;
use crate::utils::{csleep, dprint, rdprint};
use crate::GENACT_LIST;
use rand::prelude::*;
use yansi::Paint;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn run(appconfig: &AppConfig) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    const SPINNERS: &[&str] = &[""];
    const SPINNER_SLEEP: u64 = 200;
    const TEXT_SLEEP: u64 = 15;
    const MAX_SPINNER_LOOPS: u8 = 20;

    let mut rng = thread_rng();
    let mut simcity = "";

    // for _ in 0..500 {
    GENACT_LIST.iter().for_each(|simcity| {
        let spinner_loops = rng.gen_range(1, MAX_SPINNER_LOOPS);

        // Message chosen from "data/simcity.txt"
        // Thanks https://gist.github.com/erikcox/7e96d031d00d7ecb1a2f
        // let last_simcity = simcity;
        // // simcity = &GENACT_LIST.choose(&mut rng).unwrap_or(&"");
        // simcity = &GENACT_LIST.choose(&mut rng).unwrap_or(&"");

        // // Don't choose the same message twice in a row
        // while simcity == last_simcity {
        //     // Select another message
        //     simcity = &GENACT_LIST.choose(&mut rng).unwrap_or(&"");
        // }

        // Choose a status/resolution per "task"
        let resolution_id = 1 + rng.gen::<u8>() % 100;
        let mut resolution = match resolution_id {
            1...4 => "",
            5...9 => "",
            10...14 => "",
            _ => "",
        };

        // Prepare and color the messages
        let unchecked_checkbox = "";
        let checked_checkbox = "";

        // Keep track of when the message is first printed
        let mut first = true;

        'outer: for _ in 0..1 {
            for spinner in SPINNERS {
                // Output a message, with a checkbox in front and spinner behind
                let msg = format!("{}", simcity);

                // on first print, text appears letter by letter
                //if first {
                dprint(unchecked_checkbox, 0);
                rdprint(msg, TEXT_SLEEP);
                first = false;
                //} else {
                //    dprint(unchecked_checkbox, 0);
                //    dprint(msg, 0);
                //}
                // Wait a bit, then erase the line
                csleep(SPINNER_SLEEP);
                dprint("\r", 0);

                // Don't wait until finished, exit both loops if that is requested
                if appconfig.should_exit() {
                    resolution = "ABORTED";
                    break 'outer;
                }
            }
        }

        //// Select the color
        //let color_func = if resolution == "FAIL" || resolution == "ABORTED" {
        //    // Use red for FAIL
        //    Paint::white
        //} else {
        //    // Use white most of the time
        //    Paint::white
        //};

        // End of loop, the line has been removed, conclude the status
        dprint(checked_checkbox, 10);
        let ranges: Vec<(Style, &str)> = h.highlight(simcity, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        // dprint(color_func(format!("{}", simcity)).to_string(), 0);
        dprint(escaped.to_string(), 0);

        if appconfig.should_exit() {
            dprint("\nALL DONE\n", 0);
            return;
        }

        println!();
    })
}
