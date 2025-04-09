//! Defines the is_null utility.

//::math

/// Check if value is null or is nan.
pub fn is_null(value: Any) -> bool {
    fn is_none() -> bool {
        return value.is_none()
    }

    fn is_nan() -> bool {
        return isinstance(value, float) and math.isnan(value)
    }

    is_none() || is_nan()
}
