//! The threads.

pub use drone_cortexm::thr::{init, init_extended};
pub use drone_nrf_map::thr::*;

use drone_cortexm::thr;

thr::nvic! {
    /// Thread-safe storage.
    thread => pub Thr {};

    /// Thread-local storage.
    local => pub ThrLocal {};

    /// Vector table.
    vtable => pub Vtable;

    /// Thread token set.
    index => pub Thrs;

    /// Threads initialization token.
    init => pub ThrsInit;

    threads => {
        exceptions => {
            /// All classes of faults.
            pub hard_fault;
            /// System tick timer.
            pub sys_tick;
        };
    };
}
