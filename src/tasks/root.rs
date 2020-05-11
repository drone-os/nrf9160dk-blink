//! The root task.

use crate::{consts::SYS_TICK_FREQ, thr, thr::ThrsInit, Regs};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_nrf_map::{
    periph::sys_tick::{periph_sys_tick, SysTickPeriph},
    reg,
};
use futures::prelude::*;

/// An error returned when a receiver has missed too many ticks.
#[derive(Debug)]
pub struct TickOverflow;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);
    let p0_dir = reg.p0_ns_dir.into_p0_s_dir();
    let p0_out = reg.p0_ns_out.into_p0_s_out();
    let sys_tick = periph_sys_tick!(reg);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    beacon(
        p0_dir.into_unsync(),
        p0_out.into_unsync(),
        sys_tick,
        thr.sys_tick,
    )
    .root_wait()
    .expect("beacon fail");

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}

async fn beacon(
    mut p0_dir: reg::p0_s::Dir<Urt>,
    mut p0_out: reg::p0_s::Out<Urt>,
    sys_tick: SysTickPeriph,
    thr_sys_tick: thr::SysTick,
) -> Result<(), TickOverflow> {
    // Set pins 2, 3, 4, 5 as outputs
    p0_dir.modify(|r| r.set_pin2().set_pin3().set_pin4().set_pin5());

    // Attach a listener that will notify us on each interrupt trigger.
    let mut tick_stream = thr_sys_tick.add_pulse_try_stream(
        // This closure will be called when a receiver no longer can store the
        // number of ticks since the last stream poll. If this happens, a
        // `TickOverflow` error will be sent over the stream as is final value.
        || Err(TickOverflow),
        // A fiber that will be called on each interrupt trigger. It sends a
        // single tick over the stream.
        fib::new_fn(|| fib::Yielded(Some(1))),
    );
    // Clear the current value of the timer.
    sys_tick.stk_val.store(|r| r.write_current(0));
    // Set the value to load into the `stk_val` register when the counter
    // reaches 0. We set it to the count of SysTick clocks per second divided by
    // 4, so the reload will be triggered each 250 ms.
    sys_tick
        .stk_load
        .store(|r| r.write_reload(SYS_TICK_FREQ / 4));
    sys_tick.stk_ctrl.store(|r| {
        r.set_tickint() // Counting down to 0 triggers the SysTick interrupt
            .set_enable() // Start the counter in a multi-shot way
    });

    // A value cycling from 0 to 7. Full cycle represents a full second.
    let mut counter = 0;
    while let Some(tick) = tick_stream.next().await {
        for _ in 0..tick?.get() {
            // Each full second print a message.
            if counter == 0 {
                println!("sec");
            }
            match counter {
                // On 0's millisecond pull pin 2 high and pin 4 low.
                0 => p0_out.modify(|r| r.set_pin2().clear_pin4()),
                // On 250's millisecond pull pin 3 high and pin 2 low.
                1 => p0_out.modify(|r| r.set_pin3().clear_pin2()),
                // On 500's millisecond pull pin 5 high and pin 3 low.
                2 => p0_out.modify(|r| r.set_pin5().clear_pin3()),
                // On 750's millisecond pull pin 4 high and pin 5 low.
                _ => p0_out.modify(|r| r.set_pin4().clear_pin5()),
            }
            counter = (counter + 1) % 4;
        }
    }

    Ok(())
}
