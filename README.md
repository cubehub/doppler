# doppler
Command line utility that takes IQ data stream as input and produces doppler corrected output stream based on TLE

## dependencies
### mac os x
    brew install glib

### linux
    sudo apt-get install libglib2.0-dev

## install
    git clone https://github.com/cubehub/doppler.git
    cd doppler
    git submodule init
    git submodule update
    mkdir build
    cd build
    cmake ../
    make
    make install

## usage
Make realtime doppler correction to ESTCube-1 satellite that transmits on 437.505 MHz and write output to a file.
Notice that rtl_sdr is tuned to 437.500 MHz, but ESTCube-1 transmits on 437.505 MHz, therefore 5000 Hz constant offset correction is also added with -o parameter.

    rtl_sdr -f 437500000 -s 1024000 -g 20 - | doppler -s 1024000 -i i16 -d -t cubesat.txt -n 'ESTCUBE 1' --location lat=58.26541,lon=26.46667,alt=76.1 -f 437505000 -o 5000 > zero.iq

Make doppler correction to a file that is recorded before. For example someone has recorded an overpass and you want to get another file with doppler compensation.
Parameter --time specifies when recording is taken. It does doppler correction based on this time instead of using real time.

    cat last_overpass_256000sps_i16.iq | doppler -s 256000 -i i16 -d -t cubesat.txt -n 'ESTCUBE 1' --location lat=58.26541,lon=26.46667,alt=76.1 -f 437505000 -o 5000 --time 2015-01-22T09:07:16 > zero_overpass.iq

Notice that if dealing with old files you also have to use TLEs from that day, otherwise doppler correction result might be off.
