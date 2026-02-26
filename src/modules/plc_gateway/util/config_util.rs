pub fn validate_frame(frame: &str) -> Result<(), validator::ValidationError> {
    if frame == "1E" || frame == "3E" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("frame must be 1E or 3E"))
    }
}

pub fn validate_mode(mode: &str) -> Result<(), validator::ValidationError> {
    if mode == "ascii" || mode == "binary" {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "mode must be ascii or binary",
        ))
    }
}