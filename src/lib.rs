mod predicates;

use std::sync::Once;

static ALREADY_INIT: Once = Once::new();

pub fn exactinit() {
    ALREADY_INIT.call_once(|| {
        unsafe {
            predicates::exactinit();
        }
    })
}

pub fn orient2d(pa: &[f64;2], pb: &[f64;2], pc: &[f64;2]) -> f64 {
    unsafe {
        predicates::orient2d(pa.as_ptr(), pb.as_ptr(), pc.as_ptr())
    }
}

pub fn orient3d(pa: &[f64;3], pb: &[f64;3], pc: &[f64;3], pd: &[f64;3]) -> f64 {
    unsafe {
        predicates::orient3d(pa.as_ptr(), pb.as_ptr(), pc.as_ptr(), pd.as_ptr())
    }
}

pub fn incircle(pa: &[f64;2], pb: &[f64;2], pc: &[f64;2], pd: &[f64;2]) -> f64 {
    unsafe {
        predicates::incircle(pa.as_ptr(), pb.as_ptr(), pc.as_ptr(), pd.as_ptr())
    }
}

pub fn insphere(pa: &[f64;3], pb: &[f64;3], pc: &[f64;3], pd: &[f64;3], pe: &[f64;3]) -> f64 {
    unsafe {
        predicates::insphere(pa.as_ptr(), pb.as_ptr(), pc.as_ptr(), pd.as_ptr(), pe.as_ptr())
    }
}
