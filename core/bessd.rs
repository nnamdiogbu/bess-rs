use clap::Parser;
use nix::unistd::*;
use libc::{umask, S_IWGRP, S_IWOTH, exit, EXIT_FAILURE, EXIT_SUCCESS};
use log::*;

use crate::debug;

use crate::opts::*;
use crate::port;

// #include <dirent.h>
// #include <dlfcn.h>
// #include <sys/file.h>
// #include <sys/resource.h>
// #include <sys/stat.h>
// #include <unistd.h>


// Utility routines for the main bess daemon.

// When Modules extend other Modules, they may reference a shared object
// that has not yet been loaded by the BESS daemon. kInheritanceLimit is
// the number of passes that will be made while loading Module shared objects,
// and thus the maximum inheritance depth of any Module.
pub const K_INHERITANCE_LIMIT: u32 = 10;

// Process command line arguments from gflags.
pub fn process_command_line_args() {
  let flag = Options::parse();
  if flag.t {
    debug::dump_types();
    unsafe {exit(EXIT_SUCCESS)};
  }

  if F.flag {
    google::LogToStderr();
  }
}

// Checks that we are running as superuser.
pub fn check_running_as_root() {
  let flag = Options::parse();
  if !flag.skip_root_check {
      let euid = geteuid();
      if (euid != 0) {
        error!("You need root privilege to run the BESS daemon");
        unsafe {exit(EXIT_FAILURE)};
      }
    }
    // Great power comes with great responsibility.
    unsafe {umask(S_IWGRP | S_IWOTH)};
}

// Write the pid value to the given file fd.  Overwrites anything present at
// that fd.  Dies if unable to overwrite the file.
pub fn write_pid_file(fd: u32, pid: u32) {}

// Read the pid value from the given file fd.  Returns true and the read pid
// value upon success.  Returns false upon failure.
pub fn read_pid_file(fd: u32) -> (bool, u32) {}

// Tries to acquire the daemon pidfile lock for the file open at the given fd.
// Dies if an error occurs when trying to acquire the lock.  Returns a pair
// <lockheld, pid> where lockheld is a bool indicating if the lock is held and
// pid is a pid_t that is non-zero if lockheld is true indicating the process
// holding the lock.
pub fn try_acquire_pid_file_lock(fd: u32) -> (bool, u32) {}

// Ensures that we are a unique instance.
// Returns the (locked) file descriptor of pidfile_path.
pub fn check_unique_instance(pidfile_path: &str) -> u32 {}

// Starts BESS as a daemon running in the background.
pub fn daemonize()-> u32 {}

// Sets BESS's resource limit.  Returns true upon success.
pub fn set_resource_limit() -> bool {}

// Load an indiviual plugin specified by path. Return true upon success.
pub fn load_plugin(path: &str) -> bool {}

// Unload a loaded plugin specified by path. Return true upon success.
pub fn unload_plugin(path: &str) -> bool {};

// Load all the .so files in the specified directory. Return true upon success.
pub fn load_plugins(directory: &str) -> bool {}

// List all imported .so files.
pub fn list_plugins() -> Vec<String> {}

// Return the current executable's own directory. For example, if the location
// of the executable is /opt/bess/core/bessd, returns /opt/bess/core/ (with the
// slash at the end).
pub fn get_current_directory() -> String {}

