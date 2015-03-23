
extern crate docopt;
mod usage;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = usage::args();

    //println!("{:?}", args);

    println!("doppler {} andres.vahter@gmail.com\n\n", VERSION);

    if args.get_bool("const") {
        println!("constant shift mode");

        println!("\tIQ samplerate   : {}", args.get_str("--samplerate"));
        println!("\tIQ data type    : {}\n", args.get_str("--intype"));

        println!("\tfrequency shift : {} Hz", args.get_str("--shift"));
    }
    else if args.get_bool("track") {
        println!("tracking mode");

        println!("\tIQ samplerate   : {}", args.get_str("--samplerate"));
        println!("\tIQ data type    : {}\n", args.get_str("--intype"));

        println!("\tTLE file        : {}", args.get_str("--tlefile"));
        println!("\tTLE name        : {}", args.get_str("--tlename"));
        println!("\tlocation        : {}", args.get_str("--location"));
        println!("\ttime            : {}", args.get_str("--time"));
        println!("\tfrequency       : {} Hz", args.get_str("--freq"));
        println!("\tfrequency shift : {} Hz", args.get_str("--shift"));
    }
}
