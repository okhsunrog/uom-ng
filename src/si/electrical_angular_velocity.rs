//! Electrical angular velocity (base unit radian per second, s⁻¹).

quantity! {
    /// Electrical angular velocity of a rotating electric machine (base unit radian per second,
    /// s⁻¹): the mechanical angular velocity multiplied by the number of pole pairs.
    ///
    /// A distinct kind from [`AngularVelocity`](crate::si::angular_velocity::AngularVelocity).
    /// `revolution_per_minute` is the electrical rpm (eRPM).
    quantity: ElectricalAngularVelocity; "electrical angular velocity";
    /// Dimension of electrical angular velocity, T⁻¹ (base unit radian per second, s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ElectricalAngleKind;
    units {
        /// Electrical radian per second.
        @radian_per_second: 1.0_E0; "rad/s", "radian per second", "radians per second";
        /// Electrical degree per second.
        @degree_per_second: 1.745_329_251_994_329_5_E-2; "°/s", "degree per second",
            "degrees per second";
        /// Electrical revolution per second.
        @revolution_per_second: 6.283_185_307_179_586_E0; "rps", "revolution per second",
            "revolutions per second";
        /// Electrical revolution per minute (eRPM).
        @revolution_per_minute: 1.047_197_551_196_597_7_E-1; "erpm",
            "electrical revolution per minute", "electrical revolutions per minute";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::{FromPrimitive, One};
        use crate::si::electrical_angle as ea;
        use crate::si::electrical_angular_velocity as ev;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricalAngularVelocity<V> =
                (ElectricalAngle::new::<ea::radian>(V::one()) / Time::new::<t::second>(V::one()))
                    .into();
        }

        #[test]
        fn check_units() {
            Test::assert_approx_eq(
                &ElectricalAngularVelocity::new::<ev::revolution_per_minute>(
                    V::from_f64(60.0).unwrap()),
                &ElectricalAngularVelocity::new::<ev::revolution_per_second>(V::one()));
        }
    }
}
