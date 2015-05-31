# doppler
Command line utility that takes IQ data stream as input and produces doppler corrected output stream based on TLE.
Firstly it was written in C ([last commit to C version](https://github.com/cubehub/doppler/commit/e6df4d271ece09a88b8dba9b054bb10bdcb996ce)), however now it is rewritten in [rust](http://www.rust-lang.org).

## dependencies
### libgpredict
Follow install instructions from here: https://github.com/cubehub/libgpredict

### rust
http://www.rust-lang.org/install.html

    curl -sSf https://static.rust-lang.org/rustup.sh | sh

## build

    git clone https://github.com/cubehub/doppler.git
    cd doppler
    cargo build --release

## install
### mac os x

    cp target/release/doppler /usr/local/bin/

### linux

    sudo cp target/release/doppler /usr/local/bin/

## usage
### help

    doppler -h
    doppler track -h
    doppler const -h

### realtime
Do realtime doppler correction to ESTCube-1 satellite that transmits on 437.505 MHz and write output to a file.
Notice that `rtl_sdr` is tuned to 437.500 MHz, but ESTCube-1 transmits on 437.505 MHz, therefore 5000 Hz constant offset correction is also added with `--offset` parameter. It can be omitted if there is no offset.

    rtl_sdr -f 437500000 -s 1024000 -g 20 - | doppler track -s 1024000 -i i16 --tlefile cubesat.txt --tlename 'ESTCUBE 1' --location lat=58.26541,lon=26.46667,alt=76.1 --frequency 437505000 --offset 5000 > zero.iq

### recording
Do doppler correction to a file that is recorded before. For example someone has recorded an overpass and you would like to convert it to another file where doppler compensation has been made.
If parameter `--time` is specified it does doppler correction based on this time instead of real time. It denotes start time of the recording in UTC.

    cat last_overpass_256000sps_i16.iq | doppler track -s 256000 -i i16 --tlefile cubesat.txt --tlename 'ESTCUBE 1' --location lat=58.26541,lon=26.46667,alt=76.1 -frequency 437505000 --offset -2500 --time 2015-01-22T09:07:16 > zero_overpass.iq

    sox -t wav last_overpass.wav -esigned-integer -b16  -r 300000 -t raw - | doppler track -s 300000 -i i16 --tlefile cubesat.txt --tlename 'ESTCUBE 1' --location lat=58.26541,lon=26.46667,alt=76.1 -frequency 437505000 --offset -2500 --time 2015-01-22T09:07:16 > zero_overpass.iq

Notice that if dealing with old files you also have to use TLEs from that day, otherwise doppler correction result might be off. Here offset compensation of -2500 Hz is used only for example purposes.

### baseband shifting
It is also possible to just shift baseband signal "left" or "right" using `const` mode. In this example input signal has float IQ data format therefore `-i f32` is used. However output is automatically converted to int16 IQ data format.

    cat baseband_256000sps_f32.iq | doppler const -s 256000 -i f32 --shift -15000 > shifted_baseband_256000sps_i16.iq
