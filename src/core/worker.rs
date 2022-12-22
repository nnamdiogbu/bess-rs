// #include <glog/logging.h>

// #include <cstdint>
// #include <string>
// #include <thread>
// #include <type_traits>

// #include "gate.h"
// #include "traffic_class.h"
// #include "utils/common.h"
// #include "utils/random.h"

use crate::core::scheduler::Scheduler;
use crate::core::utils::random_h::Random;
use crate::packet_pool::PacketPool;
use crate::traffic_class::TrafficClass;
// class Task;

const MAX_GATES: u32 = 8192;

// /*  TODO: worker threads doesn't necessarily be pinned to 1 core
//  *
//  *  n: kMaxWorkers
//  *
//  *  Role              DPDK lcore ID      Hardware core(s)
//  *  --------------------------------------------------------
//  *  worker 0                      0      1 specified core
//  *  worker 1                      1      1 specified core
//  *  ...
//  *  worker n-1                  n-1      1 specified core
//  *  master          RTE_MAX_LCORE-1      all other cores that are allowed
//  */
pub const K_MAX_WORKERS: i32 = 64;
pub const K_ANY_WORKER: i32 = -1; // unspecified worker ID

#[derive(PartialEq)]
pub enum WorkerStatus {
    WorkerPausing, // transient state for blocking or quitting
    WorkerPaused,
    WorkerRunning,
    WorkerFinished,
}
pub struct WorkerPauser {
    workers_paused_: std::collections::LinkedList<i32>,
}

pub struct Worker<'a> {
    status_: WorkerStatus,
    wid_: i32,  // always [0, kMaxWorkers - 1]
    core_: i32, // TODO: should be cpuset_t
    socket_: i32,
    fd_event_: i32,
    packet_pool_: PacketPool,
    scheduler_: Scheduler,

    // bess::PacketPool *packet_pool_;
    // packet_pool: *PacketPool,
    // scheduler: *Scheduler,

    // bess::Scheduler *scheduler_;
    silent_drops_: u64, // packets that have been sent to a deadend
    current_tsc_: u64,
    current_ns_: u64,
    rand_: &'a Random,
}

impl Worker<'_> {
    //  /* ----------------------------------------------------------------------
    //  * functions below are invoked by non-worker threads (the master)
    //  * ---------------------------------------------------------------------- */
    //  void SetNonWorker();
    pub fn set_non_worker() {}

    //  /* ----------------------------------------------------------------------
    //   * functions below are invoked by worker threads
    //   * ---------------------------------------------------------------------- */
    #[inline(always)]
    pub fn is_paused_requested(&self) -> bool {
        self.status_ == WorkerStatus::WorkerPausing
    }
    //  /* Block myself. Return nonzero if the worker needs to die */
    pub fn block_worker() -> u32 {
        0
    }

    //  /* The entry point of worker threads */
    pub fn run() {}

    pub fn status(&self) -> &WorkerStatus {
        &self.status_
    }

    pub fn set_status(&mut self, status: WorkerStatus) {
        self.status_ = status;
    }

    pub fn wid(&self) -> i32 {
        self.wid_
    }

    pub fn core(&self) -> i32 {
        self.core_
    }

    pub fn socket(&self) -> i32 {
        self.socket_
    }

    pub fn fd_event(&self) -> i32 {
        self.fd_event_
    }

    pub fn packet_pool(&self) -> &PacketPool {
        &self.packet_pool_
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler_
    }

    pub fn silent_drops(&self) -> u64 {
        self.silent_drops_
    }

    pub fn set_silent_drops(&mut self, drops: u64) {
        self.silent_drops_ = drops
    }

    pub fn incr_silent_drops(&mut self, drops: u64) {
        self.silent_drops_ += drops
    }

    pub const fn current_tsc(&self) -> u64 {
        self.current_tsc_
    }

    pub fn set_current_tsc(&mut self, tsc: u64) {
        self.current_tsc_ = tsc;
    }

    pub const fn current_ns(&self) -> u64 {
        self.current_ns_
    }

    pub fn set_current_ns(&mut self, ns: u64) {
        self.current_tsc_ = ns;
    }

    pub fn rand(&self) -> &Random {
        self.rand_
    }
}

// impl WorkerPauser {
//constructor, dont know how to implement
//     WorkerPauser();
// }

// NOTE: Do not use "thread_local" here. It requires a function call every time
// it is accessed. Use __thread instead, which incurs minimal runtime overhead.

// extern __thread Worker current_worker; replicate this in rust

// #if __GNUC__ >= 5
// static_assert(std::is_trivially_constructible<Worker>::value,
//               "not trivially constructible");
// static_assert(std::is_trivially_destructible<Worker>::value,
//               "not trivially destructible");
// #endif
//Don't know if i should implement this

// extern "C" {
//     static NUM_WORKERS: u32;
//     worker_threads: thr
// }
pub fn is_worker_active(wid: i64) -> bool {
    true
}

/* ------------------------------------------------------------------------
 * functions below are invoked by non-worker threads (the master)
 * ------------------------------------------------------------------------ */
