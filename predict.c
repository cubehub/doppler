/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 * Copyright (c) 2015 Viljo Allik
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

#include "predict.h"
#include <sys/time.h>

int predict_load_tle(char* tle_path, char* tle_name, sat_t* sat) {
	FILE *fp;
	char tle_str[3][80];
	char* b;

	fp = fopen(tle_path, "r");

	if (fp == NULL) {
		fprintf(stderr, "Cannot open tle file %s\n", tle_path);
		return -1;
	}

	while (1) {
		b = fgets(tle_str[0], 80, fp);
		if (b == NULL) {
			fprintf(stderr, "Cannot find satellite %s in TLE file %s\n", tle_name, tle_path);
			fclose(fp);
			return -1;
		}
		else if (strncmp(b, tle_name, strlen(tle_name)) == 0 ) {
			break;
		}
	}

	b = fgets(tle_str[1], 80, fp);
	b = fgets(tle_str[2], 80, fp);
	fclose(fp);

	if (b == NULL)	{
		fprintf(stderr, "Cannot find satellite %s TLE data in file %s, unexpected EOF!\n", tle_name, tle_path);

		return -1;
	}

	if (Get_Next_Tle_Set(tle_str, &(sat->tle)) != 1) {
		fprintf(stderr, "Invalid TLE data! \n");
		return -1;
	}

	select_ephemeris(sat);

	return 0;
}

void predict_calc(sat_t* sat, geodetic_t* obs_geodetic, double t) {
	obs_set_t     obs_set;
	geodetic_t    sat_geodetic;
	double        age;
	double jul_utc;
	jul_utc = Julian_Date_of_Epoch(sat->tle.epoch); // => tsince = 0.0
	sat->jul_epoch = jul_utc;
	sat->jul_utc = t;
	sat->tsince = (sat->jul_utc - sat->jul_epoch) * xmnpda;

	// call the norad routines according to the deep-space flag
	if (sat->flags & DEEP_SPACE_EPHEM_FLAG) {
		SDP4(sat, sat->tsince);
	}
	else {
		SGP4(sat, sat->tsince);
	}

	Convert_Sat_State(&sat->pos, &sat->vel);

	// get the velocity of the satellite
	Magnitude (&sat->vel);
	sat->velo = sat->vel.w;
	Calculate_Obs(sat->jul_utc, &sat->pos, &sat->vel, obs_geodetic, &obs_set);
	Calculate_LatLonAlt(sat->jul_utc, &sat->pos, &sat_geodetic);

	while(sat_geodetic.lon < -pi) {
		sat_geodetic.lon += twopi;
	}

	while (sat_geodetic.lon > (pi)) {
		sat_geodetic.lon -= twopi;
	}

	sat->az = Degrees(obs_set.az);
	sat->el = Degrees(obs_set.el);
	sat->range = obs_set.range;
	sat->range_rate = obs_set.range_rate;
	sat->ssplat = Degrees(sat_geodetic.lat);
	sat->ssplon = Degrees(sat_geodetic.lon);
	sat->alt = sat_geodetic.alt;
	sat->ma = Degrees(sat->phase);
	sat->ma *= 256.0 / 360.0;
	sat->phase = Degrees(sat->phase);

	sat->footprint = 12756.33 * acos(xkmper / (xkmper+sat->alt));
	age = sat->jul_utc - sat->jul_epoch;
	sat->orbit = (long)floor((sat->tle.xno * xmnpda/twopi +
					age * sat->tle.bstar * ae) * age +
					sat->tle.xmo/twopi) + sat->tle.revnum - 1;
}

double predict_get_current_daynum() {
	struct tm utc;
	struct timeval tmval;
	double daynum;

	UTC_Calendar_Now(&utc);
	gettimeofday(&tmval, NULL);

	daynum = Julian_Date(&utc);
	daynum = daynum + (double)tmval.tv_usec / 8.64e+10;
	return daynum;
}
