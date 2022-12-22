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
use libc::{clock_gettime, timespec, CLOCK_THREAD_CPUTIME_ID};
use std::arch::x86_64::_rdtsc;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

extern "C" {
    static tsc_hz: f64;
}

pub unsafe fn rdtsc() -> u64 {
    _rdtsc()
}

unsafe fn tsc_to_ns(cycles: f64) -> u64 {
    (cycles * 1000000000.0 / tsc_hz) as u64
}

unsafe fn tsc_to_us(cycles: f64) -> u64 {
    (cycles * 1000000.0 / tsc_hz) as u64
}

/* Return current Duration since the Epoch.
 * This is consistent with Python's time.time() */
pub fn get_epoch_time() -> Result<Duration, SystemTimeError> {
    SystemTime::now().duration_since(UNIX_EPOCH)
}

/* CPU time spent by the current thread.
 * Use it only relatively. */
pub fn get_cpu_time() -> Result<Duration, SystemTimeError> {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    if unsafe { clock_gettime(CLOCK_THREAD_CPUTIME_ID, &mut ts) } == 0 {
        return Ok(Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32));
    }
    get_epoch_time()
}
