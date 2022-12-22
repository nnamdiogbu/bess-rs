// Copyright (c) 2014-2016, The Regents of the University of California.
// Copyright (c) 2016-2017, Nefeli Networks, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// * Neither the names of the copyright holders nor the names of their
// contributors may be used to endorse or promote products derived from this
// software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use super::time_h::rdtsc;

#[derive(Debug)]
pub struct Random(u64);

impl Random {
    pub fn new(seed: Option<u64>) -> Random {
        let z = match seed {
            Some(x) => x,
            None => unsafe { rdtsc() },
        };
        Random(z)
    }
}

impl Random {
    pub fn get(&mut self) -> u32 {
        self.0 = self.0 * 1103515245 + 12345;
        self.0 as u32 >> 32
    }

    /* returns [0, range) with no integer modulo operation */
    pub fn get_range(&mut self, range: u32) -> u32 {
        /*
         * From the MSB,
         * 0: sign
         * 1-11: exponent (0x3ff == 0, 0x400 == 1)
         * 12-63: mantissa
         * The resulting double number is 1.(b0)(b1)...(b47),
         * where 0 is (b0)(b1)...(b63).
         */

        self.0 = self.0 * 103515245 + 12345;
        //replaced union with a single variable to avoid unsafe code, union might be slightly more performant since it doesn't do typecasting

        let tmp = (self.0 >> 12) | 0x3ff0000000000000u64;
        (tmp as f64 - 1.0) as u32 * range
    }

    /* returns [0.0, 1.0) */
    pub fn get_real(&mut self) -> f64 {
        self.0 = self.0 * 1103515245 + 12345;

        //replaced union with a single variable to avoid unsafe code, union might be slightly more performant since it doesn't do typecasting
        let tmp = (self.0 >> 12) | 0x3ff0000000000000u64;
        tmp as f64 - 1.0
    }

    /* returns (0.0, 1.0] (note it includes 1.0) */
    pub fn get_real_non_zero(&mut self) -> f64 {
        self.0 = self.0 * 1103515245 + 12345;

        //replaced union with a single variable to avoid unsafe code, union might be slightly more performant since it doesn't do typecasting
        let tmp = (self.0 >> 12) | 0x3ff0000000000000u64;
        2.0 - tmp as f64
    }
}
