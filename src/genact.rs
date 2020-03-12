use crate::parse_args::AppConfig;
use crate::syntect;
use crate::utils::{csleep, dprint, rdprint};
use crate::GENACT_LIST;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

pub fn run(appconfig: &AppConfig) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    const SPINNER_SLEEP: u64 = 200;
    const TEXT_SLEEP: u64 = 15;
    let simcity = "\n";
    let ranges: Vec<(Style, &str)> = h.highlight(simcity, &ps);
    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);

    dprint(escaped.to_string(), 0);
    GENACT_LIST.iter().for_each(|simcity| {
        let unchecked_checkbox = "";
        let checked_checkbox = "";
        let msg = format!("{}", simcity);
        dprint(unchecked_checkbox, 0);
        rdprint(msg, TEXT_SLEEP, appconfig);
        if appconfig.should_exit() {
            println!();
            return;
        }
        csleep(SPINNER_SLEEP);
        dprint("\r", 0);
        dprint(checked_checkbox, 10);
        let ranges: Vec<(Style, &str)> = h.highlight(simcity, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        dprint(escaped.to_string(), 0);
        if appconfig.should_exit() {
            return;
        }
        println!();
    })
}
