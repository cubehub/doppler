
mod args;
use std::env;

fn main() {
    let mut args = args::Args::new();
    let matches = args.parse_args();

    let matches = match args.opts.parse(args.args.tail()) {
        Ok(m) => { m }
        Err(f) => {
            print!("ERROR: {}\n\n", f.to_string());
            args.print_usage();
            env::set_exit_status(1);
            return;
        }
    };

    if matches.opt_present("h") {
        args.print_usage();
        return;
    }

}
