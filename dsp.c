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

#include "dsp.h"

void dsp_convert_int16_to_float(int16_t* inbuf, float* outbuf, int len) {
	int i;
	for (i=0; i<len; i++) {
		outbuf[i] = inbuf[i] / 32768.0;
	}
}

void dsp_convert_float_to_int16(float* inbuf, int16_t* outbuf, int len) {
	int i;
	for (i=0; i<len; i++) {
		outbuf[i] = inbuf[i] * 32767.0;
	}
}

void dsp_shift_frequency(int16_t* iqinput, int16_t* iqoutput, int len, int shift_freq_hz, int samplerate) {
	static int n = 0;
	int k = 0;
	float complex c_sample;
	float complex c_corrector;

	for (k=0; k<len; k+=2) {
		// convert int16_t IQ to complex float
		c_sample = iqinput[k] / 32768.0 + iqinput[k+1] / 32768.0 * I;
		c_corrector = cexpf(0.0 -2 * M_PI * (float)shift_freq_hz/(float)samplerate * n * I);
		c_sample = c_sample * c_corrector;

		// convert float back to int16_t IQ
		iqoutput[k] = crealf(c_sample) * 32767.0; // I part
		iqoutput[k+1] = cimagf(c_sample) * 32767.0; // Q part
		n++;
	}
}
