mod cleaning;
mod cli_tools;
mod configs;
mod logging;
mod utils;

use cli_tools::run_clean_up;

fn main() {
    run_clean_up();
}
