#![feature(allocator_api)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(slice_ptr_get)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod consts;
pub mod tasks;
pub mod thr;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;

use drone_core::heap;
use drone_nrf_map::nrf_reg_tokens;

drone_nrf91_dso::set_log! {
    periph: Uarte0S,
    pin_number: 29,
    buf_size: 64,
}

nrf_reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    index => pub Regs;

    exclude => {
        uarte0_ns_baudrate, uarte0_ns_config, uarte0_ns_enable,
        uarte0_ns_errorsrc, uarte0_ns_events_cts, uarte0_ns_events_endrx,
        uarte0_ns_events_endtx, uarte0_ns_events_error, uarte0_ns_events_ncts,
        uarte0_ns_events_rxdrdy, uarte0_ns_events_rxstarted,
        uarte0_ns_events_rxto, uarte0_ns_events_txdrdy,
        uarte0_ns_events_txstarted, uarte0_ns_events_txstopped, uarte0_ns_inten,
        uarte0_ns_intenclr, uarte0_ns_intenset, uarte0_ns_psel_cts,
        uarte0_ns_psel_rts, uarte0_ns_psel_rxd, uarte0_ns_psel_txd,
        uarte0_ns_publish_cts, uarte0_ns_publish_endrx, uarte0_ns_publish_endtx,
        uarte0_ns_publish_error, uarte0_ns_publish_ncts,
        uarte0_ns_publish_rxdrdy, uarte0_ns_publish_rxstarted,
        uarte0_ns_publish_rxto, uarte0_ns_publish_txdrdy,
        uarte0_ns_publish_txstarted, uarte0_ns_publish_txstopped,
        uarte0_ns_rxd_amount, uarte0_ns_rxd_maxcnt, uarte0_ns_rxd_ptr,
        uarte0_ns_shorts, uarte0_ns_subscribe_flushrx,
        uarte0_ns_subscribe_startrx, uarte0_ns_subscribe_starttx,
        uarte0_ns_subscribe_stoprx, uarte0_ns_subscribe_stoptx,
        uarte0_ns_tasks_flushrx, uarte0_ns_tasks_startrx,
        uarte0_ns_tasks_starttx, uarte0_ns_tasks_stoprx, uarte0_ns_tasks_stoptx,
        uarte0_ns_txd_amount, uarte0_ns_txd_maxcnt, uarte0_ns_txd_ptr,

        scb_ccr,
        mpu_type, mpu_ctrl, mpu_rnr, mpu_rbar, mpu_rasr,
    };
}

heap! {
    // Heap configuration key in `Drone.toml`.
    config => main;
    /// The main heap allocator generated from the `Drone.toml`.
    metadata => pub Heap;
    // Use this heap as the global allocator.
    global => true;
    // Uncomment the following line to enable heap tracing feature:
    // trace_port => 31;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
