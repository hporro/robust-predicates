use std::sync::Once;

static ALREADY_INIT: Once = Once::new();

mod predicates {
    #[link(name = "predicates")]
    extern "C" { 
        pub fn exactinit();
        pub fn orient2d(pa: &[f64;2], pb: &[f64;2], pc: &[f64;2]) -> f64;
        pub fn orient3d(pa: &[f64;3], pb: &[f64;3], pc: &[f64;3], pd: &[f64;3]) -> f64;
        pub fn incircle(pa: &[f64;2], pb: &[f64;2], pc: &[f64;2], pd: &[f64;2]) -> f64;
        pub fn insphere(pa: &[f64;3], pb: &[f64;3], pc: &[f64;3], pd: &[f64;3], pe: &[f64;3]) -> f64;
    }
}

pub fn exactinit() {
    ALREADY_INIT.call_once(|| {
        unsafe {
            predicates::exactinit();
        }
    })
}

pub fn orient2d(pa: &[f64;2], pb: &[f64;2], pc: &[f64;2]) -> f64 {
    unsafe {
        predicates::orient2d(pa, pb, pc)
    }
}

pub fn orient3d(pa: &[f64;3], pb: &[f64;3], pc: &[f64;3], pd: &[f64;3]) -> f64 {
    unsafe {
        predicates::orient3d(pa, pb, pc, pd)
    }
}

pub fn incircle(pa: &[f64;2], pb: &[f64;2], pc: &[f64;2], pd: &[f64;2]) -> f64 {
    unsafe {
        predicates::incircle(pa, pb, pc, pd)
    }
}

pub fn insphere(pa: &[f64;3], pb: &[f64;3], pc: &[f64;3], pd: &[f64;3], pe: &[f64;3]) -> f64 {
    unsafe {
        predicates::insphere(pa, pb, pc, pd, pe)
    }
}
