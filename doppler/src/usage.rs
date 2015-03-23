
use docopt;
use docopt::Docopt;

static USAGE: &'static str = "
doppler <andres.vahter@gmail.com>

Usage:
    doppler (const (--samplerate <sps> | -s <sps>) --intype <type> --shift <Hz>)
    doppler (track (--samplerate <sps> | -s <sps>) --intype <type> --tlefile <file> --tlename <name> --location <lat,lon,alt> --freq <Hz>) [--time <Y-m-dTH:M:S>] [--shift <Hz>]
    doppler (-h | --help | --version)

Options:
    -s --samplerate <sps>       IQ data samplerate.
    --intype <type>             IQ data type <i16, f32>.

    -h --help                   Show this screen.
    --version                   Show version.

Const mode options:
    --shift <Hz>                Constant frequency shift in Hz [default: 0].

Track mode options:
    --tlefile <file>            TLE database file eg. \"http://www.celestrak.com/NORAD/elements/cubesat.txt\".
    --tlename <name>            TLE name eg. 'ESTCUBE 1'.
    --location <lat,lon,alt>    Observer location on earth.
    --time <Y-m-dTH:M:S>        Observation start time. It should be specified if input is IQ data recording. Real time is used otherwise.
    --freq <Hz>                 Satellite transmitter frequency in Hz.
    --shift <Hz>                Constant frequency shift in Hz [default: 0].
";

pub fn args() -> docopt::ArgvMap {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());
    args
}
