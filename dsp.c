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

int dsp_shift_frequency_i16(int16_t* iqbuffer, int len, int shift_freq_hz, int samplerate) {
	static int n = 0;
	int k = 0;
	float complex c_sample;
	float complex c_corrector;

	for (k=0; k<len; k+=2) {
		// convert int16_t IQ to complex float
		c_sample = iqbuffer[k] / 32768.0 + iqbuffer[k+1] / 32768.0 * I;
		c_corrector = cexpf(0.0 -2 * M_PI * (float)shift_freq_hz/(float)samplerate * n * I);
		c_sample = c_sample * c_corrector;

		// convert float back to int16_t IQ
		iqbuffer[k] = crealf(c_sample) * 32767.0; // I part
		iqbuffer[k+1] = cimagf(c_sample) * 32767.0; // Q part
		n++;
	}

	// shifted data len in bytes
	return len * 2;
}

int dsp_shift_frequency_f32(float* iqbuffer, int len, int shift_freq_hz, int samplerate) {
	static int n = 0;
	int k = 0;
	float complex c_sample;
	float complex c_corrector;

	for (k=0; k<len; k+=2) {
		c_sample = iqbuffer[k] + iqbuffer[k+1] * I;
		c_corrector = cexpf(0.0 -2 * M_PI * (float)shift_freq_hz/(float)samplerate * n * I);
		c_sample = c_sample * c_corrector;

		// convert float back to int16_t IQ
		((int16_t*)iqbuffer)[k] = crealf(c_sample) * 32767.0; // I part
		((int16_t*)iqbuffer)[k+1] = cimagf(c_sample) * 32767.0; // Q part
		n++;
	}

	// shifted data len in bytes
	return len * 2;
}
