extern crate bitcoin_asm;

use bitcoin_asm::config::Config;

fn main() {
	let config = Config::parse_args();

	bitcoin_asm::assemble_file(config);
}
