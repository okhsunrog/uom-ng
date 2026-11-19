//! Electrical angle (dimensionless quantity).

quantity! {
    /// Electrical angle of a rotating electric machine (dimensionless quantity): the
    /// mechanical angle multiplied by the number of pole pairs.
    ///
    /// A distinct kind from [`Angle`](crate::si::angle::Angle), so electrical and mechanical
    /// angles cannot be added or assigned to each other.
    quantity: ElectricalAngle; "electrical angle";
    /// Dimension of electrical angle, 1 (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ElectricalAngleKind;
    units {
        /// Electrical radian.
        @radian: 1.0_E0; "rad", "radian", "radians";
        /// One electrical revolution, i.e. one pole pair.
        @revolution: 6.283_185_307_179_586_E0; "r", "revolution", "revolutions";
        /// Electrical degree.
        @degree: 1.745_329_251_994_329_5_E-2; "°", "degree", "degrees";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::lib::f64::consts::PI;
        use crate::num::{FromPrimitive, One};
        use crate::si::electrical_angle as ea;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_units() {
            Test::assert_eq(&ElectricalAngle::new::<ea::radian>(V::from_f64(2.0 * PI).unwrap()),
                &ElectricalAngle::new::<ea::revolution>(V::one()));
            Test::assert_eq(&ElectricalAngle::new::<ea::degree>(V::from_f64(360.0).unwrap()),
                &ElectricalAngle::new::<ea::revolution>(V::one()));
        }
    }
}
