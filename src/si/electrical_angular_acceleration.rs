//! Electrical angular acceleration (base unit radian per second squared, s⁻²).

quantity! {
    /// Electrical angular acceleration of a rotating electric machine (base unit radian per
    /// second squared, s⁻²): the mechanical angular acceleration multiplied by the number of pole
    /// pairs.
    quantity: ElectricalAngularAcceleration; "electrical angular acceleration";
    /// Dimension of electrical angular acceleration, T⁻² (base unit radian per second squared,
    /// s⁻²).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ElectricalAngleKind;
    units {
        /// Electrical radian per second squared.
        @radian_per_second_squared: 1.0; "rad/s²", "radian per second squared",
            "radians per second squared";
    }
}
