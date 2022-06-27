mod lib;

use lib::process;
use lib::CONF_FILE;

fn main() {
    process(CONF_FILE);
}
