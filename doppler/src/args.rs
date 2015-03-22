

extern crate getopts;
use std::env;


pub struct Args {
    pub args: Vec<String>,
    pub opts: getopts::Options,
}

impl Args {

    pub fn new() -> Args {
        Args{   args: env::args().collect(),
                opts: getopts::Options::new(),
            }
    }

    pub fn print_usage(&self) {
        let ref progname = self.args[0];
        let brief = format!("Usage: {} [options]", progname);
        print!("{}", self.opts.usage(&brief));
    }

    pub fn parse_args(&mut self) -> Result<getopts::Matches, getopts::Fail> {
        self.opts.reqopt("s", "samplerate", "input data stream samplerate", "<sps>");
        self.opts.reqopt("i", "inputtype", "input data stream type", "<i16,f32>");

        self.opts.optflag("c", "const", "constant shift mode");
        self.opts.optflag("d", "doppler", "doppler correction mode");

        self.opts.optopt("t", "tlefile", "doppler: TLE file", "<filename>");
        self.opts.optopt("n", "tlename", "doppler: which TLE to use from TLE file", "<tle name>");
        self.opts.optopt("l", "location", "doppler: specifies observer location on earth", "<lat,lon,alt>");
        self.opts.optopt("f", "freq", "doppler: specifies object transmission frequency in Hz", "<Hz>");
        self.opts.optopt("", "time", "doppler: specifies observation start time in UTC (eg. 2015-01-31T17:00:01), uses current time if not specified", "<Y-m-dTH:M:S>");

        self.opts.optopt("o", "offset", "doppler/const: specifies by how much input stream will be constantly shifted in Hz", "<Hz>");

        self.opts.optopt("", "log", "logs information about frequnecy shifting to a file", "<logfile>");
        self.opts.optflag("h", "help", "prints this usage information");


        let matches = self.opts.parse(self.args.tail());

        matches
    }
}
