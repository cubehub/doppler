/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h>

#define TLE_FILE_NAME_LEN 512
#define TLE_NAME_FIELD_LEN 512
#define OUTPUT_FILE_NAME_LEN 512

typedef struct {
	int arg_samplerate;
	int samplerate;

	int arg_const_mode;

	int arg_doppler_mode;

	int arg_tlefile;
	char tlefile[TLE_FILE_NAME_LEN];

	int arg_tlename;
	char tlename[TLE_NAME_FIELD_LEN];

	int arg_lat;
	int arg_lon;
	int arg_alt;
	double lat;
	double lon;
	double alt;

	int arg_freg_hz;
	int freq_hz;

	int arg_output_file;
	char output_file[OUTPUT_FILE_NAME_LEN];
} args_t;

void print_help() {
	printf("doppler\t(C) 2015 Andres Vahter (andres.vahter@gmail.com)\n\n");
	printf("doppler takes signed 16 bit IQ data stream as input and produces doppler corrected or constant shifted output\n");
	printf("usage: doppler args\n");
	printf("\t--const \t-c \t\t\t: constant shift mode: needs also --freq parameter\n");
	printf("\t--doppler \t-d \t\t\t: doppler correction mode: needs also --freq, --tlefile, --tlename and --location parameters\n\n");

	printf("\t--samplerate \t-s <samplerate>\t\t: input data stream samplerate\n");
	printf("\t--tlefile \t-t <filename>\t\t: doppler: TLE file\n");
	printf("\t--tlename \t-n <name>\t\t: doppler: which TLE to use from TLE file\n");
	printf("\t--location \t-l <lat,lon,alt>\t: doppler: specifies observer location on earth\n");
	printf("\t--freq \t\t-f <freq_hz>\t\t: doppler: specifies object transmission frequency in Hz\n");
	printf("\t\t\t\t\t\t: const: specifies by how much input stream will be shifted in Hz\n\n");

	printf("\t--output \t-o <filename>\t\t: logs information about frequnecy shifting to a file\n");
	printf("\t--help \t\t-h \t\t\t: prints this usage information\n");
}

int main(int argc, char *argv[]) {
	int opt = 0;
	int long_index = 0;
	char* subopts;
	char* value;

	args_t args;
	memset((void*)&args, 0, sizeof(args_t));

	static struct option long_options[] = {
		{"samplerate",	required_argument,	0,		's' }, // samplerate of input IQ data stream

		{"const",		no_argument,		0,		'c' }, // constant shift mode and its parameters

		{"doppler",		no_argument,		0,		'd' }, // doppler mode and its parameters
		{"tlefile",		required_argument,	0,		't' },
		{"tlename",		required_argument,	0,		'n' },
		{"location",	required_argument,	0,		'l' },

		{"freq",		required_argument,	0,		'f' }, // const mode: frequency shift, doppler mode: original signal frequency
		{"output",		required_argument,	0,		'o' }, // log doppler correction information to file
		{"help",		required_argument,	0,		'h' },
		{NULL,			0,					NULL,	0	}
	};

	enum {
		LAT_OPTION = 0,
		LON_OPTION,
		ALT_OPTION,
	};

	const char* location_opts[] = {
		[LAT_OPTION] = "lat",
		[LON_OPTION] = "lon",
		[ALT_OPTION] = "alt",
		NULL
	};

	while ((opt = getopt_long(argc, argv,"s:cdt:n:l:f:o:h", long_options, &long_index )) != -1) {
		switch (opt) {
			case 's' :
				args.arg_samplerate = 1;
				args.samplerate = atoi(optarg);
				if (args.samplerate < 1) {
					printf("samplerate must be > 0\n");
					exit(EXIT_FAILURE);
				}
				break;
			case 'c' :
				args.arg_const_mode = 1;
				break;
			case 'd' :
				args.arg_doppler_mode = 1;
				break;
			case 'f' :
				args.arg_freg_hz = 1;
				args.freq_hz = atoi(optarg);
				break;
			case 't' :
				args.arg_tlefile = 1;
				if (strlen(optarg) < TLE_FILE_NAME_LEN) {
					memcpy(&(args.tlefile[0]), optarg, strlen(optarg));
				}
				else {
					printf("--tlefile (-t) argument %s is longer than %u, cannot use it as input!\n", optarg, TLE_FILE_NAME_LEN);
					exit(EXIT_FAILURE);
				}
				break;
			case 'n' :
				args.arg_tlename = 1;
				if (strlen(optarg) < TLE_NAME_FIELD_LEN) {
					memcpy(&(args.tlename[0]), optarg, strlen(optarg));
				}
				else {
					printf("--tlename (-n) argument %s is longer than %u, cannot use it as input!\n", optarg, TLE_NAME_FIELD_LEN);
					exit(EXIT_FAILURE);
				}
				break;
			case 'l' :
				subopts = optarg;
				while (*subopts != '\0') {
					char* saved = subopts;
					switch(getsubopt(&subopts, (char **)location_opts, &value)) {
						case LAT_OPTION:
							args.arg_lat = 1;
							args.lat = strtod(value, NULL);
							break;
						case LON_OPTION:
							args.arg_lon = 1;
							args.lon = strtod(value, NULL);
							break;
						case ALT_OPTION:
							args.arg_alt = 1;
							args.alt = strtod(value, NULL);
							break;
						default:
							printf("incorrect suboption: '%s'\n", saved);
							printf("correct usage is: --location (-l) lat=58.64560,lon=23.15163,alt=7.8\n");
							exit(EXIT_FAILURE);
					}
				}

				if (!args.arg_lat) {
					printf("'lat' is not specified with --location (-l) argument\n");
				}

				if (!args.arg_lon) {
					printf("'lon' is not specified with --location (-l) argument\n");
				}

				if (!args.arg_alt) {
					printf("'alt' is not specified with --location (-l) argument\n");
				}

				if (!args.arg_lat || !args.arg_lon || !args.arg_alt) {
					printf("correct usage is: --location (-l) lat=58.64560,lon=23.15163,alt=7.8\n");
					exit(EXIT_FAILURE);
				}
				break;
			case 'o' :
				args.arg_output_file = 1;
				if (strlen(optarg) < OUTPUT_FILE_NAME_LEN) {
					memcpy(&(args.output_file[0]), optarg, strlen(optarg));
				}
				else {
					printf("--output (-o) argument %s is longer than %u, cannot use it as input!\n", optarg, OUTPUT_FILE_NAME_LEN);
					exit(EXIT_FAILURE);
				}
				break;
			case 'h' :
				print_help();
				exit(EXIT_SUCCESS);
				break;
			 default:
				print_help();
				exit(EXIT_FAILURE);
		}
	}

	// arg samplerate
	if (!args.arg_samplerate) {
		printf("samplerate not specified!\n");
		exit(EXIT_FAILURE);
	}
	else {
		printf("samplerate: %u\n", args.samplerate);
	}

	// arg const mode
	if (args.arg_const_mode && args.arg_doppler_mode) {
		printf("--const (-c) and --doppler (-d) arguments cannot be used together\n");
		exit(EXIT_FAILURE);
	}

	if (args.arg_const_mode && args.arg_freg_hz) {
		printf("constant shift mode with %d Hz shift\n", args.freq_hz);
	}
	else if (args.arg_const_mode && !args.arg_freg_hz) {
		printf("constant shift mode also needs --freq (-f) argument to know how much to shift\n");
		exit(EXIT_FAILURE);
	}

	// arg doppler mode
	if (args.arg_doppler_mode && args.arg_freg_hz && args.arg_tlefile && args.arg_tlename && args.arg_lat && args.arg_lon && args.arg_alt) {
		printf("doppler correction mode\n");
		printf("\tTLE file: %s\n", args.tlefile);
		printf("\tTLE name: %s\n", args.tlename);
		printf("\tobserver location: lat %2.4f, lon %2.4f, alt %.1f m\n", args.lat, args.lon, args.alt);
	}
	if (args.arg_doppler_mode && !args.arg_freg_hz) {
		printf("doppler mode also needs --freq (-f) parameter which specifies object transmission frequency, ");
		printf("for example 'ESTCUBE 1' uses 437505000 Hz\n");
	}
	if (args.arg_doppler_mode && !args.arg_tlefile) {
		printf("doppler mode also needs --tlefile (-t) parameter which specifies file with TLEs\n");
		printf("such file can be downloaded from: https://celestrak.com/NORAD/elements/cubesat.txt\n");
	}
	if (args.arg_doppler_mode && !args.arg_tlename) {
		printf("doppler mode also needs --tlename (-n) parameter which specifies which TLE to use from TLE file\n");
		printf("for example use as --tlename -n 'ESTCUBE 1'\n");
	}
	if (args.arg_doppler_mode && (!args.arg_lat || !args.arg_lon || !args.arg_alt)) {
		printf("doppler mode also needs --location (-l) parameter which specifies observer location\n");
		printf("for example use as --location (-l) lat=58.64560,lon=23.15163,alt=7.8\n");
	}

	if (args.arg_doppler_mode && (!args.arg_freg_hz || !args.arg_tlefile || !args.arg_tlename || !args.arg_lat || !args.arg_lon || !args.arg_alt)) {
		printf("\ndoppler mode example command:\n\tdoppler -s 1024000 -d -f 437505000 -t cubesats.txt -n 'ESTCUBE 1' --location lat=58.64560,lon=23.15163,alt=7.8 -o dopplet.out\n");
		exit(EXIT_FAILURE);
	}

	// arg output file
	if (args.arg_output_file) {
		printf("write events to file: %s\n", args.output_file);
	}

	return 0;
}
