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
