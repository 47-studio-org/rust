#![allow(
    unused,
    clippy::unused_unit,
    clippy::unnecessary_operation,
    clippy::no_effect,
    clippy::single_element_loop
)]
#![warn(clippy::semicolon_outside_block)]

macro_rules! m {
    (()) => {
        ()
    };
    (0) => {{
        0
    };};
    (1) => {{
        1;
    }};
    (2) => {{
        2;
    }};
}

fn unit_fn_block() {
    ()
}

#[rustfmt::skip]
fn main() {
    { unit_fn_block() }
    unsafe { unit_fn_block() }

    {
        unit_fn_block()
    }

    { unit_fn_block() };
    unsafe { unit_fn_block() };

    { unit_fn_block(); }
    //~^ semicolon_outside_block
    unsafe { unit_fn_block(); }
    //~^ semicolon_outside_block

    { unit_fn_block(); };
    unsafe { unit_fn_block(); };

    {
        unit_fn_block();
        unit_fn_block()
    };
    {
    //~^ semicolon_outside_block
        unit_fn_block();
        unit_fn_block();
    }
    {
        unit_fn_block();
        unit_fn_block();
    };

    { m!(()) };
    { m!(()); }
    //~^ semicolon_outside_block
    { m!(()); };
    m!(0);
    m!(1);
    m!(2);

    for _ in [()] {
        unit_fn_block();
    }
    for _ in [()] {
        unit_fn_block()
    }

    let _d = || {
        unit_fn_block();
    };
    let _d = || {
        unit_fn_block()
    };

    { unit_fn_block(); };

    unsafe {
    //~^ semicolon_outside_block
        std::arch::asm!("");
    }

    {
    //~^ semicolon_outside_block
        line!();
    }

    unit_fn_block()
}
