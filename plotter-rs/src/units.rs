//! This module defines a system of units for plotter motion planning, using the
//! [`uom`][uom] crate.
//!
//! The motion planning computation happens in the base units (inches and
//! seconds), but the low-level move commands in the [EBB][ebb] firmware are
//! specified in terms of a motor step accumulator, with high and low resolution
//! modes affecting the steps/in factor. The EBB Interrupt Service Routine (ISR)
//! runs at at a 25 kHz rate, and thus use a 40 µs tick interval unit. The step
//! is the smallest unit of motion that the plotter can execute, but the low-
//! level move commands operate on a (u32) step accumulator which triggers a
//! step on a given tick if the accumulator exceeds 2³¹ (in which case 2³¹ is
//! subtracted from it). This system allows type-safe conversion from the motion
//! plan (base units) to EBB commands (motor step accumulator and ISR ticks).
//!
//! [uom]: https://docs.rs/uom/latest/uom
//! [ebb]: https://evil-mad.github.io/EggBot/ebb.html

pub mod length {
    quantity! {
        /// Length (base unit inch, in).
        quantity: Length; "length";
        /// Length dimension, in.
        dimension: PQ</* length */ P1, /* time */ Z0>;
        units {
            @inch: 1.0; "in", "inch", "inches";

            /// Motor step in high resolution mode (2032 steps/in)
            @step_high: 4.9213E-4; "step (high)",
            "motor step (high resolution)",
            "motor steps (high resolution)";
            /// Motor step in low resolution mode (1016 steps/in)
            @step_low: 9.8425E-4; "step (low)",
            "motor step (low resolution)",
            "motor steps (low resolution)";

            /// Step accumulator in high resolution mode (2032 steps/in · 2³¹ accum/step)
            @accum_high: 2.29164019E-13; "accum (high)",
            "step accumulator (high resolution)",
            "step accumulator (high resolution)";
            /// Step accumulator in low resolution mode (1016 steps/in · 2³¹ accum/step)
            @accum_low: 4.58328039E-13; "accum (low)",
            "step accumulator (low resolution)",
            "step accumulator (low resolution)";
        }
    }
}

pub mod time {
    quantity! {
        /// Time (base unit second, s).
        quantity: Time; "time";
        /// Time dimension, s.
        dimension: PQ</* length */ Z0, /* time */ P1>;
        units {
            @second: 1.0; "s", "second", "seconds";

            /// EBB ISR runs at 25 kHz => 40 µs
            @tick: 40.0E-6; "tick", "ISR tick", "ISR ticks";
        }
    }
}

pub mod velocity {
    quantity! {
        /// Velocity (base unit inch per second, in · s⁻¹).
        quantity: Velocity; "velocity";
        /// Dimension of velocity, LT⁻¹ (base unit inch per second, in · s⁻¹).
        dimension: PQ</* length */ P1, /* time */ N1>;
        units {
            @inch_per_second: 1.0; "in/s", "inch per second", "inches per second";

            /// Step accumulator (high resolution) per tick (2032 steps/in · 2³¹ accum/step ÷ 25000 Hz)
            @accum_high_per_tick: 174547470.90944; "accum (high)/tick",
            "step accumulator (high resolution) per tick",
            "step accumulator (high resolution) per tick";
            /// Step accumulator (high resolution) per tick (1016 steps/in · 2³¹ accum/step ÷ 25000 Hz)
            @accum_low_per_tick: 87273735.45472; "accum (low)/tick",
            "step accumulator (low resolution) per tick",
            "step accumulator (low resolution) per tick";
        }
    }
}

pub mod acceleration {
    quantity! {
        /// Acceleration (base unit inch per second squared, in · s⁻²).
        quantity: Acceleration; "acceleration";
        /// Dimension of acceleration, LT⁻² (base unit inch per second squared, in · s⁻²).
        dimension: PQ</* length */ P1, /* time */ N2>;
        units {
            @inch_per_second_squared: 1.0; "in/s²", "inch per second squared", "inches per second squared";

            /// Step accumulator (high resolution) per tick squared (2032 steps/in · 2³¹ accum/step ÷ (25000 Hz)²)
            @accum_high_per_tick_squared: 6981.89883638; "accum (high)/tick²",
            "step accumulator (high resolution) per tick squared",
            "step accumulator (high resolution) per tick squared";
            /// Step accumulator (low resolution) per tick squared (1016 steps/in · 2³¹ accum/step ÷ (25000 Hz)²)
            @accum_low_per_tick_squared: 3490.94941819; "accum (low)/tick²",
            "step accumulator (low resolution) per tick squared",
            "step accumulator (low resolution) per tick squared";
        }
    }
}

pub mod jerk {
    quantity! {
        /// Jerk (base unit inch per second cubed, in · s⁻³).
        quantity: Jerk; "jerk";
        /// Dimension of jerk, LT⁻³ (base unit inch per second cubed, in · s⁻³).
        dimension: PQ</* length */ P1, /* time */ N3>;
        units {
            @inch_per_second_cubed: 1.0; "in/s³", "inch per second cubed", "inches per second cubed";

            /// Step accumulator (high resolution) per tick cubed (2032 steps/in · 2³¹ accum/step ÷ (25000 Hz)³)
            @accum_high_per_tick_cubed: 0.27927595; "accum (high)/tick³",
            "step accumulator (high resolution) per tick cubed",
            "step accumulator (high resolution) per tick cubed";
            /// Step accumulator (low resolution) per tick cubed (1016 steps/in · 2³¹ accum/step ÷ (25000 Hz)³)
            @accum_low_per_tick_cubed: 0.13963798; "accum (low)/tick³",
            "step accumulator (low resolution) per tick cubed",
            "step accumulator (low resolution) per tick cubed";
        }
    }
}

system! {
    quantities: PQ {
        length: inch, L;
        time: second, T;
    }

    units: PU {
        mod length::Length,
        mod time::Time,
        mod velocity::Velocity,
        mod acceleration::Acceleration,
        mod jerk::Jerk,
    }
}

PQ!(crate::units, f64);

mod i32 {
    PQ!(crate::units, i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_inches() {
        let one_inch = Length::new::<length::inch>(1.0);
        let two_inches = Length::new::<length::inch>(2.0);

        assert_eq!(one_inch + two_inches, Length::new::<length::inch>(3.0));

        let one_inch = i32::Length::new::<length::inch>(1);
        let two_inches = i32::Length::new::<length::inch>(2);

        assert_eq!(one_inch + two_inches, i32::Length::new::<length::inch>(3));
    }
}
