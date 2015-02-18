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
#include <getopt.h>

#include "predict.h"
#include "dsp.h"

#define TLE_FILE_NAME_LEN 512
#define TLE_NAME_FIELD_LEN 512
#define LOG_FILE_NAME_LEN 512

#define SPEED_OF_LIGHT_M_S 299792458.

#define INPUT_STREAM_BLOCK_SIZE 8192

typedef struct {
	int arg_samplerate;
	int samplerate;

	int arg_inputtype;
	int inputtype;

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

	int arg_freq_hz;
	int freq_hz;

	int arg_utc_time;
	struct tm utc_time;

	int arg_offset_hz;
	int offset_hz;

	int arg_log_file;
	char log_file[LOG_FILE_NAME_LEN];
} args_t;

char* utc_timestamp(struct tm* t){
	static char timestamp[50];
	time_t utime;

	if (t == NULL) {
		utime = time(NULL);
		strftime(timestamp, 50, "%Y-%m-%dT%H:%M:%SZ", gmtime(&utime));
	}
	else {
		strftime(timestamp, 50, "%Y-%m-%dT%H:%M:%SZ", t);
	}

	return timestamp;
}

void print_help() {
	fprintf(stderr, "doppler\t(C) 2015 Andres Vahter (andres.vahter@gmail.com)\n\n");
	fprintf(stderr, "doppler takes signed 16 bit IQ data stream as input and produces doppler corrected or constant shifted output\n");
	fprintf(stderr, "usage: doppler args\n");
	fprintf(stderr, "\t--samplerate \t-s <samplerate>\t\t: input data stream samplerate\n");
	fprintf(stderr, "\t--inputtype \t-i <i16, f32>\t\t: input data stream type\n\n");

	fprintf(stderr, "\t--const \t-c \t\t\t: constant shift mode: needs also --offset parameter\n");
	fprintf(stderr, "\t--doppler \t-d \t\t\t: doppler correction mode: needs also --freq, --tlefile, --tlename and --location parameters\n\n");

	fprintf(stderr, "\t--tlefile \t-t <filename>\t\t: doppler: TLE file\n");
	fprintf(stderr, "\t--tlename \t-n <name>\t\t: doppler: which TLE to use from TLE file\n");
	fprintf(stderr, "\t--location \t-l <lat,lon,alt>\t: doppler: specifies observer location on earth\n");
	fprintf(stderr, "\t--freq \t\t-f <freq_hz>\t\t: doppler: specifies object transmission frequency in Hz\n");
	fprintf(stderr, "\t--time \t\t<Y-m-dTH:M:S>\t\t: doppler: specifies observation start time in UTC (eg. 2015-01-31T17:00:01), uses current time if not specified\n\n");

	fprintf(stderr, "\t--offset \t-o <offset_hz>\t\t: doppler/const: specifies by how much input stream will be constantly shifted in Hz\n\n");

	fprintf(stderr, "\t--log \t\t<filename>\t\t: logs information about frequnecy shifting to a file\n");
	fprintf(stderr, "\t--help \t\t-h \t\t\t: prints this usage information\n");
}

int main(int argc, char *argv[]) {
	int opt = 0;
	int long_index = 0;
	char* subopts;
	char* value;

	uint8_t iq_buffer[INPUT_STREAM_BLOCK_SIZE];
	int bytes_read;
	int shifted_data_len;

	args_t args;
	memset((void*)&args, 0, sizeof(args_t));

	static struct option long_options[] = {
		{"samplerate",	required_argument,	0,		's' }, // samplerate of input IQ data stream
		{"inputtype",	required_argument,	0,		'i' }, // IQ data stream type: i16, f32

		{"const",		no_argument,		0,		'c' }, // constant shift mode and its parameters

		{"doppler",		no_argument,		0,		'd' }, // doppler mode and its parameters
		{"tlefile",		required_argument,	0,		't' },
		{"tlename",		required_argument,	0,		'n' },
		{"location",	required_argument,	0,		'l' },
		{"freq",		required_argument,	0,		'f' }, // object transmitter frequency
		{"time",		required_argument,	0,		 0  }, // specify time in UTC, default is current time

		{"offset",		required_argument,	0,		'o' }, // const mode: how much to shift, doppler mode: how much to shift constantly

		{"log",			required_argument,	0,		 0	}, // log activity to a file
		{"help",		required_argument,	0,		'h' },
		{NULL,			0,					NULL,	 0	}
	};

	enum {
		INPUT_TYPE_OPTION_I16 = 0,
		INPUT_TYPE_OPTION_F32,
	};

	const char* input_type_opts[] = {
		[INPUT_TYPE_OPTION_I16] = "i16",
		[INPUT_TYPE_OPTION_F32] = "f32",
		NULL
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

	// default values
	args.arg_inputtype = 1;
	args.inputtype = INPUT_TYPE_OPTION_I16;

	while ((opt = getopt_long(argc, argv,"s:i:cdt:n:l:f:o:h", long_options, &long_index )) != -1) {
		switch (opt) {
			case 0:
				if (strcmp("log", long_options[long_index].name) == 0) {
					args.arg_log_file = 1;
					if (strlen(optarg) < LOG_FILE_NAME_LEN) {
						memcpy(&(args.log_file[0]), optarg, strlen(optarg));
					}
					else {
						fprintf(stderr, "--log argument %s is longer than %u, cannot use it as filename!\n", optarg, LOG_FILE_NAME_LEN);
						exit(EXIT_FAILURE);
					}
				}

				if (strcmp("time", long_options[long_index].name) == 0) {
					if (strlen(optarg) == 19) {
						if (strptime(optarg, "%Y-%m-%dT%H:%M:%S", &args.utc_time) != NULL) {
							args.arg_utc_time = 1;
						}
					}

					if (!args.arg_utc_time) {
						fprintf(stderr, "there is error in timestamp, it should use format like 2015-01-31T17:00:01\n");
						exit(EXIT_FAILURE);
					}
				}
				break;

			case 's' :
				args.arg_samplerate = 1;
				args.samplerate = atoi(optarg);
				if (args.samplerate < 1) {
					fprintf(stderr, "samplerate must be > 0\n");
					exit(EXIT_FAILURE);
				}
				break;
			case 'i' :
				args.arg_inputtype = 1;

				if (strcmp("i16", optarg) == 0) {
					args.inputtype = INPUT_TYPE_OPTION_I16;
				}
				else if (strcmp("f32", optarg) == 0) {
					args.inputtype = INPUT_TYPE_OPTION_F32;
				}
				else {
					fprintf(stderr, "valid input IQ stream types are: i16, f32\n");
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
				args.arg_freq_hz = 1;
				args.freq_hz = atoi(optarg);
				break;
			case 't' :
				args.arg_tlefile = 1;
				if (strlen(optarg) < TLE_FILE_NAME_LEN) {
					memcpy(&(args.tlefile[0]), optarg, strlen(optarg));
				}
				else {
					fprintf(stderr, "--tlefile (-t) argument %s is longer than %u, cannot use it as input!\n", optarg, TLE_FILE_NAME_LEN);
					exit(EXIT_FAILURE);
				}
				break;
			case 'n' :
				args.arg_tlename = 1;
				if (strlen(optarg) < TLE_NAME_FIELD_LEN) {
					memcpy(&(args.tlename[0]), optarg, strlen(optarg));
				}
				else {
					fprintf(stderr, "--tlename (-n) argument %s is longer than %u, cannot use it as input!\n", optarg, TLE_NAME_FIELD_LEN);
					exit(EXIT_FAILURE);
				}
				break;
			case 'l' :
				subopts = optarg;
				while (*subopts != '\0') {
					char* saved = subopts;
					switch (getsubopt(&subopts, (char **)location_opts, &value)) {
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
							fprintf(stderr, "incorrect suboption: '%s'\n", saved);
							fprintf(stderr, "correct usage is: --location (-l) lat=58.64560,lon=23.15163,alt=7.8\n");
							exit(EXIT_FAILURE);
					}
				}

				if (!args.arg_lat) {
					fprintf(stderr, "'lat' is not specified with --location (-l) argument\n");
				}

				if (!args.arg_lon) {
					fprintf(stderr, "'lon' is not specified with --location (-l) argument\n");
				}

				if (!args.arg_alt) {
					fprintf(stderr, "'alt' is not specified with --location (-l) argument\n");
				}

				if (!args.arg_lat || !args.arg_lon || !args.arg_alt) {
					fprintf(stderr, "correct usage is: --location (-l) lat=58.64560,lon=23.15163,alt=7.8\n");
					exit(EXIT_FAILURE);
				}
				break;
			case 'o' :
				args.arg_offset_hz = 1;
				args.offset_hz = atoi(optarg);
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
		fprintf(stderr, "samplerate not specified!\n");
		exit(EXIT_FAILURE);
	}
	else {
		fprintf(stderr, "IQ samplerate: %u, stream type %s\n", args.samplerate, input_type_opts[args.inputtype]);
	}

	// check if only 1 mode is specified
	if (args.arg_const_mode && args.arg_doppler_mode) {
		fprintf(stderr, "--const (-c) and --doppler (-d) arguments cannot be used together\n");
		exit(EXIT_FAILURE);
	}

	// check which const mode parameters are missing
	if (args.arg_const_mode && !args.arg_offset_hz) {
		fprintf(stderr, "constant shift mode also needs --offset (-o) argument to know how much to shift\n");
		exit(EXIT_FAILURE);
	}

	// check which doppler mode parameters are missing
	if (args.arg_doppler_mode && !args.arg_freq_hz) {
		fprintf(stderr, "doppler mode also needs --freq (-f) parameter which specifies object transmission frequency, ");
		fprintf(stderr, "for example 'ESTCUBE 1' uses 437505000 Hz\n");
	}
	if (args.arg_doppler_mode && !args.arg_tlefile) {
		fprintf(stderr, "doppler mode also needs --tlefile (-t) parameter which specifies file with TLEs\n");
		fprintf(stderr, "such file can be downloaded from: https://celestrak.com/NORAD/elements/cubesat.txt\n");
	}
	if (args.arg_doppler_mode && !args.arg_tlename) {
		fprintf(stderr, "doppler mode also needs --tlename (-n) parameter which specifies which TLE to use from TLE file\n");
		fprintf(stderr, "for example use as --tlename -n 'ESTCUBE 1'\n");
	}
	if (args.arg_doppler_mode && (!args.arg_lat || !args.arg_lon || !args.arg_alt)) {
		fprintf(stderr, "doppler mode also needs --location (-l) parameter which specifies observer location\n");
		fprintf(stderr, "for example use as --location (-l) lat=58.64560,lon=23.15163,alt=7.8\n");
	}

	if (args.arg_doppler_mode && (!args.arg_freq_hz || !args.arg_tlefile || !args.arg_tlename || !args.arg_lat || !args.arg_lon || !args.arg_alt)) {
		fprintf(stderr, "\ndoppler mode example command:\n\tdoppler -s 1024000 -d -f 437505000 -t cubesats.txt -n 'ESTCUBE 1' --location lat=58.64560,lon=23.15163,alt=7.8 -o dopplet.out\n");
		exit(EXIT_FAILURE);
	}


	// CONST MODE
	if (args.arg_const_mode && args.arg_offset_hz) {
		fprintf(stderr, "constant shift mode with %d Hz shift\n", args.offset_hz);

		while (1) {
			// read IQ stream
			// shift baseband frequency
			// write IQ stream
			bytes_read = fread(iq_buffer, 1, INPUT_STREAM_BLOCK_SIZE, stdin);
			if (bytes_read) {
				if (args.inputtype == INPUT_TYPE_OPTION_I16) {
					shifted_data_len = dsp_shift_frequency_i16((int16_t*)iq_buffer, bytes_read / 2, (int)args.offset_hz, args.samplerate);
				}
				else if (args.inputtype == INPUT_TYPE_OPTION_F32) {
					shifted_data_len = dsp_shift_frequency_f32((float*)iq_buffer, bytes_read / 4, (int)args.offset_hz, args.samplerate);
				}
				else {
					fprintf(stderr, "ASSERT: wrong args.inputtype %u\n", args.inputtype);
					exit(EXIT_FAILURE);
				}

				fwrite(iq_buffer, 1, shifted_data_len, stdout);
				fflush(stdout);
			}

			if (feof(stdin)) {
				break;
			}
		}
	}

	// DOPPLER MODE
	if (args.arg_doppler_mode && args.arg_freq_hz && args.arg_tlefile && args.arg_tlename && args.arg_lat && args.arg_lon && args.arg_alt) {
		FILE* logfp;
		FILE* loggerfp;
		sat_t sat;
		geodetic_t observer_location;

		double doppler;
		double shift;
		time_t systime;
		int sample_count = 0;
		struct tm t;
		struct tm* timestamp = &t;
		memcpy(&t, &args.utc_time, sizeof(t));

		fprintf(stderr, "doppler correction mode\n");
		fprintf(stderr, "\tTLE file: %s\n", args.tlefile);
		fprintf(stderr, "\tTLE name: %s\n", args.tlename);
		fprintf(stderr, "\tobserver location: lat %2.4f, lon %2.4f, alt %.1f m\n", args.lat, args.lon, args.alt);

		observer_location.lat = Radians(args.lat);
		observer_location.lon = Radians(args.lon);
		observer_location.alt = args.alt / 1000.; // km
		observer_location.theta = 0.;

		predict_load_tle(args.tlefile, args.tlename, &sat);

		// arg output file
		if (args.arg_log_file) {
			fprintf(stderr, "log events to file: %s\n", args.log_file);
			logfp = fopen(args.log_file, "w");
			if (logfp == NULL) {
				fprintf(stderr, "cannot open events output file %s\n", args.log_file);
				exit(EXIT_FAILURE);
			}
		}

		if (args.arg_utc_time) {
			fprintf(stderr, "\tobservation start time: %s\n", utc_timestamp(timestamp));
		}
		else {
			timestamp = NULL; // use current system time for logging
		}

		if (args.arg_log_file) {
			loggerfp = logfp;
		}
		else {
			loggerfp = stderr;
		}

		// take current timestamp
		if (args.arg_utc_time) {
			systime = mktime(&args.utc_time);
		}
		else {
			systime = time(NULL);
		}

		while (1) {
			if (args.arg_utc_time) {
				predict_calc(&sat, &observer_location, predict_get_daynum(timestamp));
			}
			else {
				predict_calc(&sat, &observer_location, predict_get_current_daynum());
			}

			doppler = (sat.range_rate * 1000 / SPEED_OF_LIGHT_M_S) * args.freq_hz * (-1.0);

			// advance timestamp based on samples read
			if (args.arg_utc_time) {
				time_t tt = mktime(&args.utc_time);
				tt += sample_count/args.samplerate; // advance time
				timestamp = localtime(&tt);

				if (tt - systime >= 5.0) {
					systime = tt;

					fprintf(loggerfp, "\n%s: az:%6.1f, el:%6.1f, range rate:%6.3f km/s\n", utc_timestamp(timestamp), sat.az, sat.el, sat.range_rate);
					fprintf(loggerfp, "%s: %3.3f MHz doppler: %6.1f Hz\n", utc_timestamp(timestamp), args.freq_hz/1e+6, doppler);
					fflush(loggerfp);
				}
			}

			// print realtime doppler after every 1 s
			if (!args.arg_utc_time && (time(NULL) - systime) > 0.01) {
				systime = time(NULL);

				fprintf(loggerfp, "\n%s: az:%6.1f, el:%6.1f, range rate:%6.3f km/s\n", utc_timestamp(timestamp), sat.az, sat.el, sat.range_rate);
				fprintf(loggerfp, "%s: %3.3f MHz doppler: %6.1f Hz\n", utc_timestamp(timestamp), args.freq_hz/1e+6, doppler);
				fflush(loggerfp);
			}

			// check if also constant offset correction is needed
			if (args.arg_offset_hz) {
				shift = args.offset_hz + doppler;
			}
			else {
				shift = doppler;
			}

			// read IQ stream
			// shift baseband frequency by doppler frequency
			// write IQ stream
			bytes_read = fread(iq_buffer, 1, INPUT_STREAM_BLOCK_SIZE, stdin);
			if (bytes_read) {
				if (args.inputtype == INPUT_TYPE_OPTION_I16) {
					sample_count += bytes_read / 4;
					shifted_data_len = dsp_shift_frequency_i16((int16_t*)iq_buffer, bytes_read / 2, (int)shift, args.samplerate);
				}
				else if (args.inputtype == INPUT_TYPE_OPTION_F32) {
					sample_count += bytes_read / 8;
					shifted_data_len = dsp_shift_frequency_f32((float*)iq_buffer, bytes_read / 4, (int)shift, args.samplerate);
				}
				else {
					fprintf(loggerfp, "ASSERT: wrong args.inputtype %u\n", args.inputtype);
					exit(EXIT_FAILURE);
				}

				fwrite(iq_buffer, 1, shifted_data_len, stdout);
				fflush(stdout);
			}

			if (feof(stdin)) {
				break;
			}
		}

		if (args.arg_log_file) {
			fclose(logfp);
		}
	}

	return 0;
}
