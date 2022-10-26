#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[no_mangle]
pub static mut splitter: libc::c_double = 0.;
#[no_mangle]
pub static mut epsilon: libc::c_double = 0.;
#[no_mangle]
pub static mut resulterrbound: libc::c_double = 0.;
#[no_mangle]
pub static mut ccwerrboundA: libc::c_double = 0.;
#[no_mangle]
pub static mut ccwerrboundB: libc::c_double = 0.;
#[no_mangle]
pub static mut ccwerrboundC: libc::c_double = 0.;
#[no_mangle]
pub static mut o3derrboundA: libc::c_double = 0.;
#[no_mangle]
pub static mut o3derrboundB: libc::c_double = 0.;
#[no_mangle]
pub static mut o3derrboundC: libc::c_double = 0.;
#[no_mangle]
pub static mut iccerrboundA: libc::c_double = 0.;
#[no_mangle]
pub static mut iccerrboundB: libc::c_double = 0.;
#[no_mangle]
pub static mut iccerrboundC: libc::c_double = 0.;
#[no_mangle]
pub static mut isperrboundA: libc::c_double = 0.;
#[no_mangle]
pub static mut isperrboundB: libc::c_double = 0.;
#[no_mangle]
pub static mut isperrboundC: libc::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn exactinit() {
    let mut half: libc::c_double = 0.;
    let mut check: libc::c_double = 0.;
    let mut lastcheck: libc::c_double = 0.;
    let mut every_other: libc::c_int = 0;
    every_other = 1 as libc::c_int;
    half = 0.5f64;
    epsilon = 1.0f64;
    splitter = 1.0f64;
    check = 1.0f64;
    loop {
        lastcheck = check;
        epsilon *= half;
        if every_other != 0 {
            splitter *= 2.0f64;
        }
        every_other = (every_other == 0) as libc::c_int;
        check = 1.0f64 + epsilon;
        if !(check != 1.0f64 && check != lastcheck) {
            break;
        }
    }
    splitter += 1.0f64;
    resulterrbound = (3.0f64 + 8.0f64 * epsilon) * epsilon;
    ccwerrboundA = (3.0f64 + 16.0f64 * epsilon) * epsilon;
    ccwerrboundB = (2.0f64 + 12.0f64 * epsilon) * epsilon;
    ccwerrboundC = (9.0f64 + 64.0f64 * epsilon) * epsilon * epsilon;
    o3derrboundA = (7.0f64 + 56.0f64 * epsilon) * epsilon;
    o3derrboundB = (3.0f64 + 28.0f64 * epsilon) * epsilon;
    o3derrboundC = (26.0f64 + 288.0f64 * epsilon) * epsilon * epsilon;
    iccerrboundA = (10.0f64 + 96.0f64 * epsilon) * epsilon;
    iccerrboundB = (4.0f64 + 48.0f64 * epsilon) * epsilon;
    iccerrboundC = (44.0f64 + 576.0f64 * epsilon) * epsilon * epsilon;
    isperrboundA = (16.0f64 + 224.0f64 * epsilon) * epsilon;
    isperrboundB = (5.0f64 + 72.0f64 * epsilon) * epsilon;
    isperrboundC = (71.0f64 + 1408.0f64 * epsilon) * epsilon * epsilon;
}
#[no_mangle]
pub unsafe extern "C" fn grow_expansion(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut b: libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    Q = b;
    eindex = 0 as libc::c_int;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = enow - bvirt;
        around = Q - avirt;
        *h.offset(eindex as isize) = around + bround;
        Q = Qnew;
        eindex += 1;
    }
    *h.offset(eindex as isize) = Q;
    return eindex + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn grow_expansion_zeroelim(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut b: libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    hindex = 0 as libc::c_int;
    Q = b;
    eindex = 0 as libc::c_int;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = enow - bvirt;
        around = Q - avirt;
        hh = around + bround;
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh0 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh0 as isize) = hh;
        }
        eindex += 1;
    }
    if Q != 0.0f64 || hindex == 0 as libc::c_int {
        let fresh1 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh1 as isize) = Q;
    }
    return hindex;
}
#[no_mangle]
pub unsafe extern "C" fn expansion_sum(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut hlast: libc::c_int = 0;
    let mut hnow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    Q = *f.offset(0 as libc::c_int as isize);
    hindex = 0 as libc::c_int;
    while hindex < elen {
        hnow = *e.offset(hindex as isize);
        Qnew = Q + hnow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = hnow - bvirt;
        around = Q - avirt;
        *h.offset(hindex as isize) = around + bround;
        Q = Qnew;
        hindex += 1;
    }
    *h.offset(hindex as isize) = Q;
    hlast = hindex;
    findex = 1 as libc::c_int;
    while findex < flen {
        Q = *f.offset(findex as isize);
        hindex = findex;
        while hindex <= hlast {
            hnow = *h.offset(hindex as isize);
            Qnew = Q + hnow;
            bvirt = Qnew - Q;
            avirt = Qnew - bvirt;
            bround = hnow - bvirt;
            around = Q - avirt;
            *h.offset(hindex as isize) = around + bround;
            Q = Qnew;
            hindex += 1;
        }
        hlast += 1;
        *h.offset(hlast as isize) = Q;
        findex += 1;
    }
    return hlast + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn expansion_sum_zeroelim1(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut index: libc::c_int = 0;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut hlast: libc::c_int = 0;
    let mut hnow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    Q = *f.offset(0 as libc::c_int as isize);
    hindex = 0 as libc::c_int;
    while hindex < elen {
        hnow = *e.offset(hindex as isize);
        Qnew = Q + hnow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = hnow - bvirt;
        around = Q - avirt;
        *h.offset(hindex as isize) = around + bround;
        Q = Qnew;
        hindex += 1;
    }
    *h.offset(hindex as isize) = Q;
    hlast = hindex;
    findex = 1 as libc::c_int;
    while findex < flen {
        Q = *f.offset(findex as isize);
        hindex = findex;
        while hindex <= hlast {
            hnow = *h.offset(hindex as isize);
            Qnew = Q + hnow;
            bvirt = Qnew - Q;
            avirt = Qnew - bvirt;
            bround = hnow - bvirt;
            around = Q - avirt;
            *h.offset(hindex as isize) = around + bround;
            Q = Qnew;
            hindex += 1;
        }
        hlast += 1;
        *h.offset(hlast as isize) = Q;
        findex += 1;
    }
    hindex = -(1 as libc::c_int);
    index = 0 as libc::c_int;
    while index <= hlast {
        hnow = *h.offset(index as isize);
        if hnow != 0.0f64 {
            hindex += 1;
            *h.offset(hindex as isize) = hnow;
        }
        index += 1;
    }
    if hindex == -(1 as libc::c_int) {
        return 1 as libc::c_int
    } else {
        return hindex + 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn expansion_sum_zeroelim2(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut hlast: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    hindex = 0 as libc::c_int;
    Q = *f.offset(0 as libc::c_int as isize);
    eindex = 0 as libc::c_int;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = enow - bvirt;
        around = Q - avirt;
        hh = around + bround;
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh2 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh2 as isize) = hh;
        }
        eindex += 1;
    }
    *h.offset(hindex as isize) = Q;
    hlast = hindex;
    findex = 1 as libc::c_int;
    while findex < flen {
        hindex = 0 as libc::c_int;
        Q = *f.offset(findex as isize);
        eindex = 0 as libc::c_int;
        while eindex <= hlast {
            enow = *h.offset(eindex as isize);
            Qnew = Q + enow;
            bvirt = Qnew - Q;
            avirt = Qnew - bvirt;
            bround = enow - bvirt;
            around = Q - avirt;
            hh = around + bround;
            Q = Qnew;
            if hh != 0 as libc::c_int as libc::c_double {
                let fresh3 = hindex;
                hindex = hindex + 1;
                *h.offset(fresh3 as isize) = hh;
            }
            eindex += 1;
        }
        *h.offset(hindex as isize) = Q;
        hlast = hindex;
        findex += 1;
    }
    return hlast + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fast_expansion_sum(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut fnow: libc::c_double = 0.;
    enow = *e.offset(0 as libc::c_int as isize);
    fnow = *f.offset(0 as libc::c_int as isize);
    findex = 0 as libc::c_int;
    eindex = findex;
    if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
        Q = enow;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        Q = fnow;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    hindex = 0 as libc::c_int;
    if eindex < elen && findex < flen {
        if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
            Qnew = enow + Q;
            bvirt = Qnew - enow;
            *h.offset(0 as libc::c_int as isize) = Q - bvirt;
            eindex += 1;
            enow = *e.offset(eindex as isize);
        } else {
            Qnew = fnow + Q;
            bvirt = Qnew - fnow;
            *h.offset(0 as libc::c_int as isize) = Q - bvirt;
            findex += 1;
            fnow = *f.offset(findex as isize);
        }
        Q = Qnew;
        hindex = 1 as libc::c_int;
        while eindex < elen && findex < flen {
            if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
                Qnew = Q + enow;
                bvirt = Qnew - Q;
                avirt = Qnew - bvirt;
                bround = enow - bvirt;
                around = Q - avirt;
                *h.offset(hindex as isize) = around + bround;
                eindex += 1;
                enow = *e.offset(eindex as isize);
            } else {
                Qnew = Q + fnow;
                bvirt = Qnew - Q;
                avirt = Qnew - bvirt;
                bround = fnow - bvirt;
                around = Q - avirt;
                *h.offset(hindex as isize) = around + bround;
                findex += 1;
                fnow = *f.offset(findex as isize);
            }
            Q = Qnew;
            hindex += 1;
        }
    }
    while eindex < elen {
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = enow - bvirt;
        around = Q - avirt;
        *h.offset(hindex as isize) = around + bround;
        eindex += 1;
        enow = *e.offset(eindex as isize);
        Q = Qnew;
        hindex += 1;
    }
    while findex < flen {
        Qnew = Q + fnow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = fnow - bvirt;
        around = Q - avirt;
        *h.offset(hindex as isize) = around + bround;
        findex += 1;
        fnow = *f.offset(findex as isize);
        Q = Qnew;
        hindex += 1;
    }
    *h.offset(hindex as isize) = Q;
    return hindex + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fast_expansion_sum_zeroelim(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut fnow: libc::c_double = 0.;
    enow = *e.offset(0 as libc::c_int as isize);
    fnow = *f.offset(0 as libc::c_int as isize);
    findex = 0 as libc::c_int;
    eindex = findex;
    if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
        Q = enow;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        Q = fnow;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    hindex = 0 as libc::c_int;
    if eindex < elen && findex < flen {
        if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
            Qnew = enow + Q;
            bvirt = Qnew - enow;
            hh = Q - bvirt;
            eindex += 1;
            enow = *e.offset(eindex as isize);
        } else {
            Qnew = fnow + Q;
            bvirt = Qnew - fnow;
            hh = Q - bvirt;
            findex += 1;
            fnow = *f.offset(findex as isize);
        }
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh4 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh4 as isize) = hh;
        }
        while eindex < elen && findex < flen {
            if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
                Qnew = Q + enow;
                bvirt = Qnew - Q;
                avirt = Qnew - bvirt;
                bround = enow - bvirt;
                around = Q - avirt;
                hh = around + bround;
                eindex += 1;
                enow = *e.offset(eindex as isize);
            } else {
                Qnew = Q + fnow;
                bvirt = Qnew - Q;
                avirt = Qnew - bvirt;
                bround = fnow - bvirt;
                around = Q - avirt;
                hh = around + bround;
                findex += 1;
                fnow = *f.offset(findex as isize);
            }
            Q = Qnew;
            if hh != 0.0f64 {
                let fresh5 = hindex;
                hindex = hindex + 1;
                *h.offset(fresh5 as isize) = hh;
            }
        }
    }
    while eindex < elen {
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = enow - bvirt;
        around = Q - avirt;
        hh = around + bround;
        eindex += 1;
        enow = *e.offset(eindex as isize);
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh6 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh6 as isize) = hh;
        }
    }
    while findex < flen {
        Qnew = Q + fnow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = fnow - bvirt;
        around = Q - avirt;
        hh = around + bround;
        findex += 1;
        fnow = *f.offset(findex as isize);
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh7 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh7 as isize) = hh;
        }
    }
    if Q != 0.0f64 || hindex == 0 as libc::c_int {
        let fresh8 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh8 as isize) = Q;
    }
    return hindex;
}
#[no_mangle]
pub unsafe extern "C" fn linear_expansion_sum(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut R: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut fnow: libc::c_double = 0.;
    let mut g0: libc::c_double = 0.;
    enow = *e.offset(0 as libc::c_int as isize);
    fnow = *f.offset(0 as libc::c_int as isize);
    findex = 0 as libc::c_int;
    eindex = findex;
    if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
        g0 = enow;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        g0 = fnow;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    if eindex < elen
        && (findex >= flen
            || (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int)
    {
        Qnew = enow + g0;
        bvirt = Qnew - enow;
        q = g0 - bvirt;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        Qnew = fnow + g0;
        bvirt = Qnew - fnow;
        q = g0 - bvirt;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    Q = Qnew;
    hindex = 0 as libc::c_int;
    while hindex < elen + flen - 2 as libc::c_int {
        if eindex < elen
            && (findex >= flen
                || (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int)
        {
            R = enow + q;
            bvirt = R - enow;
            *h.offset(hindex as isize) = q - bvirt;
            eindex += 1;
            enow = *e.offset(eindex as isize);
        } else {
            R = fnow + q;
            bvirt = R - fnow;
            *h.offset(hindex as isize) = q - bvirt;
            findex += 1;
            fnow = *f.offset(findex as isize);
        }
        Qnew = Q + R;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = R - bvirt;
        around = Q - avirt;
        q = around + bround;
        Q = Qnew;
        hindex += 1;
    }
    *h.offset(hindex as isize) = q;
    *h.offset((hindex + 1 as libc::c_int) as isize) = Q;
    return hindex + 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn linear_expansion_sum_zeroelim(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut flen: libc::c_int,
    mut f: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut R: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut findex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut fnow: libc::c_double = 0.;
    let mut g0: libc::c_double = 0.;
    enow = *e.offset(0 as libc::c_int as isize);
    fnow = *f.offset(0 as libc::c_int as isize);
    findex = 0 as libc::c_int;
    eindex = findex;
    hindex = 0 as libc::c_int;
    if (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int {
        g0 = enow;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        g0 = fnow;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    if eindex < elen
        && (findex >= flen
            || (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int)
    {
        Qnew = enow + g0;
        bvirt = Qnew - enow;
        q = g0 - bvirt;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        Qnew = fnow + g0;
        bvirt = Qnew - fnow;
        q = g0 - bvirt;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    Q = Qnew;
    count = 2 as libc::c_int;
    while count < elen + flen {
        if eindex < elen
            && (findex >= flen
                || (fnow > enow) as libc::c_int == (fnow > -enow) as libc::c_int)
        {
            R = enow + q;
            bvirt = R - enow;
            hh = q - bvirt;
            eindex += 1;
            enow = *e.offset(eindex as isize);
        } else {
            R = fnow + q;
            bvirt = R - fnow;
            hh = q - bvirt;
            findex += 1;
            fnow = *f.offset(findex as isize);
        }
        Qnew = Q + R;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = R - bvirt;
        around = Q - avirt;
        q = around + bround;
        Q = Qnew;
        if hh != 0 as libc::c_int as libc::c_double {
            let fresh9 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh9 as isize) = hh;
        }
        count += 1;
    }
    if q != 0 as libc::c_int as libc::c_double {
        let fresh10 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh10 as isize) = q;
    }
    if Q != 0.0f64 || hindex == 0 as libc::c_int {
        let fresh11 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh11 as isize) = Q;
    }
    return hindex;
}
#[no_mangle]
pub unsafe extern "C" fn scale_expansion(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut b: libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut product1: libc::c_double = 0.;
    let mut product0: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    c = splitter * b;
    abig = c - b;
    bhi = c - abig;
    blo = b - bhi;
    Q = *e.offset(0 as libc::c_int as isize) * b;
    c = splitter * *e.offset(0 as libc::c_int as isize);
    abig = c - *e.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *e.offset(0 as libc::c_int as isize) - ahi;
    err1 = Q - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    *h.offset(0 as libc::c_int as isize) = alo * blo - err3;
    hindex = 1 as libc::c_int;
    eindex = 1 as libc::c_int;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        product1 = enow * b;
        c = splitter * enow;
        abig = c - enow;
        ahi = c - abig;
        alo = enow - ahi;
        err1 = product1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        product0 = alo * blo - err3;
        sum = Q + product0;
        bvirt = sum - Q;
        avirt = sum - bvirt;
        bround = product0 - bvirt;
        around = Q - avirt;
        *h.offset(hindex as isize) = around + bround;
        hindex += 1;
        Q = product1 + sum;
        bvirt = Q - product1;
        avirt = Q - bvirt;
        bround = sum - bvirt;
        around = product1 - avirt;
        *h.offset(hindex as isize) = around + bround;
        hindex += 1;
        eindex += 1;
    }
    *h.offset(hindex as isize) = Q;
    return elen + elen;
}
#[no_mangle]
pub unsafe extern "C" fn scale_expansion_zeroelim(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut b: libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut product1: libc::c_double = 0.;
    let mut product0: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut enow: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    c = splitter * b;
    abig = c - b;
    bhi = c - abig;
    blo = b - bhi;
    Q = *e.offset(0 as libc::c_int as isize) * b;
    c = splitter * *e.offset(0 as libc::c_int as isize);
    abig = c - *e.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *e.offset(0 as libc::c_int as isize) - ahi;
    err1 = Q - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    hh = alo * blo - err3;
    hindex = 0 as libc::c_int;
    if hh != 0 as libc::c_int as libc::c_double {
        let fresh12 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh12 as isize) = hh;
    }
    eindex = 1 as libc::c_int;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        product1 = enow * b;
        c = splitter * enow;
        abig = c - enow;
        ahi = c - abig;
        alo = enow - ahi;
        err1 = product1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        product0 = alo * blo - err3;
        sum = Q + product0;
        bvirt = sum - Q;
        avirt = sum - bvirt;
        bround = product0 - bvirt;
        around = Q - avirt;
        hh = around + bround;
        if hh != 0 as libc::c_int as libc::c_double {
            let fresh13 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh13 as isize) = hh;
        }
        Q = product1 + sum;
        bvirt = Q - product1;
        hh = sum - bvirt;
        if hh != 0 as libc::c_int as libc::c_double {
            let fresh14 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh14 as isize) = hh;
        }
        eindex += 1;
    }
    if Q != 0.0f64 || hindex == 0 as libc::c_int {
        let fresh15 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh15 as isize) = Q;
    }
    return hindex;
}
#[no_mangle]
pub unsafe extern "C" fn compress(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut Q: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut Qnew: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    let mut hindex: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut enow: libc::c_double = 0.;
    let mut hnow: libc::c_double = 0.;
    let mut top: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    bottom = elen - 1 as libc::c_int;
    Q = *e.offset(bottom as isize);
    eindex = elen - 2 as libc::c_int;
    while eindex >= 0 as libc::c_int {
        enow = *e.offset(eindex as isize);
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        q = enow - bvirt;
        if q != 0 as libc::c_int as libc::c_double {
            let fresh16 = bottom;
            bottom = bottom - 1;
            *h.offset(fresh16 as isize) = Qnew;
            Q = q;
        } else {
            Q = Qnew;
        }
        eindex -= 1;
    }
    top = 0 as libc::c_int;
    hindex = bottom + 1 as libc::c_int;
    while hindex < elen {
        hnow = *h.offset(hindex as isize);
        Qnew = hnow + Q;
        bvirt = Qnew - hnow;
        q = Q - bvirt;
        if q != 0 as libc::c_int as libc::c_double {
            let fresh17 = top;
            top = top + 1;
            *h.offset(fresh17 as isize) = q;
        }
        Q = Qnew;
        hindex += 1;
    }
    *h.offset(top as isize) = Q;
    return top + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn estimate(
    mut elen: libc::c_int,
    mut e: *mut libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut eindex: libc::c_int = 0;
    Q = *e.offset(0 as libc::c_int as isize);
    eindex = 1 as libc::c_int;
    while eindex < elen {
        Q += *e.offset(eindex as isize);
        eindex += 1;
    }
    return Q;
}
#[no_mangle]
pub unsafe extern "C" fn orient2dfast(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
) -> libc::c_double {
    let mut acx: libc::c_double = 0.;
    let mut bcx: libc::c_double = 0.;
    let mut acy: libc::c_double = 0.;
    let mut bcy: libc::c_double = 0.;
    acx = *pa.offset(0 as libc::c_int as isize) - *pc.offset(0 as libc::c_int as isize);
    bcx = *pb.offset(0 as libc::c_int as isize) - *pc.offset(0 as libc::c_int as isize);
    acy = *pa.offset(1 as libc::c_int as isize) - *pc.offset(1 as libc::c_int as isize);
    bcy = *pb.offset(1 as libc::c_int as isize) - *pc.offset(1 as libc::c_int as isize);
    return acx * bcy - acy * bcx;
}
#[no_mangle]
pub unsafe extern "C" fn orient2dexact(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
) -> libc::c_double {
    let mut axby1: libc::c_double = 0.;
    let mut axcy1: libc::c_double = 0.;
    let mut bxcy1: libc::c_double = 0.;
    let mut bxay1: libc::c_double = 0.;
    let mut cxay1: libc::c_double = 0.;
    let mut cxby1: libc::c_double = 0.;
    let mut axby0: libc::c_double = 0.;
    let mut axcy0: libc::c_double = 0.;
    let mut bxcy0: libc::c_double = 0.;
    let mut bxay0: libc::c_double = 0.;
    let mut cxay0: libc::c_double = 0.;
    let mut cxby0: libc::c_double = 0.;
    let mut aterms: [libc::c_double; 4] = [0.; 4];
    let mut bterms: [libc::c_double; 4] = [0.; 4];
    let mut cterms: [libc::c_double; 4] = [0.; 4];
    let mut aterms3: libc::c_double = 0.;
    let mut bterms3: libc::c_double = 0.;
    let mut cterms3: libc::c_double = 0.;
    let mut v: [libc::c_double; 8] = [0.; 8];
    let mut w: [libc::c_double; 12] = [0.; 12];
    let mut vlength: libc::c_int = 0;
    let mut wlength: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    axby1 = *pa.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = axby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axby0 = alo * blo - err3;
    axcy1 = *pa.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = axcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axcy0 = alo * blo - err3;
    _i = axby0 - axcy0;
    bvirt = axby0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axcy0;
    around = axby0 - avirt;
    aterms[0 as libc::c_int as usize] = around + bround;
    _j = axby1 + _i;
    bvirt = _j - axby1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axby1 - avirt;
    _0 = around + bround;
    _i = _0 - axcy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axcy1;
    around = _0 - avirt;
    aterms[1 as libc::c_int as usize] = around + bround;
    aterms3 = _j + _i;
    bvirt = aterms3 - _j;
    avirt = aterms3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    aterms[2 as libc::c_int as usize] = around + bround;
    aterms[3 as libc::c_int as usize] = aterms3;
    bxcy1 = *pb.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxcy0 = alo * blo - err3;
    bxay1 = *pb.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxay0 = alo * blo - err3;
    _i = bxcy0 - bxay0;
    bvirt = bxcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay0;
    around = bxcy0 - avirt;
    bterms[0 as libc::c_int as usize] = around + bround;
    _j = bxcy1 + _i;
    bvirt = _j - bxcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxcy1 - avirt;
    _0 = around + bround;
    _i = _0 - bxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay1;
    around = _0 - avirt;
    bterms[1 as libc::c_int as usize] = around + bround;
    bterms3 = _j + _i;
    bvirt = bterms3 - _j;
    avirt = bterms3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bterms[2 as libc::c_int as usize] = around + bround;
    bterms[3 as libc::c_int as usize] = bterms3;
    cxay1 = *pc.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxay0 = alo * blo - err3;
    cxby1 = *pc.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxby0 = alo * blo - err3;
    _i = cxay0 - cxby0;
    bvirt = cxay0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby0;
    around = cxay0 - avirt;
    cterms[0 as libc::c_int as usize] = around + bround;
    _j = cxay1 + _i;
    bvirt = _j - cxay1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cxay1 - avirt;
    _0 = around + bround;
    _i = _0 - cxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby1;
    around = _0 - avirt;
    cterms[1 as libc::c_int as usize] = around + bround;
    cterms3 = _j + _i;
    bvirt = cterms3 - _j;
    avirt = cterms3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    cterms[2 as libc::c_int as usize] = around + bround;
    cterms[3 as libc::c_int as usize] = cterms3;
    vlength = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        aterms.as_mut_ptr(),
        4 as libc::c_int,
        bterms.as_mut_ptr(),
        v.as_mut_ptr(),
    );
    wlength = fast_expansion_sum_zeroelim(
        vlength,
        v.as_mut_ptr(),
        4 as libc::c_int,
        cterms.as_mut_ptr(),
        w.as_mut_ptr(),
    );
    return w[(wlength - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn orient2dslow(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
) -> libc::c_double {
    let mut acx: libc::c_double = 0.;
    let mut acy: libc::c_double = 0.;
    let mut bcx: libc::c_double = 0.;
    let mut bcy: libc::c_double = 0.;
    let mut acxtail: libc::c_double = 0.;
    let mut acytail: libc::c_double = 0.;
    let mut bcxtail: libc::c_double = 0.;
    let mut bcytail: libc::c_double = 0.;
    let mut negate: libc::c_double = 0.;
    let mut negatetail: libc::c_double = 0.;
    let mut axby: [libc::c_double; 8] = [0.; 8];
    let mut bxay: [libc::c_double; 8] = [0.; 8];
    let mut axby7: libc::c_double = 0.;
    let mut bxay7: libc::c_double = 0.;
    let mut deter: [libc::c_double; 16] = [0.; 16];
    let mut deterlen: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut a0hi: libc::c_double = 0.;
    let mut a0lo: libc::c_double = 0.;
    let mut a1hi: libc::c_double = 0.;
    let mut a1lo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _k: libc::c_double = 0.;
    let mut _l: libc::c_double = 0.;
    let mut _m: libc::c_double = 0.;
    let mut _n: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    let mut _1: libc::c_double = 0.;
    let mut _2: libc::c_double = 0.;
    acx = *pa.offset(0 as libc::c_int as isize) - *pc.offset(0 as libc::c_int as isize);
    bvirt = *pa.offset(0 as libc::c_int as isize) - acx;
    avirt = acx + bvirt;
    bround = bvirt - *pc.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    acxtail = around + bround;
    acy = *pa.offset(1 as libc::c_int as isize) - *pc.offset(1 as libc::c_int as isize);
    bvirt = *pa.offset(1 as libc::c_int as isize) - acy;
    avirt = acy + bvirt;
    bround = bvirt - *pc.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    acytail = around + bround;
    bcx = *pb.offset(0 as libc::c_int as isize) - *pc.offset(0 as libc::c_int as isize);
    bvirt = *pb.offset(0 as libc::c_int as isize) - bcx;
    avirt = bcx + bvirt;
    bround = bvirt - *pc.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bcxtail = around + bround;
    bcy = *pb.offset(1 as libc::c_int as isize) - *pc.offset(1 as libc::c_int as isize);
    bvirt = *pb.offset(1 as libc::c_int as isize) - bcy;
    avirt = bcy + bvirt;
    bround = bvirt - *pc.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    bcytail = around + bround;
    c = splitter * acxtail;
    abig = c - acxtail;
    a0hi = c - abig;
    a0lo = acxtail - a0hi;
    c = splitter * bcytail;
    abig = c - bcytail;
    bhi = c - abig;
    blo = bcytail - bhi;
    _i = acxtail * bcytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * acx;
    abig = c - acx;
    a1hi = c - abig;
    a1lo = acx - a1hi;
    _j = acx * bcytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * bcy;
    abig = c - bcy;
    bhi = c - abig;
    blo = bcy - bhi;
    _i = acxtail * bcy;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = acx * bcy;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axby[5 as libc::c_int as usize] = around + bround;
    axby7 = _m + _k;
    bvirt = axby7 - _m;
    avirt = axby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axby[6 as libc::c_int as usize] = around + bround;
    axby[7 as libc::c_int as usize] = axby7;
    negate = -acy;
    negatetail = -acytail;
    c = splitter * bcxtail;
    abig = c - bcxtail;
    a0hi = c - abig;
    a0lo = bcxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = bcxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bcx;
    abig = c - bcx;
    a1hi = c - abig;
    a1lo = bcx - a1hi;
    _j = bcx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = bcxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bcx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxay[5 as libc::c_int as usize] = around + bround;
    bxay7 = _m + _k;
    bvirt = bxay7 - _m;
    avirt = bxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxay[6 as libc::c_int as usize] = around + bround;
    bxay[7 as libc::c_int as usize] = bxay7;
    deterlen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        axby.as_mut_ptr(),
        8 as libc::c_int,
        bxay.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn orient2dadapt(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut detsum: libc::c_double,
) -> libc::c_double {
    let mut acx: libc::c_double = 0.;
    let mut acy: libc::c_double = 0.;
    let mut bcx: libc::c_double = 0.;
    let mut bcy: libc::c_double = 0.;
    let mut acxtail: libc::c_double = 0.;
    let mut acytail: libc::c_double = 0.;
    let mut bcxtail: libc::c_double = 0.;
    let mut bcytail: libc::c_double = 0.;
    let mut detleft: libc::c_double = 0.;
    let mut detright: libc::c_double = 0.;
    let mut detlefttail: libc::c_double = 0.;
    let mut detrighttail: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    let mut B: [libc::c_double; 4] = [0.; 4];
    let mut C1: [libc::c_double; 8] = [0.; 8];
    let mut C2: [libc::c_double; 12] = [0.; 12];
    let mut D: [libc::c_double; 16] = [0.; 16];
    let mut B3: libc::c_double = 0.;
    let mut C1length: libc::c_int = 0;
    let mut C2length: libc::c_int = 0;
    let mut Dlength: libc::c_int = 0;
    let mut u: [libc::c_double; 4] = [0.; 4];
    let mut u3: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut s0: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    acx = *pa.offset(0 as libc::c_int as isize) - *pc.offset(0 as libc::c_int as isize);
    bcx = *pb.offset(0 as libc::c_int as isize) - *pc.offset(0 as libc::c_int as isize);
    acy = *pa.offset(1 as libc::c_int as isize) - *pc.offset(1 as libc::c_int as isize);
    bcy = *pb.offset(1 as libc::c_int as isize) - *pc.offset(1 as libc::c_int as isize);
    detleft = acx * bcy;
    c = splitter * acx;
    abig = c - acx;
    ahi = c - abig;
    alo = acx - ahi;
    c = splitter * bcy;
    abig = c - bcy;
    bhi = c - abig;
    blo = bcy - bhi;
    err1 = detleft - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    detlefttail = alo * blo - err3;
    detright = acy * bcx;
    c = splitter * acy;
    abig = c - acy;
    ahi = c - abig;
    alo = acy - ahi;
    c = splitter * bcx;
    abig = c - bcx;
    bhi = c - abig;
    blo = bcx - bhi;
    err1 = detright - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    detrighttail = alo * blo - err3;
    _i = detlefttail - detrighttail;
    bvirt = detlefttail - _i;
    avirt = _i + bvirt;
    bround = bvirt - detrighttail;
    around = detlefttail - avirt;
    B[0 as libc::c_int as usize] = around + bround;
    _j = detleft + _i;
    bvirt = _j - detleft;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = detleft - avirt;
    _0 = around + bround;
    _i = _0 - detright;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - detright;
    around = _0 - avirt;
    B[1 as libc::c_int as usize] = around + bround;
    B3 = _j + _i;
    bvirt = B3 - _j;
    avirt = B3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    B[2 as libc::c_int as usize] = around + bround;
    B[3 as libc::c_int as usize] = B3;
    det = estimate(4 as libc::c_int, B.as_mut_ptr());
    errbound = ccwerrboundB * detsum;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = *pa.offset(0 as libc::c_int as isize) - acx;
    avirt = acx + bvirt;
    bround = bvirt - *pc.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    acxtail = around + bround;
    bvirt = *pb.offset(0 as libc::c_int as isize) - bcx;
    avirt = bcx + bvirt;
    bround = bvirt - *pc.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bcxtail = around + bround;
    bvirt = *pa.offset(1 as libc::c_int as isize) - acy;
    avirt = acy + bvirt;
    bround = bvirt - *pc.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    acytail = around + bround;
    bvirt = *pb.offset(1 as libc::c_int as isize) - bcy;
    avirt = bcy + bvirt;
    bround = bvirt - *pc.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    bcytail = around + bround;
    if acxtail == 0.0f64 && acytail == 0.0f64 && bcxtail == 0.0f64 && bcytail == 0.0f64 {
        return det;
    }
    errbound = ccwerrboundC * detsum
        + resulterrbound * (if det >= 0.0f64 { det } else { -det });
    det += acx * bcytail + bcy * acxtail - (acy * bcxtail + bcx * acytail);
    if det >= errbound || -det >= errbound {
        return det;
    }
    s1 = acxtail * bcy;
    c = splitter * acxtail;
    abig = c - acxtail;
    ahi = c - abig;
    alo = acxtail - ahi;
    c = splitter * bcy;
    abig = c - bcy;
    bhi = c - abig;
    blo = bcy - bhi;
    err1 = s1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    s0 = alo * blo - err3;
    t1 = acytail * bcx;
    c = splitter * acytail;
    abig = c - acytail;
    ahi = c - abig;
    alo = acytail - ahi;
    c = splitter * bcx;
    abig = c - bcx;
    bhi = c - abig;
    blo = bcx - bhi;
    err1 = t1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    t0 = alo * blo - err3;
    _i = s0 - t0;
    bvirt = s0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - t0;
    around = s0 - avirt;
    u[0 as libc::c_int as usize] = around + bround;
    _j = s1 + _i;
    bvirt = _j - s1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = s1 - avirt;
    _0 = around + bround;
    _i = _0 - t1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - t1;
    around = _0 - avirt;
    u[1 as libc::c_int as usize] = around + bround;
    u3 = _j + _i;
    bvirt = u3 - _j;
    avirt = u3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    u[2 as libc::c_int as usize] = around + bround;
    u[3 as libc::c_int as usize] = u3;
    C1length = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        B.as_mut_ptr(),
        4 as libc::c_int,
        u.as_mut_ptr(),
        C1.as_mut_ptr(),
    );
    s1 = acx * bcytail;
    c = splitter * acx;
    abig = c - acx;
    ahi = c - abig;
    alo = acx - ahi;
    c = splitter * bcytail;
    abig = c - bcytail;
    bhi = c - abig;
    blo = bcytail - bhi;
    err1 = s1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    s0 = alo * blo - err3;
    t1 = acy * bcxtail;
    c = splitter * acy;
    abig = c - acy;
    ahi = c - abig;
    alo = acy - ahi;
    c = splitter * bcxtail;
    abig = c - bcxtail;
    bhi = c - abig;
    blo = bcxtail - bhi;
    err1 = t1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    t0 = alo * blo - err3;
    _i = s0 - t0;
    bvirt = s0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - t0;
    around = s0 - avirt;
    u[0 as libc::c_int as usize] = around + bround;
    _j = s1 + _i;
    bvirt = _j - s1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = s1 - avirt;
    _0 = around + bround;
    _i = _0 - t1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - t1;
    around = _0 - avirt;
    u[1 as libc::c_int as usize] = around + bround;
    u3 = _j + _i;
    bvirt = u3 - _j;
    avirt = u3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    u[2 as libc::c_int as usize] = around + bround;
    u[3 as libc::c_int as usize] = u3;
    C2length = fast_expansion_sum_zeroelim(
        C1length,
        C1.as_mut_ptr(),
        4 as libc::c_int,
        u.as_mut_ptr(),
        C2.as_mut_ptr(),
    );
    s1 = acxtail * bcytail;
    c = splitter * acxtail;
    abig = c - acxtail;
    ahi = c - abig;
    alo = acxtail - ahi;
    c = splitter * bcytail;
    abig = c - bcytail;
    bhi = c - abig;
    blo = bcytail - bhi;
    err1 = s1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    s0 = alo * blo - err3;
    t1 = acytail * bcxtail;
    c = splitter * acytail;
    abig = c - acytail;
    ahi = c - abig;
    alo = acytail - ahi;
    c = splitter * bcxtail;
    abig = c - bcxtail;
    bhi = c - abig;
    blo = bcxtail - bhi;
    err1 = t1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    t0 = alo * blo - err3;
    _i = s0 - t0;
    bvirt = s0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - t0;
    around = s0 - avirt;
    u[0 as libc::c_int as usize] = around + bround;
    _j = s1 + _i;
    bvirt = _j - s1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = s1 - avirt;
    _0 = around + bround;
    _i = _0 - t1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - t1;
    around = _0 - avirt;
    u[1 as libc::c_int as usize] = around + bround;
    u3 = _j + _i;
    bvirt = u3 - _j;
    avirt = u3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    u[2 as libc::c_int as usize] = around + bround;
    u[3 as libc::c_int as usize] = u3;
    Dlength = fast_expansion_sum_zeroelim(
        C2length,
        C2.as_mut_ptr(),
        4 as libc::c_int,
        u.as_mut_ptr(),
        D.as_mut_ptr(),
    );
    return D[(Dlength - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn orient2d(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
) -> libc::c_double {
    let mut detleft: libc::c_double = 0.;
    let mut detright: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut detsum: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    detleft = (*pa.offset(0 as libc::c_int as isize)
        - *pc.offset(0 as libc::c_int as isize))
        * (*pb.offset(1 as libc::c_int as isize)
            - *pc.offset(1 as libc::c_int as isize));
    detright = (*pa.offset(1 as libc::c_int as isize)
        - *pc.offset(1 as libc::c_int as isize))
        * (*pb.offset(0 as libc::c_int as isize)
            - *pc.offset(0 as libc::c_int as isize));
    det = detleft - detright;
    if detleft > 0.0f64 {
        if detright <= 0.0f64 {
            return det
        } else {
            detsum = detleft + detright;
        }
    } else if detleft < 0.0f64 {
        if detright >= 0.0f64 {
            return det
        } else {
            detsum = -detleft - detright;
        }
    } else {
        return det
    }
    errbound = ccwerrboundA * detsum;
    if det >= errbound || -det >= errbound {
        return det;
    }
    return orient2dadapt(pa, pb, pc, detsum);
}
#[no_mangle]
pub unsafe extern "C" fn orient3dfast(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut adz: libc::c_double = 0.;
    let mut bdz: libc::c_double = 0.;
    let mut cdz: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    adz = *pa.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bdz = *pb.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    cdz = *pc.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    return adx * (bdy * cdz - bdz * cdy) + bdx * (cdy * adz - cdz * ady)
        + cdx * (ady * bdz - adz * bdy);
}
#[no_mangle]
pub unsafe extern "C" fn orient3dexact(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut axby1: libc::c_double = 0.;
    let mut bxcy1: libc::c_double = 0.;
    let mut cxdy1: libc::c_double = 0.;
    let mut dxay1: libc::c_double = 0.;
    let mut axcy1: libc::c_double = 0.;
    let mut bxdy1: libc::c_double = 0.;
    let mut bxay1: libc::c_double = 0.;
    let mut cxby1: libc::c_double = 0.;
    let mut dxcy1: libc::c_double = 0.;
    let mut axdy1: libc::c_double = 0.;
    let mut cxay1: libc::c_double = 0.;
    let mut dxby1: libc::c_double = 0.;
    let mut axby0: libc::c_double = 0.;
    let mut bxcy0: libc::c_double = 0.;
    let mut cxdy0: libc::c_double = 0.;
    let mut dxay0: libc::c_double = 0.;
    let mut axcy0: libc::c_double = 0.;
    let mut bxdy0: libc::c_double = 0.;
    let mut bxay0: libc::c_double = 0.;
    let mut cxby0: libc::c_double = 0.;
    let mut dxcy0: libc::c_double = 0.;
    let mut axdy0: libc::c_double = 0.;
    let mut cxay0: libc::c_double = 0.;
    let mut dxby0: libc::c_double = 0.;
    let mut ab: [libc::c_double; 4] = [0.; 4];
    let mut bc: [libc::c_double; 4] = [0.; 4];
    let mut cd: [libc::c_double; 4] = [0.; 4];
    let mut da: [libc::c_double; 4] = [0.; 4];
    let mut ac: [libc::c_double; 4] = [0.; 4];
    let mut bd: [libc::c_double; 4] = [0.; 4];
    let mut temp8: [libc::c_double; 8] = [0.; 8];
    let mut templen: libc::c_int = 0;
    let mut abc: [libc::c_double; 12] = [0.; 12];
    let mut bcd: [libc::c_double; 12] = [0.; 12];
    let mut cda: [libc::c_double; 12] = [0.; 12];
    let mut dab: [libc::c_double; 12] = [0.; 12];
    let mut abclen: libc::c_int = 0;
    let mut bcdlen: libc::c_int = 0;
    let mut cdalen: libc::c_int = 0;
    let mut dablen: libc::c_int = 0;
    let mut adet: [libc::c_double; 24] = [0.; 24];
    let mut bdet: [libc::c_double; 24] = [0.; 24];
    let mut cdet: [libc::c_double; 24] = [0.; 24];
    let mut ddet: [libc::c_double; 24] = [0.; 24];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 48] = [0.; 48];
    let mut cddet: [libc::c_double; 48] = [0.; 48];
    let mut ablen: libc::c_int = 0;
    let mut cdlen: libc::c_int = 0;
    let mut deter: [libc::c_double; 96] = [0.; 96];
    let mut deterlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    axby1 = *pa.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = axby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axby0 = alo * blo - err3;
    bxay1 = *pb.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxay0 = alo * blo - err3;
    _i = axby0 - bxay0;
    bvirt = axby0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay0;
    around = axby0 - avirt;
    ab[0 as libc::c_int as usize] = around + bround;
    _j = axby1 + _i;
    bvirt = _j - axby1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axby1 - avirt;
    _0 = around + bround;
    _i = _0 - bxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay1;
    around = _0 - avirt;
    ab[1 as libc::c_int as usize] = around + bround;
    ab[3 as libc::c_int as usize] = _j + _i;
    bvirt = ab[3 as libc::c_int as usize] - _j;
    avirt = ab[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as libc::c_int as usize] = around + bround;
    bxcy1 = *pb.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxcy0 = alo * blo - err3;
    cxby1 = *pc.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxby0 = alo * blo - err3;
    _i = bxcy0 - cxby0;
    bvirt = bxcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby0;
    around = bxcy0 - avirt;
    bc[0 as libc::c_int as usize] = around + bround;
    _j = bxcy1 + _i;
    bvirt = _j - bxcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxcy1 - avirt;
    _0 = around + bround;
    _i = _0 - cxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby1;
    around = _0 - avirt;
    bc[1 as libc::c_int as usize] = around + bround;
    bc[3 as libc::c_int as usize] = _j + _i;
    bvirt = bc[3 as libc::c_int as usize] - _j;
    avirt = bc[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as libc::c_int as usize] = around + bround;
    cxdy1 = *pc.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxdy0 = alo * blo - err3;
    dxcy1 = *pd.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxcy0 = alo * blo - err3;
    _i = cxdy0 - dxcy0;
    bvirt = cxdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxcy0;
    around = cxdy0 - avirt;
    cd[0 as libc::c_int as usize] = around + bround;
    _j = cxdy1 + _i;
    bvirt = _j - cxdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cxdy1 - avirt;
    _0 = around + bround;
    _i = _0 - dxcy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxcy1;
    around = _0 - avirt;
    cd[1 as libc::c_int as usize] = around + bround;
    cd[3 as libc::c_int as usize] = _j + _i;
    bvirt = cd[3 as libc::c_int as usize] - _j;
    avirt = cd[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    cd[2 as libc::c_int as usize] = around + bround;
    dxay1 = *pd.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxay0 = alo * blo - err3;
    axdy1 = *pa.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = axdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axdy0 = alo * blo - err3;
    _i = dxay0 - axdy0;
    bvirt = dxay0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axdy0;
    around = dxay0 - avirt;
    da[0 as libc::c_int as usize] = around + bround;
    _j = dxay1 + _i;
    bvirt = _j - dxay1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = dxay1 - avirt;
    _0 = around + bround;
    _i = _0 - axdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axdy1;
    around = _0 - avirt;
    da[1 as libc::c_int as usize] = around + bround;
    da[3 as libc::c_int as usize] = _j + _i;
    bvirt = da[3 as libc::c_int as usize] - _j;
    avirt = da[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    da[2 as libc::c_int as usize] = around + bround;
    axcy1 = *pa.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = axcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axcy0 = alo * blo - err3;
    cxay1 = *pc.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxay0 = alo * blo - err3;
    _i = axcy0 - cxay0;
    bvirt = axcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxay0;
    around = axcy0 - avirt;
    ac[0 as libc::c_int as usize] = around + bround;
    _j = axcy1 + _i;
    bvirt = _j - axcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axcy1 - avirt;
    _0 = around + bround;
    _i = _0 - cxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxay1;
    around = _0 - avirt;
    ac[1 as libc::c_int as usize] = around + bround;
    ac[3 as libc::c_int as usize] = _j + _i;
    bvirt = ac[3 as libc::c_int as usize] - _j;
    avirt = ac[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ac[2 as libc::c_int as usize] = around + bround;
    bxdy1 = *pb.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxdy0 = alo * blo - err3;
    dxby1 = *pd.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxby0 = alo * blo - err3;
    _i = bxdy0 - dxby0;
    bvirt = bxdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxby0;
    around = bxdy0 - avirt;
    bd[0 as libc::c_int as usize] = around + bround;
    _j = bxdy1 + _i;
    bvirt = _j - bxdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxdy1 - avirt;
    _0 = around + bround;
    _i = _0 - dxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxby1;
    around = _0 - avirt;
    bd[1 as libc::c_int as usize] = around + bround;
    bd[3 as libc::c_int as usize] = _j + _i;
    bvirt = bd[3 as libc::c_int as usize] - _j;
    avirt = bd[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bd[2 as libc::c_int as usize] = around + bround;
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        4 as libc::c_int,
        da.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    cdalen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        ac.as_mut_ptr(),
        cda.as_mut_ptr(),
    );
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        4 as libc::c_int,
        ab.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    dablen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        bd.as_mut_ptr(),
        dab.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        bd[i as usize] = -bd[i as usize];
        ac[i as usize] = -ac[i as usize];
        i += 1;
    }
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        4 as libc::c_int,
        bc.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    abclen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        ac.as_mut_ptr(),
        abc.as_mut_ptr(),
    );
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        4 as libc::c_int,
        cd.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    bcdlen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        bd.as_mut_ptr(),
        bcd.as_mut_ptr(),
    );
    alen = scale_expansion_zeroelim(
        bcdlen,
        bcd.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        adet.as_mut_ptr(),
    );
    blen = scale_expansion_zeroelim(
        cdalen,
        cda.as_mut_ptr(),
        -*pb.offset(2 as libc::c_int as isize),
        bdet.as_mut_ptr(),
    );
    clen = scale_expansion_zeroelim(
        dablen,
        dab.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        cdet.as_mut_ptr(),
    );
    dlen = scale_expansion_zeroelim(
        abclen,
        abc.as_mut_ptr(),
        -*pd.offset(2 as libc::c_int as isize),
        ddet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    cdlen = fast_expansion_sum_zeroelim(
        clen,
        cdet.as_mut_ptr(),
        dlen,
        ddet.as_mut_ptr(),
        cddet.as_mut_ptr(),
    );
    deterlen = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        cdlen,
        cddet.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn orient3dslow(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut adz: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut bdz: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut cdz: libc::c_double = 0.;
    let mut adxtail: libc::c_double = 0.;
    let mut adytail: libc::c_double = 0.;
    let mut adztail: libc::c_double = 0.;
    let mut bdxtail: libc::c_double = 0.;
    let mut bdytail: libc::c_double = 0.;
    let mut bdztail: libc::c_double = 0.;
    let mut cdxtail: libc::c_double = 0.;
    let mut cdytail: libc::c_double = 0.;
    let mut cdztail: libc::c_double = 0.;
    let mut negate: libc::c_double = 0.;
    let mut negatetail: libc::c_double = 0.;
    let mut axby7: libc::c_double = 0.;
    let mut bxcy7: libc::c_double = 0.;
    let mut axcy7: libc::c_double = 0.;
    let mut bxay7: libc::c_double = 0.;
    let mut cxby7: libc::c_double = 0.;
    let mut cxay7: libc::c_double = 0.;
    let mut axby: [libc::c_double; 8] = [0.; 8];
    let mut bxcy: [libc::c_double; 8] = [0.; 8];
    let mut axcy: [libc::c_double; 8] = [0.; 8];
    let mut bxay: [libc::c_double; 8] = [0.; 8];
    let mut cxby: [libc::c_double; 8] = [0.; 8];
    let mut cxay: [libc::c_double; 8] = [0.; 8];
    let mut temp16: [libc::c_double; 16] = [0.; 16];
    let mut temp32: [libc::c_double; 32] = [0.; 32];
    let mut temp32t: [libc::c_double; 32] = [0.; 32];
    let mut temp16len: libc::c_int = 0;
    let mut temp32len: libc::c_int = 0;
    let mut temp32tlen: libc::c_int = 0;
    let mut adet: [libc::c_double; 64] = [0.; 64];
    let mut bdet: [libc::c_double; 64] = [0.; 64];
    let mut cdet: [libc::c_double; 64] = [0.; 64];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 128] = [0.; 128];
    let mut ablen: libc::c_int = 0;
    let mut deter: [libc::c_double; 192] = [0.; 192];
    let mut deterlen: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut a0hi: libc::c_double = 0.;
    let mut a0lo: libc::c_double = 0.;
    let mut a1hi: libc::c_double = 0.;
    let mut a1lo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _k: libc::c_double = 0.;
    let mut _l: libc::c_double = 0.;
    let mut _m: libc::c_double = 0.;
    let mut _n: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    let mut _1: libc::c_double = 0.;
    let mut _2: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bvirt = *pa.offset(0 as libc::c_int as isize) - adx;
    avirt = adx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    adxtail = around + bround;
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bvirt = *pa.offset(1 as libc::c_int as isize) - ady;
    avirt = ady + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    adytail = around + bround;
    adz = *pa.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bvirt = *pa.offset(2 as libc::c_int as isize) - adz;
    avirt = adz + bvirt;
    bround = bvirt - *pd.offset(2 as libc::c_int as isize);
    around = *pa.offset(2 as libc::c_int as isize) - avirt;
    adztail = around + bround;
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bvirt = *pb.offset(0 as libc::c_int as isize) - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bdxtail = around + bround;
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bvirt = *pb.offset(1 as libc::c_int as isize) - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    bdytail = around + bround;
    bdz = *pb.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bvirt = *pb.offset(2 as libc::c_int as isize) - bdz;
    avirt = bdz + bvirt;
    bround = bvirt - *pd.offset(2 as libc::c_int as isize);
    around = *pb.offset(2 as libc::c_int as isize) - avirt;
    bdztail = around + bround;
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bvirt = *pc.offset(0 as libc::c_int as isize) - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pc.offset(0 as libc::c_int as isize) - avirt;
    cdxtail = around + bround;
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bvirt = *pc.offset(1 as libc::c_int as isize) - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pc.offset(1 as libc::c_int as isize) - avirt;
    cdytail = around + bround;
    cdz = *pc.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bvirt = *pc.offset(2 as libc::c_int as isize) - cdz;
    avirt = cdz + bvirt;
    bround = bvirt - *pd.offset(2 as libc::c_int as isize);
    around = *pc.offset(2 as libc::c_int as isize) - avirt;
    cdztail = around + bround;
    c = splitter * adxtail;
    abig = c - adxtail;
    a0hi = c - abig;
    a0lo = adxtail - a0hi;
    c = splitter * bdytail;
    abig = c - bdytail;
    bhi = c - abig;
    blo = bdytail - bhi;
    _i = adxtail * bdytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * adx;
    abig = c - adx;
    a1hi = c - abig;
    a1lo = adx - a1hi;
    _j = adx * bdytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    _i = adxtail * bdy;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = adx * bdy;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axby[5 as libc::c_int as usize] = around + bround;
    axby7 = _m + _k;
    bvirt = axby7 - _m;
    avirt = axby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axby[6 as libc::c_int as usize] = around + bround;
    axby[7 as libc::c_int as usize] = axby7;
    negate = -ady;
    negatetail = -adytail;
    c = splitter * bdxtail;
    abig = c - bdxtail;
    a0hi = c - abig;
    a0lo = bdxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = bdxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bdx;
    abig = c - bdx;
    a1hi = c - abig;
    a1lo = bdx - a1hi;
    _j = bdx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = bdxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bdx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxay[5 as libc::c_int as usize] = around + bround;
    bxay7 = _m + _k;
    bvirt = bxay7 - _m;
    avirt = bxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxay[6 as libc::c_int as usize] = around + bround;
    bxay[7 as libc::c_int as usize] = bxay7;
    c = splitter * bdxtail;
    abig = c - bdxtail;
    a0hi = c - abig;
    a0lo = bdxtail - a0hi;
    c = splitter * cdytail;
    abig = c - cdytail;
    bhi = c - abig;
    blo = cdytail - bhi;
    _i = bdxtail * cdytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bdx;
    abig = c - bdx;
    a1hi = c - abig;
    a1lo = bdx - a1hi;
    _j = bdx * cdytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    _i = bdxtail * cdy;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bdx * cdy;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxcy[5 as libc::c_int as usize] = around + bround;
    bxcy7 = _m + _k;
    bvirt = bxcy7 - _m;
    avirt = bxcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxcy[6 as libc::c_int as usize] = around + bround;
    bxcy[7 as libc::c_int as usize] = bxcy7;
    negate = -bdy;
    negatetail = -bdytail;
    c = splitter * cdxtail;
    abig = c - cdxtail;
    a0hi = c - abig;
    a0lo = cdxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = cdxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cdx;
    abig = c - cdx;
    a1hi = c - abig;
    a1lo = cdx - a1hi;
    _j = cdx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = cdxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cdx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxby[5 as libc::c_int as usize] = around + bround;
    cxby7 = _m + _k;
    bvirt = cxby7 - _m;
    avirt = cxby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxby[6 as libc::c_int as usize] = around + bround;
    cxby[7 as libc::c_int as usize] = cxby7;
    c = splitter * cdxtail;
    abig = c - cdxtail;
    a0hi = c - abig;
    a0lo = cdxtail - a0hi;
    c = splitter * adytail;
    abig = c - adytail;
    bhi = c - abig;
    blo = adytail - bhi;
    _i = cdxtail * adytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cdx;
    abig = c - cdx;
    a1hi = c - abig;
    a1lo = cdx - a1hi;
    _j = cdx * adytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    _i = cdxtail * ady;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cdx * ady;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxay[5 as libc::c_int as usize] = around + bround;
    cxay7 = _m + _k;
    bvirt = cxay7 - _m;
    avirt = cxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxay[6 as libc::c_int as usize] = around + bround;
    cxay[7 as libc::c_int as usize] = cxay7;
    negate = -cdy;
    negatetail = -cdytail;
    c = splitter * adxtail;
    abig = c - adxtail;
    a0hi = c - abig;
    a0lo = adxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = adxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * adx;
    abig = c - adx;
    a1hi = c - abig;
    a1lo = adx - a1hi;
    _j = adx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = adxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = adx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axcy[5 as libc::c_int as usize] = around + bround;
    axcy7 = _m + _k;
    bvirt = axcy7 - _m;
    avirt = axcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axcy[6 as libc::c_int as usize] = around + bround;
    axcy[7 as libc::c_int as usize] = axcy7;
    temp16len = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        bxcy.as_mut_ptr(),
        8 as libc::c_int,
        cxby.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp32len = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        adz,
        temp32.as_mut_ptr(),
    );
    temp32tlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        adztail,
        temp32t.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        temp32len,
        temp32.as_mut_ptr(),
        temp32tlen,
        temp32t.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        cxay.as_mut_ptr(),
        8 as libc::c_int,
        axcy.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp32len = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        bdz,
        temp32.as_mut_ptr(),
    );
    temp32tlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        bdztail,
        temp32t.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        temp32len,
        temp32.as_mut_ptr(),
        temp32tlen,
        temp32t.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        axby.as_mut_ptr(),
        8 as libc::c_int,
        bxay.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp32len = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        cdz,
        temp32.as_mut_ptr(),
    );
    temp32tlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        cdztail,
        temp32t.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        temp32len,
        temp32.as_mut_ptr(),
        temp32tlen,
        temp32t.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    deterlen = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        clen,
        cdet.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn orient3dadapt(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut permanent: libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut adz: libc::c_double = 0.;
    let mut bdz: libc::c_double = 0.;
    let mut cdz: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    let mut bdxcdy1: libc::c_double = 0.;
    let mut cdxbdy1: libc::c_double = 0.;
    let mut cdxady1: libc::c_double = 0.;
    let mut adxcdy1: libc::c_double = 0.;
    let mut adxbdy1: libc::c_double = 0.;
    let mut bdxady1: libc::c_double = 0.;
    let mut bdxcdy0: libc::c_double = 0.;
    let mut cdxbdy0: libc::c_double = 0.;
    let mut cdxady0: libc::c_double = 0.;
    let mut adxcdy0: libc::c_double = 0.;
    let mut adxbdy0: libc::c_double = 0.;
    let mut bdxady0: libc::c_double = 0.;
    let mut bc: [libc::c_double; 4] = [0.; 4];
    let mut ca: [libc::c_double; 4] = [0.; 4];
    let mut ab: [libc::c_double; 4] = [0.; 4];
    let mut bc3: libc::c_double = 0.;
    let mut ca3: libc::c_double = 0.;
    let mut ab3: libc::c_double = 0.;
    let mut adet: [libc::c_double; 8] = [0.; 8];
    let mut bdet: [libc::c_double; 8] = [0.; 8];
    let mut cdet: [libc::c_double; 8] = [0.; 8];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 16] = [0.; 16];
    let mut ablen: libc::c_int = 0;
    let mut finnow: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut finother: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut finswap: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut fin1: [libc::c_double; 192] = [0.; 192];
    let mut fin2: [libc::c_double; 192] = [0.; 192];
    let mut finlength: libc::c_int = 0;
    let mut adxtail: libc::c_double = 0.;
    let mut bdxtail: libc::c_double = 0.;
    let mut cdxtail: libc::c_double = 0.;
    let mut adytail: libc::c_double = 0.;
    let mut bdytail: libc::c_double = 0.;
    let mut cdytail: libc::c_double = 0.;
    let mut adztail: libc::c_double = 0.;
    let mut bdztail: libc::c_double = 0.;
    let mut cdztail: libc::c_double = 0.;
    let mut at_blarge: libc::c_double = 0.;
    let mut at_clarge: libc::c_double = 0.;
    let mut bt_clarge: libc::c_double = 0.;
    let mut bt_alarge: libc::c_double = 0.;
    let mut ct_alarge: libc::c_double = 0.;
    let mut ct_blarge: libc::c_double = 0.;
    let mut at_b: [libc::c_double; 4] = [0.; 4];
    let mut at_c: [libc::c_double; 4] = [0.; 4];
    let mut bt_c: [libc::c_double; 4] = [0.; 4];
    let mut bt_a: [libc::c_double; 4] = [0.; 4];
    let mut ct_a: [libc::c_double; 4] = [0.; 4];
    let mut ct_b: [libc::c_double; 4] = [0.; 4];
    let mut at_blen: libc::c_int = 0;
    let mut at_clen: libc::c_int = 0;
    let mut bt_clen: libc::c_int = 0;
    let mut bt_alen: libc::c_int = 0;
    let mut ct_alen: libc::c_int = 0;
    let mut ct_blen: libc::c_int = 0;
    let mut bdxt_cdy1: libc::c_double = 0.;
    let mut cdxt_bdy1: libc::c_double = 0.;
    let mut cdxt_ady1: libc::c_double = 0.;
    let mut adxt_cdy1: libc::c_double = 0.;
    let mut adxt_bdy1: libc::c_double = 0.;
    let mut bdxt_ady1: libc::c_double = 0.;
    let mut bdxt_cdy0: libc::c_double = 0.;
    let mut cdxt_bdy0: libc::c_double = 0.;
    let mut cdxt_ady0: libc::c_double = 0.;
    let mut adxt_cdy0: libc::c_double = 0.;
    let mut adxt_bdy0: libc::c_double = 0.;
    let mut bdxt_ady0: libc::c_double = 0.;
    let mut bdyt_cdx1: libc::c_double = 0.;
    let mut cdyt_bdx1: libc::c_double = 0.;
    let mut cdyt_adx1: libc::c_double = 0.;
    let mut adyt_cdx1: libc::c_double = 0.;
    let mut adyt_bdx1: libc::c_double = 0.;
    let mut bdyt_adx1: libc::c_double = 0.;
    let mut bdyt_cdx0: libc::c_double = 0.;
    let mut cdyt_bdx0: libc::c_double = 0.;
    let mut cdyt_adx0: libc::c_double = 0.;
    let mut adyt_cdx0: libc::c_double = 0.;
    let mut adyt_bdx0: libc::c_double = 0.;
    let mut bdyt_adx0: libc::c_double = 0.;
    let mut bct: [libc::c_double; 8] = [0.; 8];
    let mut cat: [libc::c_double; 8] = [0.; 8];
    let mut abt: [libc::c_double; 8] = [0.; 8];
    let mut bctlen: libc::c_int = 0;
    let mut catlen: libc::c_int = 0;
    let mut abtlen: libc::c_int = 0;
    let mut bdxt_cdyt1: libc::c_double = 0.;
    let mut cdxt_bdyt1: libc::c_double = 0.;
    let mut cdxt_adyt1: libc::c_double = 0.;
    let mut adxt_cdyt1: libc::c_double = 0.;
    let mut adxt_bdyt1: libc::c_double = 0.;
    let mut bdxt_adyt1: libc::c_double = 0.;
    let mut bdxt_cdyt0: libc::c_double = 0.;
    let mut cdxt_bdyt0: libc::c_double = 0.;
    let mut cdxt_adyt0: libc::c_double = 0.;
    let mut adxt_cdyt0: libc::c_double = 0.;
    let mut adxt_bdyt0: libc::c_double = 0.;
    let mut bdxt_adyt0: libc::c_double = 0.;
    let mut u: [libc::c_double; 4] = [0.; 4];
    let mut v: [libc::c_double; 12] = [0.; 12];
    let mut w: [libc::c_double; 16] = [0.; 16];
    let mut u3: libc::c_double = 0.;
    let mut vlength: libc::c_int = 0;
    let mut wlength: libc::c_int = 0;
    let mut negate: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _k: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    adz = *pa.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bdz = *pb.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    cdz = *pc.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bdxcdy1 = bdx * cdy;
    c = splitter * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = bdxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bdxcdy0 = alo * blo - err3;
    cdxbdy1 = cdx * bdy;
    c = splitter * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = cdxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cdxbdy0 = alo * blo - err3;
    _i = bdxcdy0 - cdxbdy0;
    bvirt = bdxcdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy0;
    around = bdxcdy0 - avirt;
    bc[0 as libc::c_int as usize] = around + bround;
    _j = bdxcdy1 + _i;
    bvirt = _j - bdxcdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bdxcdy1 - avirt;
    _0 = around + bround;
    _i = _0 - cdxbdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy1;
    around = _0 - avirt;
    bc[1 as libc::c_int as usize] = around + bround;
    bc3 = _j + _i;
    bvirt = bc3 - _j;
    avirt = bc3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as libc::c_int as usize] = around + bround;
    bc[3 as libc::c_int as usize] = bc3;
    alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        adz,
        adet.as_mut_ptr(),
    );
    cdxady1 = cdx * ady;
    c = splitter * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = cdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cdxady0 = alo * blo - err3;
    adxcdy1 = adx * cdy;
    c = splitter * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = adxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    adxcdy0 = alo * blo - err3;
    _i = cdxady0 - adxcdy0;
    bvirt = cdxady0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy0;
    around = cdxady0 - avirt;
    ca[0 as libc::c_int as usize] = around + bround;
    _j = cdxady1 + _i;
    bvirt = _j - cdxady1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cdxady1 - avirt;
    _0 = around + bround;
    _i = _0 - adxcdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy1;
    around = _0 - avirt;
    ca[1 as libc::c_int as usize] = around + bround;
    ca3 = _j + _i;
    bvirt = ca3 - _j;
    avirt = ca3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ca[2 as libc::c_int as usize] = around + bround;
    ca[3 as libc::c_int as usize] = ca3;
    blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ca.as_mut_ptr(),
        bdz,
        bdet.as_mut_ptr(),
    );
    adxbdy1 = adx * bdy;
    c = splitter * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = adxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    adxbdy0 = alo * blo - err3;
    bdxady1 = bdx * ady;
    c = splitter * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = bdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bdxady0 = alo * blo - err3;
    _i = adxbdy0 - bdxady0;
    bvirt = adxbdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady0;
    around = adxbdy0 - avirt;
    ab[0 as libc::c_int as usize] = around + bround;
    _j = adxbdy1 + _i;
    bvirt = _j - adxbdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = adxbdy1 - avirt;
    _0 = around + bround;
    _i = _0 - bdxady1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady1;
    around = _0 - avirt;
    ab[1 as libc::c_int as usize] = around + bround;
    ab3 = _j + _i;
    bvirt = ab3 - _j;
    avirt = ab3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as libc::c_int as usize] = around + bround;
    ab[3 as libc::c_int as usize] = ab3;
    clen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        cdz,
        cdet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    finlength = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        clen,
        cdet.as_mut_ptr(),
        fin1.as_mut_ptr(),
    );
    det = estimate(finlength, fin1.as_mut_ptr());
    errbound = o3derrboundB * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = *pa.offset(0 as libc::c_int as isize) - adx;
    avirt = adx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    adxtail = around + bround;
    bvirt = *pb.offset(0 as libc::c_int as isize) - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bdxtail = around + bround;
    bvirt = *pc.offset(0 as libc::c_int as isize) - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pc.offset(0 as libc::c_int as isize) - avirt;
    cdxtail = around + bround;
    bvirt = *pa.offset(1 as libc::c_int as isize) - ady;
    avirt = ady + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    adytail = around + bround;
    bvirt = *pb.offset(1 as libc::c_int as isize) - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    bdytail = around + bround;
    bvirt = *pc.offset(1 as libc::c_int as isize) - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pc.offset(1 as libc::c_int as isize) - avirt;
    cdytail = around + bround;
    bvirt = *pa.offset(2 as libc::c_int as isize) - adz;
    avirt = adz + bvirt;
    bround = bvirt - *pd.offset(2 as libc::c_int as isize);
    around = *pa.offset(2 as libc::c_int as isize) - avirt;
    adztail = around + bround;
    bvirt = *pb.offset(2 as libc::c_int as isize) - bdz;
    avirt = bdz + bvirt;
    bround = bvirt - *pd.offset(2 as libc::c_int as isize);
    around = *pb.offset(2 as libc::c_int as isize) - avirt;
    bdztail = around + bround;
    bvirt = *pc.offset(2 as libc::c_int as isize) - cdz;
    avirt = cdz + bvirt;
    bround = bvirt - *pd.offset(2 as libc::c_int as isize);
    around = *pc.offset(2 as libc::c_int as isize) - avirt;
    cdztail = around + bround;
    if adxtail == 0.0f64 && bdxtail == 0.0f64 && cdxtail == 0.0f64 && adytail == 0.0f64
        && bdytail == 0.0f64 && cdytail == 0.0f64 && adztail == 0.0f64
        && bdztail == 0.0f64 && cdztail == 0.0f64
    {
        return det;
    }
    errbound = o3derrboundC * permanent
        + resulterrbound * (if det >= 0.0f64 { det } else { -det });
    det
        += adz * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail))
            + adztail * (bdx * cdy - bdy * cdx)
            + (bdz * (cdx * adytail + ady * cdxtail - (cdy * adxtail + adx * cdytail))
                + bdztail * (cdx * ady - cdy * adx))
            + (cdz * (adx * bdytail + bdy * adxtail - (ady * bdxtail + bdx * adytail))
                + cdztail * (adx * bdy - ady * bdx));
    if det >= errbound || -det >= errbound {
        return det;
    }
    finnow = fin1.as_mut_ptr();
    finother = fin2.as_mut_ptr();
    if adxtail == 0.0f64 {
        if adytail == 0.0f64 {
            at_b[0 as libc::c_int as usize] = 0.0f64;
            at_blen = 1 as libc::c_int;
            at_c[0 as libc::c_int as usize] = 0.0f64;
            at_clen = 1 as libc::c_int;
        } else {
            negate = -adytail;
            at_blarge = negate * bdx;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = at_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_b[0 as libc::c_int as usize] = alo * blo - err3;
            at_b[1 as libc::c_int as usize] = at_blarge;
            at_blen = 2 as libc::c_int;
            at_clarge = adytail * cdx;
            c = splitter * adytail;
            abig = c - adytail;
            ahi = c - abig;
            alo = adytail - ahi;
            c = splitter * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = at_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_c[0 as libc::c_int as usize] = alo * blo - err3;
            at_c[1 as libc::c_int as usize] = at_clarge;
            at_clen = 2 as libc::c_int;
        }
    } else if adytail == 0.0f64 {
        at_blarge = adxtail * bdy;
        c = splitter * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = at_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_b[0 as libc::c_int as usize] = alo * blo - err3;
        at_b[1 as libc::c_int as usize] = at_blarge;
        at_blen = 2 as libc::c_int;
        negate = -adxtail;
        at_clarge = negate * cdy;
        c = splitter * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = at_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_c[0 as libc::c_int as usize] = alo * blo - err3;
        at_c[1 as libc::c_int as usize] = at_clarge;
        at_clen = 2 as libc::c_int;
    } else {
        adxt_bdy1 = adxtail * bdy;
        c = splitter * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = adxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_bdy0 = alo * blo - err3;
        adyt_bdx1 = adytail * bdx;
        c = splitter * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = splitter * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = adyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_bdx0 = alo * blo - err3;
        _i = adxt_bdy0 - adyt_bdx0;
        bvirt = adxt_bdy0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adyt_bdx0;
        around = adxt_bdy0 - avirt;
        at_b[0 as libc::c_int as usize] = around + bround;
        _j = adxt_bdy1 + _i;
        bvirt = _j - adxt_bdy1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = adxt_bdy1 - avirt;
        _0 = around + bround;
        _i = _0 - adyt_bdx1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adyt_bdx1;
        around = _0 - avirt;
        at_b[1 as libc::c_int as usize] = around + bround;
        at_blarge = _j + _i;
        bvirt = at_blarge - _j;
        avirt = at_blarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        at_b[2 as libc::c_int as usize] = around + bround;
        at_b[3 as libc::c_int as usize] = at_blarge;
        at_blen = 4 as libc::c_int;
        adyt_cdx1 = adytail * cdx;
        c = splitter * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = splitter * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = adyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_cdx0 = alo * blo - err3;
        adxt_cdy1 = adxtail * cdy;
        c = splitter * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = adxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_cdy0 = alo * blo - err3;
        _i = adyt_cdx0 - adxt_cdy0;
        bvirt = adyt_cdx0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adxt_cdy0;
        around = adyt_cdx0 - avirt;
        at_c[0 as libc::c_int as usize] = around + bround;
        _j = adyt_cdx1 + _i;
        bvirt = _j - adyt_cdx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = adyt_cdx1 - avirt;
        _0 = around + bround;
        _i = _0 - adxt_cdy1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adxt_cdy1;
        around = _0 - avirt;
        at_c[1 as libc::c_int as usize] = around + bround;
        at_clarge = _j + _i;
        bvirt = at_clarge - _j;
        avirt = at_clarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        at_c[2 as libc::c_int as usize] = around + bround;
        at_c[3 as libc::c_int as usize] = at_clarge;
        at_clen = 4 as libc::c_int;
    }
    if bdxtail == 0.0f64 {
        if bdytail == 0.0f64 {
            bt_c[0 as libc::c_int as usize] = 0.0f64;
            bt_clen = 1 as libc::c_int;
            bt_a[0 as libc::c_int as usize] = 0.0f64;
            bt_alen = 1 as libc::c_int;
        } else {
            negate = -bdytail;
            bt_clarge = negate * cdx;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = bt_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_c[0 as libc::c_int as usize] = alo * blo - err3;
            bt_c[1 as libc::c_int as usize] = bt_clarge;
            bt_clen = 2 as libc::c_int;
            bt_alarge = bdytail * adx;
            c = splitter * bdytail;
            abig = c - bdytail;
            ahi = c - abig;
            alo = bdytail - ahi;
            c = splitter * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = bt_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_a[0 as libc::c_int as usize] = alo * blo - err3;
            bt_a[1 as libc::c_int as usize] = bt_alarge;
            bt_alen = 2 as libc::c_int;
        }
    } else if bdytail == 0.0f64 {
        bt_clarge = bdxtail * cdy;
        c = splitter * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bt_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_c[0 as libc::c_int as usize] = alo * blo - err3;
        bt_c[1 as libc::c_int as usize] = bt_clarge;
        bt_clen = 2 as libc::c_int;
        negate = -bdxtail;
        bt_alarge = negate * ady;
        c = splitter * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bt_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_a[0 as libc::c_int as usize] = alo * blo - err3;
        bt_a[1 as libc::c_int as usize] = bt_alarge;
        bt_alen = 2 as libc::c_int;
    } else {
        bdxt_cdy1 = bdxtail * cdy;
        c = splitter * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bdxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_cdy0 = alo * blo - err3;
        bdyt_cdx1 = bdytail * cdx;
        c = splitter * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = splitter * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = bdyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_cdx0 = alo * blo - err3;
        _i = bdxt_cdy0 - bdyt_cdx0;
        bvirt = bdxt_cdy0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdyt_cdx0;
        around = bdxt_cdy0 - avirt;
        bt_c[0 as libc::c_int as usize] = around + bround;
        _j = bdxt_cdy1 + _i;
        bvirt = _j - bdxt_cdy1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = bdxt_cdy1 - avirt;
        _0 = around + bround;
        _i = _0 - bdyt_cdx1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdyt_cdx1;
        around = _0 - avirt;
        bt_c[1 as libc::c_int as usize] = around + bround;
        bt_clarge = _j + _i;
        bvirt = bt_clarge - _j;
        avirt = bt_clarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bt_c[2 as libc::c_int as usize] = around + bround;
        bt_c[3 as libc::c_int as usize] = bt_clarge;
        bt_clen = 4 as libc::c_int;
        bdyt_adx1 = bdytail * adx;
        c = splitter * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = splitter * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = bdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_adx0 = alo * blo - err3;
        bdxt_ady1 = bdxtail * ady;
        c = splitter * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_ady0 = alo * blo - err3;
        _i = bdyt_adx0 - bdxt_ady0;
        bvirt = bdyt_adx0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdxt_ady0;
        around = bdyt_adx0 - avirt;
        bt_a[0 as libc::c_int as usize] = around + bround;
        _j = bdyt_adx1 + _i;
        bvirt = _j - bdyt_adx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = bdyt_adx1 - avirt;
        _0 = around + bround;
        _i = _0 - bdxt_ady1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdxt_ady1;
        around = _0 - avirt;
        bt_a[1 as libc::c_int as usize] = around + bround;
        bt_alarge = _j + _i;
        bvirt = bt_alarge - _j;
        avirt = bt_alarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bt_a[2 as libc::c_int as usize] = around + bround;
        bt_a[3 as libc::c_int as usize] = bt_alarge;
        bt_alen = 4 as libc::c_int;
    }
    if cdxtail == 0.0f64 {
        if cdytail == 0.0f64 {
            ct_a[0 as libc::c_int as usize] = 0.0f64;
            ct_alen = 1 as libc::c_int;
            ct_b[0 as libc::c_int as usize] = 0.0f64;
            ct_blen = 1 as libc::c_int;
        } else {
            negate = -cdytail;
            ct_alarge = negate * adx;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = ct_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_a[0 as libc::c_int as usize] = alo * blo - err3;
            ct_a[1 as libc::c_int as usize] = ct_alarge;
            ct_alen = 2 as libc::c_int;
            ct_blarge = cdytail * bdx;
            c = splitter * cdytail;
            abig = c - cdytail;
            ahi = c - abig;
            alo = cdytail - ahi;
            c = splitter * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = ct_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_b[0 as libc::c_int as usize] = alo * blo - err3;
            ct_b[1 as libc::c_int as usize] = ct_blarge;
            ct_blen = 2 as libc::c_int;
        }
    } else if cdytail == 0.0f64 {
        ct_alarge = cdxtail * ady;
        c = splitter * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = ct_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_a[0 as libc::c_int as usize] = alo * blo - err3;
        ct_a[1 as libc::c_int as usize] = ct_alarge;
        ct_alen = 2 as libc::c_int;
        negate = -cdxtail;
        ct_blarge = negate * bdy;
        c = splitter * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = ct_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_b[0 as libc::c_int as usize] = alo * blo - err3;
        ct_b[1 as libc::c_int as usize] = ct_blarge;
        ct_blen = 2 as libc::c_int;
    } else {
        cdxt_ady1 = cdxtail * ady;
        c = splitter * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = cdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_ady0 = alo * blo - err3;
        cdyt_adx1 = cdytail * adx;
        c = splitter * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = splitter * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = cdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_adx0 = alo * blo - err3;
        _i = cdxt_ady0 - cdyt_adx0;
        bvirt = cdxt_ady0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdyt_adx0;
        around = cdxt_ady0 - avirt;
        ct_a[0 as libc::c_int as usize] = around + bround;
        _j = cdxt_ady1 + _i;
        bvirt = _j - cdxt_ady1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = cdxt_ady1 - avirt;
        _0 = around + bround;
        _i = _0 - cdyt_adx1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdyt_adx1;
        around = _0 - avirt;
        ct_a[1 as libc::c_int as usize] = around + bround;
        ct_alarge = _j + _i;
        bvirt = ct_alarge - _j;
        avirt = ct_alarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        ct_a[2 as libc::c_int as usize] = around + bround;
        ct_a[3 as libc::c_int as usize] = ct_alarge;
        ct_alen = 4 as libc::c_int;
        cdyt_bdx1 = cdytail * bdx;
        c = splitter * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = splitter * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = cdyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_bdx0 = alo * blo - err3;
        cdxt_bdy1 = cdxtail * bdy;
        c = splitter * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = cdxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_bdy0 = alo * blo - err3;
        _i = cdyt_bdx0 - cdxt_bdy0;
        bvirt = cdyt_bdx0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdxt_bdy0;
        around = cdyt_bdx0 - avirt;
        ct_b[0 as libc::c_int as usize] = around + bround;
        _j = cdyt_bdx1 + _i;
        bvirt = _j - cdyt_bdx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = cdyt_bdx1 - avirt;
        _0 = around + bround;
        _i = _0 - cdxt_bdy1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdxt_bdy1;
        around = _0 - avirt;
        ct_b[1 as libc::c_int as usize] = around + bround;
        ct_blarge = _j + _i;
        bvirt = ct_blarge - _j;
        avirt = ct_blarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        ct_b[2 as libc::c_int as usize] = around + bround;
        ct_b[3 as libc::c_int as usize] = ct_blarge;
        ct_blen = 4 as libc::c_int;
    }
    bctlen = fast_expansion_sum_zeroelim(
        bt_clen,
        bt_c.as_mut_ptr(),
        ct_blen,
        ct_b.as_mut_ptr(),
        bct.as_mut_ptr(),
    );
    wlength = scale_expansion_zeroelim(bctlen, bct.as_mut_ptr(), adz, w.as_mut_ptr());
    finlength = fast_expansion_sum_zeroelim(
        finlength,
        finnow,
        wlength,
        w.as_mut_ptr(),
        finother,
    );
    finswap = finnow;
    finnow = finother;
    finother = finswap;
    catlen = fast_expansion_sum_zeroelim(
        ct_alen,
        ct_a.as_mut_ptr(),
        at_clen,
        at_c.as_mut_ptr(),
        cat.as_mut_ptr(),
    );
    wlength = scale_expansion_zeroelim(catlen, cat.as_mut_ptr(), bdz, w.as_mut_ptr());
    finlength = fast_expansion_sum_zeroelim(
        finlength,
        finnow,
        wlength,
        w.as_mut_ptr(),
        finother,
    );
    finswap = finnow;
    finnow = finother;
    finother = finswap;
    abtlen = fast_expansion_sum_zeroelim(
        at_blen,
        at_b.as_mut_ptr(),
        bt_alen,
        bt_a.as_mut_ptr(),
        abt.as_mut_ptr(),
    );
    wlength = scale_expansion_zeroelim(abtlen, abt.as_mut_ptr(), cdz, w.as_mut_ptr());
    finlength = fast_expansion_sum_zeroelim(
        finlength,
        finnow,
        wlength,
        w.as_mut_ptr(),
        finother,
    );
    finswap = finnow;
    finnow = finother;
    finother = finswap;
    if adztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(
            4 as libc::c_int,
            bc.as_mut_ptr(),
            adztail,
            v.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            vlength,
            v.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if bdztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(
            4 as libc::c_int,
            ca.as_mut_ptr(),
            bdztail,
            v.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            vlength,
            v.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if cdztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(
            4 as libc::c_int,
            ab.as_mut_ptr(),
            cdztail,
            v.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            vlength,
            v.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if adxtail != 0.0f64 {
        if bdytail != 0.0f64 {
            adxt_bdyt1 = adxtail * bdytail;
            c = splitter * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = adxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_bdyt0 = alo * blo - err3;
            c = splitter * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            _i = adxt_bdyt0 * cdz;
            c = splitter * adxt_bdyt0;
            abig = c - adxt_bdyt0;
            ahi = c - abig;
            alo = adxt_bdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as libc::c_int as usize] = alo * blo - err3;
            _j = adxt_bdyt1 * cdz;
            c = splitter * adxt_bdyt1;
            abig = c - adxt_bdyt1;
            ahi = c - abig;
            alo = adxt_bdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as libc::c_int as usize] = _k - bvirt;
            u[3 as libc::c_int as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as libc::c_int,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdztail != 0.0f64 {
                c = splitter * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                _i = adxt_bdyt0 * cdztail;
                c = splitter * adxt_bdyt0;
                abig = c - adxt_bdyt0;
                ahi = c - abig;
                alo = adxt_bdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as libc::c_int as usize] = alo * blo - err3;
                _j = adxt_bdyt1 * cdztail;
                c = splitter * adxt_bdyt1;
                abig = c - adxt_bdyt1;
                ahi = c - abig;
                alo = adxt_bdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1 as libc::c_int as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as libc::c_int as usize] = _k - bvirt;
                u[3 as libc::c_int as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as libc::c_int,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
        if cdytail != 0.0f64 {
            negate = -adxtail;
            adxt_cdyt1 = negate * cdytail;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = adxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_cdyt0 = alo * blo - err3;
            c = splitter * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            _i = adxt_cdyt0 * bdz;
            c = splitter * adxt_cdyt0;
            abig = c - adxt_cdyt0;
            ahi = c - abig;
            alo = adxt_cdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as libc::c_int as usize] = alo * blo - err3;
            _j = adxt_cdyt1 * bdz;
            c = splitter * adxt_cdyt1;
            abig = c - adxt_cdyt1;
            ahi = c - abig;
            alo = adxt_cdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as libc::c_int as usize] = _k - bvirt;
            u[3 as libc::c_int as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as libc::c_int,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdztail != 0.0f64 {
                c = splitter * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                _i = adxt_cdyt0 * bdztail;
                c = splitter * adxt_cdyt0;
                abig = c - adxt_cdyt0;
                ahi = c - abig;
                alo = adxt_cdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as libc::c_int as usize] = alo * blo - err3;
                _j = adxt_cdyt1 * bdztail;
                c = splitter * adxt_cdyt1;
                abig = c - adxt_cdyt1;
                ahi = c - abig;
                alo = adxt_cdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1 as libc::c_int as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as libc::c_int as usize] = _k - bvirt;
                u[3 as libc::c_int as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as libc::c_int,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
    }
    if bdxtail != 0.0f64 {
        if cdytail != 0.0f64 {
            bdxt_cdyt1 = bdxtail * cdytail;
            c = splitter * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = bdxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_cdyt0 = alo * blo - err3;
            c = splitter * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            _i = bdxt_cdyt0 * adz;
            c = splitter * bdxt_cdyt0;
            abig = c - bdxt_cdyt0;
            ahi = c - abig;
            alo = bdxt_cdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as libc::c_int as usize] = alo * blo - err3;
            _j = bdxt_cdyt1 * adz;
            c = splitter * bdxt_cdyt1;
            abig = c - bdxt_cdyt1;
            ahi = c - abig;
            alo = bdxt_cdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as libc::c_int as usize] = _k - bvirt;
            u[3 as libc::c_int as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as libc::c_int,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adztail != 0.0f64 {
                c = splitter * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                _i = bdxt_cdyt0 * adztail;
                c = splitter * bdxt_cdyt0;
                abig = c - bdxt_cdyt0;
                ahi = c - abig;
                alo = bdxt_cdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as libc::c_int as usize] = alo * blo - err3;
                _j = bdxt_cdyt1 * adztail;
                c = splitter * bdxt_cdyt1;
                abig = c - bdxt_cdyt1;
                ahi = c - abig;
                alo = bdxt_cdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1 as libc::c_int as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as libc::c_int as usize] = _k - bvirt;
                u[3 as libc::c_int as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as libc::c_int,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
        if adytail != 0.0f64 {
            negate = -bdxtail;
            bdxt_adyt1 = negate * adytail;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = bdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_adyt0 = alo * blo - err3;
            c = splitter * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            _i = bdxt_adyt0 * cdz;
            c = splitter * bdxt_adyt0;
            abig = c - bdxt_adyt0;
            ahi = c - abig;
            alo = bdxt_adyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as libc::c_int as usize] = alo * blo - err3;
            _j = bdxt_adyt1 * cdz;
            c = splitter * bdxt_adyt1;
            abig = c - bdxt_adyt1;
            ahi = c - abig;
            alo = bdxt_adyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as libc::c_int as usize] = _k - bvirt;
            u[3 as libc::c_int as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as libc::c_int,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdztail != 0.0f64 {
                c = splitter * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                _i = bdxt_adyt0 * cdztail;
                c = splitter * bdxt_adyt0;
                abig = c - bdxt_adyt0;
                ahi = c - abig;
                alo = bdxt_adyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as libc::c_int as usize] = alo * blo - err3;
                _j = bdxt_adyt1 * cdztail;
                c = splitter * bdxt_adyt1;
                abig = c - bdxt_adyt1;
                ahi = c - abig;
                alo = bdxt_adyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1 as libc::c_int as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as libc::c_int as usize] = _k - bvirt;
                u[3 as libc::c_int as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as libc::c_int,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
    }
    if cdxtail != 0.0f64 {
        if adytail != 0.0f64 {
            cdxt_adyt1 = cdxtail * adytail;
            c = splitter * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = cdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_adyt0 = alo * blo - err3;
            c = splitter * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            _i = cdxt_adyt0 * bdz;
            c = splitter * cdxt_adyt0;
            abig = c - cdxt_adyt0;
            ahi = c - abig;
            alo = cdxt_adyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as libc::c_int as usize] = alo * blo - err3;
            _j = cdxt_adyt1 * bdz;
            c = splitter * cdxt_adyt1;
            abig = c - cdxt_adyt1;
            ahi = c - abig;
            alo = cdxt_adyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as libc::c_int as usize] = _k - bvirt;
            u[3 as libc::c_int as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as libc::c_int,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdztail != 0.0f64 {
                c = splitter * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                _i = cdxt_adyt0 * bdztail;
                c = splitter * cdxt_adyt0;
                abig = c - cdxt_adyt0;
                ahi = c - abig;
                alo = cdxt_adyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as libc::c_int as usize] = alo * blo - err3;
                _j = cdxt_adyt1 * bdztail;
                c = splitter * cdxt_adyt1;
                abig = c - cdxt_adyt1;
                ahi = c - abig;
                alo = cdxt_adyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1 as libc::c_int as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as libc::c_int as usize] = _k - bvirt;
                u[3 as libc::c_int as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as libc::c_int,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
        if bdytail != 0.0f64 {
            negate = -cdxtail;
            cdxt_bdyt1 = negate * bdytail;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = cdxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_bdyt0 = alo * blo - err3;
            c = splitter * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            _i = cdxt_bdyt0 * adz;
            c = splitter * cdxt_bdyt0;
            abig = c - cdxt_bdyt0;
            ahi = c - abig;
            alo = cdxt_bdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as libc::c_int as usize] = alo * blo - err3;
            _j = cdxt_bdyt1 * adz;
            c = splitter * cdxt_bdyt1;
            abig = c - cdxt_bdyt1;
            ahi = c - abig;
            alo = cdxt_bdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as libc::c_int as usize] = _k - bvirt;
            u[3 as libc::c_int as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as libc::c_int,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adztail != 0.0f64 {
                c = splitter * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                _i = cdxt_bdyt0 * adztail;
                c = splitter * cdxt_bdyt0;
                abig = c - cdxt_bdyt0;
                ahi = c - abig;
                alo = cdxt_bdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as libc::c_int as usize] = alo * blo - err3;
                _j = cdxt_bdyt1 * adztail;
                c = splitter * cdxt_bdyt1;
                abig = c - cdxt_bdyt1;
                ahi = c - abig;
                alo = cdxt_bdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1 as libc::c_int as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as libc::c_int as usize] = _k - bvirt;
                u[3 as libc::c_int as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as libc::c_int,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
    }
    if adztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(
            bctlen,
            bct.as_mut_ptr(),
            adztail,
            w.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            wlength,
            w.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if bdztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(
            catlen,
            cat.as_mut_ptr(),
            bdztail,
            w.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            wlength,
            w.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if cdztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(
            abtlen,
            abt.as_mut_ptr(),
            cdztail,
            w.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            wlength,
            w.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    return *finnow.offset((finlength - 1 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn orient3d(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut adz: libc::c_double = 0.;
    let mut bdz: libc::c_double = 0.;
    let mut cdz: libc::c_double = 0.;
    let mut bdxcdy: libc::c_double = 0.;
    let mut cdxbdy: libc::c_double = 0.;
    let mut cdxady: libc::c_double = 0.;
    let mut adxcdy: libc::c_double = 0.;
    let mut adxbdy: libc::c_double = 0.;
    let mut bdxady: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut permanent: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    adz = *pa.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bdz = *pb.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    cdz = *pc.offset(2 as libc::c_int as isize) - *pd.offset(2 as libc::c_int as isize);
    bdxcdy = bdx * cdy;
    cdxbdy = cdx * bdy;
    cdxady = cdx * ady;
    adxcdy = adx * cdy;
    adxbdy = adx * bdy;
    bdxady = bdx * ady;
    det = adz * (bdxcdy - cdxbdy) + bdz * (cdxady - adxcdy) + cdz * (adxbdy - bdxady);
    permanent = ((if bdxcdy >= 0.0f64 { bdxcdy } else { -bdxcdy })
        + (if cdxbdy >= 0.0f64 { cdxbdy } else { -cdxbdy }))
        * (if adz >= 0.0f64 { adz } else { -adz })
        + ((if cdxady >= 0.0f64 { cdxady } else { -cdxady })
            + (if adxcdy >= 0.0f64 { adxcdy } else { -adxcdy }))
            * (if bdz >= 0.0f64 { bdz } else { -bdz })
        + ((if adxbdy >= 0.0f64 { adxbdy } else { -adxbdy })
            + (if bdxady >= 0.0f64 { bdxady } else { -bdxady }))
            * (if cdz >= 0.0f64 { cdz } else { -cdz });
    errbound = o3derrboundA * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    return orient3dadapt(pa, pb, pc, pd, permanent);
}
#[no_mangle]
pub unsafe extern "C" fn incirclefast(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut abdet: libc::c_double = 0.;
    let mut bcdet: libc::c_double = 0.;
    let mut cadet: libc::c_double = 0.;
    let mut alift: libc::c_double = 0.;
    let mut blift: libc::c_double = 0.;
    let mut clift: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    abdet = adx * bdy - bdx * ady;
    bcdet = bdx * cdy - cdx * bdy;
    cadet = cdx * ady - adx * cdy;
    alift = adx * adx + ady * ady;
    blift = bdx * bdx + bdy * bdy;
    clift = cdx * cdx + cdy * cdy;
    return alift * bcdet + blift * cadet + clift * abdet;
}
#[no_mangle]
pub unsafe extern "C" fn incircleexact(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut axby1: libc::c_double = 0.;
    let mut bxcy1: libc::c_double = 0.;
    let mut cxdy1: libc::c_double = 0.;
    let mut dxay1: libc::c_double = 0.;
    let mut axcy1: libc::c_double = 0.;
    let mut bxdy1: libc::c_double = 0.;
    let mut bxay1: libc::c_double = 0.;
    let mut cxby1: libc::c_double = 0.;
    let mut dxcy1: libc::c_double = 0.;
    let mut axdy1: libc::c_double = 0.;
    let mut cxay1: libc::c_double = 0.;
    let mut dxby1: libc::c_double = 0.;
    let mut axby0: libc::c_double = 0.;
    let mut bxcy0: libc::c_double = 0.;
    let mut cxdy0: libc::c_double = 0.;
    let mut dxay0: libc::c_double = 0.;
    let mut axcy0: libc::c_double = 0.;
    let mut bxdy0: libc::c_double = 0.;
    let mut bxay0: libc::c_double = 0.;
    let mut cxby0: libc::c_double = 0.;
    let mut dxcy0: libc::c_double = 0.;
    let mut axdy0: libc::c_double = 0.;
    let mut cxay0: libc::c_double = 0.;
    let mut dxby0: libc::c_double = 0.;
    let mut ab: [libc::c_double; 4] = [0.; 4];
    let mut bc: [libc::c_double; 4] = [0.; 4];
    let mut cd: [libc::c_double; 4] = [0.; 4];
    let mut da: [libc::c_double; 4] = [0.; 4];
    let mut ac: [libc::c_double; 4] = [0.; 4];
    let mut bd: [libc::c_double; 4] = [0.; 4];
    let mut temp8: [libc::c_double; 8] = [0.; 8];
    let mut templen: libc::c_int = 0;
    let mut abc: [libc::c_double; 12] = [0.; 12];
    let mut bcd: [libc::c_double; 12] = [0.; 12];
    let mut cda: [libc::c_double; 12] = [0.; 12];
    let mut dab: [libc::c_double; 12] = [0.; 12];
    let mut abclen: libc::c_int = 0;
    let mut bcdlen: libc::c_int = 0;
    let mut cdalen: libc::c_int = 0;
    let mut dablen: libc::c_int = 0;
    let mut det24x: [libc::c_double; 24] = [0.; 24];
    let mut det24y: [libc::c_double; 24] = [0.; 24];
    let mut det48x: [libc::c_double; 48] = [0.; 48];
    let mut det48y: [libc::c_double; 48] = [0.; 48];
    let mut xlen: libc::c_int = 0;
    let mut ylen: libc::c_int = 0;
    let mut adet: [libc::c_double; 96] = [0.; 96];
    let mut bdet: [libc::c_double; 96] = [0.; 96];
    let mut cdet: [libc::c_double; 96] = [0.; 96];
    let mut ddet: [libc::c_double; 96] = [0.; 96];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 192] = [0.; 192];
    let mut cddet: [libc::c_double; 192] = [0.; 192];
    let mut ablen: libc::c_int = 0;
    let mut cdlen: libc::c_int = 0;
    let mut deter: [libc::c_double; 384] = [0.; 384];
    let mut deterlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    axby1 = *pa.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = axby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axby0 = alo * blo - err3;
    bxay1 = *pb.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxay0 = alo * blo - err3;
    _i = axby0 - bxay0;
    bvirt = axby0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay0;
    around = axby0 - avirt;
    ab[0 as libc::c_int as usize] = around + bround;
    _j = axby1 + _i;
    bvirt = _j - axby1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axby1 - avirt;
    _0 = around + bround;
    _i = _0 - bxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay1;
    around = _0 - avirt;
    ab[1 as libc::c_int as usize] = around + bround;
    ab[3 as libc::c_int as usize] = _j + _i;
    bvirt = ab[3 as libc::c_int as usize] - _j;
    avirt = ab[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as libc::c_int as usize] = around + bround;
    bxcy1 = *pb.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxcy0 = alo * blo - err3;
    cxby1 = *pc.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxby0 = alo * blo - err3;
    _i = bxcy0 - cxby0;
    bvirt = bxcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby0;
    around = bxcy0 - avirt;
    bc[0 as libc::c_int as usize] = around + bround;
    _j = bxcy1 + _i;
    bvirt = _j - bxcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxcy1 - avirt;
    _0 = around + bround;
    _i = _0 - cxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby1;
    around = _0 - avirt;
    bc[1 as libc::c_int as usize] = around + bround;
    bc[3 as libc::c_int as usize] = _j + _i;
    bvirt = bc[3 as libc::c_int as usize] - _j;
    avirt = bc[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as libc::c_int as usize] = around + bround;
    cxdy1 = *pc.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxdy0 = alo * blo - err3;
    dxcy1 = *pd.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxcy0 = alo * blo - err3;
    _i = cxdy0 - dxcy0;
    bvirt = cxdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxcy0;
    around = cxdy0 - avirt;
    cd[0 as libc::c_int as usize] = around + bround;
    _j = cxdy1 + _i;
    bvirt = _j - cxdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cxdy1 - avirt;
    _0 = around + bround;
    _i = _0 - dxcy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxcy1;
    around = _0 - avirt;
    cd[1 as libc::c_int as usize] = around + bround;
    cd[3 as libc::c_int as usize] = _j + _i;
    bvirt = cd[3 as libc::c_int as usize] - _j;
    avirt = cd[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    cd[2 as libc::c_int as usize] = around + bround;
    dxay1 = *pd.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxay0 = alo * blo - err3;
    axdy1 = *pa.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = axdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axdy0 = alo * blo - err3;
    _i = dxay0 - axdy0;
    bvirt = dxay0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axdy0;
    around = dxay0 - avirt;
    da[0 as libc::c_int as usize] = around + bround;
    _j = dxay1 + _i;
    bvirt = _j - dxay1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = dxay1 - avirt;
    _0 = around + bround;
    _i = _0 - axdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axdy1;
    around = _0 - avirt;
    da[1 as libc::c_int as usize] = around + bround;
    da[3 as libc::c_int as usize] = _j + _i;
    bvirt = da[3 as libc::c_int as usize] - _j;
    avirt = da[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    da[2 as libc::c_int as usize] = around + bround;
    axcy1 = *pa.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = axcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axcy0 = alo * blo - err3;
    cxay1 = *pc.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxay0 = alo * blo - err3;
    _i = axcy0 - cxay0;
    bvirt = axcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxay0;
    around = axcy0 - avirt;
    ac[0 as libc::c_int as usize] = around + bround;
    _j = axcy1 + _i;
    bvirt = _j - axcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axcy1 - avirt;
    _0 = around + bround;
    _i = _0 - cxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxay1;
    around = _0 - avirt;
    ac[1 as libc::c_int as usize] = around + bround;
    ac[3 as libc::c_int as usize] = _j + _i;
    bvirt = ac[3 as libc::c_int as usize] - _j;
    avirt = ac[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ac[2 as libc::c_int as usize] = around + bround;
    bxdy1 = *pb.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxdy0 = alo * blo - err3;
    dxby1 = *pd.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxby0 = alo * blo - err3;
    _i = bxdy0 - dxby0;
    bvirt = bxdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxby0;
    around = bxdy0 - avirt;
    bd[0 as libc::c_int as usize] = around + bround;
    _j = bxdy1 + _i;
    bvirt = _j - bxdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxdy1 - avirt;
    _0 = around + bround;
    _i = _0 - dxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxby1;
    around = _0 - avirt;
    bd[1 as libc::c_int as usize] = around + bround;
    bd[3 as libc::c_int as usize] = _j + _i;
    bvirt = bd[3 as libc::c_int as usize] - _j;
    avirt = bd[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bd[2 as libc::c_int as usize] = around + bround;
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        4 as libc::c_int,
        da.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    cdalen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        ac.as_mut_ptr(),
        cda.as_mut_ptr(),
    );
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        4 as libc::c_int,
        ab.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    dablen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        bd.as_mut_ptr(),
        dab.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        bd[i as usize] = -bd[i as usize];
        ac[i as usize] = -ac[i as usize];
        i += 1;
    }
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        4 as libc::c_int,
        bc.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    abclen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        ac.as_mut_ptr(),
        abc.as_mut_ptr(),
    );
    templen = fast_expansion_sum_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        4 as libc::c_int,
        cd.as_mut_ptr(),
        temp8.as_mut_ptr(),
    );
    bcdlen = fast_expansion_sum_zeroelim(
        templen,
        temp8.as_mut_ptr(),
        4 as libc::c_int,
        bd.as_mut_ptr(),
        bcd.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        bcdlen,
        bcd.as_mut_ptr(),
        *pa.offset(0 as libc::c_int as isize),
        det24x.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        det24x.as_mut_ptr(),
        *pa.offset(0 as libc::c_int as isize),
        det48x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        bcdlen,
        bcd.as_mut_ptr(),
        *pa.offset(1 as libc::c_int as isize),
        det24y.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        det24y.as_mut_ptr(),
        *pa.offset(1 as libc::c_int as isize),
        det48y.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        xlen,
        det48x.as_mut_ptr(),
        ylen,
        det48y.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        cdalen,
        cda.as_mut_ptr(),
        *pb.offset(0 as libc::c_int as isize),
        det24x.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        det24x.as_mut_ptr(),
        -*pb.offset(0 as libc::c_int as isize),
        det48x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        cdalen,
        cda.as_mut_ptr(),
        *pb.offset(1 as libc::c_int as isize),
        det24y.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        det24y.as_mut_ptr(),
        -*pb.offset(1 as libc::c_int as isize),
        det48y.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        xlen,
        det48x.as_mut_ptr(),
        ylen,
        det48y.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        dablen,
        dab.as_mut_ptr(),
        *pc.offset(0 as libc::c_int as isize),
        det24x.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        det24x.as_mut_ptr(),
        *pc.offset(0 as libc::c_int as isize),
        det48x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        dablen,
        dab.as_mut_ptr(),
        *pc.offset(1 as libc::c_int as isize),
        det24y.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        det24y.as_mut_ptr(),
        *pc.offset(1 as libc::c_int as isize),
        det48y.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        xlen,
        det48x.as_mut_ptr(),
        ylen,
        det48y.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        abclen,
        abc.as_mut_ptr(),
        *pd.offset(0 as libc::c_int as isize),
        det24x.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        det24x.as_mut_ptr(),
        -*pd.offset(0 as libc::c_int as isize),
        det48x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        abclen,
        abc.as_mut_ptr(),
        *pd.offset(1 as libc::c_int as isize),
        det24y.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        det24y.as_mut_ptr(),
        -*pd.offset(1 as libc::c_int as isize),
        det48y.as_mut_ptr(),
    );
    dlen = fast_expansion_sum_zeroelim(
        xlen,
        det48x.as_mut_ptr(),
        ylen,
        det48y.as_mut_ptr(),
        ddet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    cdlen = fast_expansion_sum_zeroelim(
        clen,
        cdet.as_mut_ptr(),
        dlen,
        ddet.as_mut_ptr(),
        cddet.as_mut_ptr(),
    );
    deterlen = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        cdlen,
        cddet.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn incircleslow(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut adxtail: libc::c_double = 0.;
    let mut bdxtail: libc::c_double = 0.;
    let mut cdxtail: libc::c_double = 0.;
    let mut adytail: libc::c_double = 0.;
    let mut bdytail: libc::c_double = 0.;
    let mut cdytail: libc::c_double = 0.;
    let mut negate: libc::c_double = 0.;
    let mut negatetail: libc::c_double = 0.;
    let mut axby7: libc::c_double = 0.;
    let mut bxcy7: libc::c_double = 0.;
    let mut axcy7: libc::c_double = 0.;
    let mut bxay7: libc::c_double = 0.;
    let mut cxby7: libc::c_double = 0.;
    let mut cxay7: libc::c_double = 0.;
    let mut axby: [libc::c_double; 8] = [0.; 8];
    let mut bxcy: [libc::c_double; 8] = [0.; 8];
    let mut axcy: [libc::c_double; 8] = [0.; 8];
    let mut bxay: [libc::c_double; 8] = [0.; 8];
    let mut cxby: [libc::c_double; 8] = [0.; 8];
    let mut cxay: [libc::c_double; 8] = [0.; 8];
    let mut temp16: [libc::c_double; 16] = [0.; 16];
    let mut temp16len: libc::c_int = 0;
    let mut detx: [libc::c_double; 32] = [0.; 32];
    let mut detxx: [libc::c_double; 64] = [0.; 64];
    let mut detxt: [libc::c_double; 32] = [0.; 32];
    let mut detxxt: [libc::c_double; 64] = [0.; 64];
    let mut detxtxt: [libc::c_double; 64] = [0.; 64];
    let mut xlen: libc::c_int = 0;
    let mut xxlen: libc::c_int = 0;
    let mut xtlen: libc::c_int = 0;
    let mut xxtlen: libc::c_int = 0;
    let mut xtxtlen: libc::c_int = 0;
    let mut x1: [libc::c_double; 128] = [0.; 128];
    let mut x2: [libc::c_double; 192] = [0.; 192];
    let mut x1len: libc::c_int = 0;
    let mut x2len: libc::c_int = 0;
    let mut dety: [libc::c_double; 32] = [0.; 32];
    let mut detyy: [libc::c_double; 64] = [0.; 64];
    let mut detyt: [libc::c_double; 32] = [0.; 32];
    let mut detyyt: [libc::c_double; 64] = [0.; 64];
    let mut detytyt: [libc::c_double; 64] = [0.; 64];
    let mut ylen: libc::c_int = 0;
    let mut yylen: libc::c_int = 0;
    let mut ytlen: libc::c_int = 0;
    let mut yytlen: libc::c_int = 0;
    let mut ytytlen: libc::c_int = 0;
    let mut y1: [libc::c_double; 128] = [0.; 128];
    let mut y2: [libc::c_double; 192] = [0.; 192];
    let mut y1len: libc::c_int = 0;
    let mut y2len: libc::c_int = 0;
    let mut adet: [libc::c_double; 384] = [0.; 384];
    let mut bdet: [libc::c_double; 384] = [0.; 384];
    let mut cdet: [libc::c_double; 384] = [0.; 384];
    let mut abdet: [libc::c_double; 768] = [0.; 768];
    let mut deter: [libc::c_double; 1152] = [0.; 1152];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut ablen: libc::c_int = 0;
    let mut deterlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut a0hi: libc::c_double = 0.;
    let mut a0lo: libc::c_double = 0.;
    let mut a1hi: libc::c_double = 0.;
    let mut a1lo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _k: libc::c_double = 0.;
    let mut _l: libc::c_double = 0.;
    let mut _m: libc::c_double = 0.;
    let mut _n: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    let mut _1: libc::c_double = 0.;
    let mut _2: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bvirt = *pa.offset(0 as libc::c_int as isize) - adx;
    avirt = adx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    adxtail = around + bround;
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bvirt = *pa.offset(1 as libc::c_int as isize) - ady;
    avirt = ady + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    adytail = around + bround;
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bvirt = *pb.offset(0 as libc::c_int as isize) - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bdxtail = around + bround;
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bvirt = *pb.offset(1 as libc::c_int as isize) - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    bdytail = around + bround;
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bvirt = *pc.offset(0 as libc::c_int as isize) - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pc.offset(0 as libc::c_int as isize) - avirt;
    cdxtail = around + bround;
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bvirt = *pc.offset(1 as libc::c_int as isize) - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pc.offset(1 as libc::c_int as isize) - avirt;
    cdytail = around + bround;
    c = splitter * adxtail;
    abig = c - adxtail;
    a0hi = c - abig;
    a0lo = adxtail - a0hi;
    c = splitter * bdytail;
    abig = c - bdytail;
    bhi = c - abig;
    blo = bdytail - bhi;
    _i = adxtail * bdytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * adx;
    abig = c - adx;
    a1hi = c - abig;
    a1lo = adx - a1hi;
    _j = adx * bdytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    _i = adxtail * bdy;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = adx * bdy;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axby[5 as libc::c_int as usize] = around + bround;
    axby7 = _m + _k;
    bvirt = axby7 - _m;
    avirt = axby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axby[6 as libc::c_int as usize] = around + bround;
    axby[7 as libc::c_int as usize] = axby7;
    negate = -ady;
    negatetail = -adytail;
    c = splitter * bdxtail;
    abig = c - bdxtail;
    a0hi = c - abig;
    a0lo = bdxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = bdxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bdx;
    abig = c - bdx;
    a1hi = c - abig;
    a1lo = bdx - a1hi;
    _j = bdx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = bdxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bdx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxay[5 as libc::c_int as usize] = around + bround;
    bxay7 = _m + _k;
    bvirt = bxay7 - _m;
    avirt = bxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxay[6 as libc::c_int as usize] = around + bround;
    bxay[7 as libc::c_int as usize] = bxay7;
    c = splitter * bdxtail;
    abig = c - bdxtail;
    a0hi = c - abig;
    a0lo = bdxtail - a0hi;
    c = splitter * cdytail;
    abig = c - cdytail;
    bhi = c - abig;
    blo = cdytail - bhi;
    _i = bdxtail * cdytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bdx;
    abig = c - bdx;
    a1hi = c - abig;
    a1lo = bdx - a1hi;
    _j = bdx * cdytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    _i = bdxtail * cdy;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bdx * cdy;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxcy[5 as libc::c_int as usize] = around + bround;
    bxcy7 = _m + _k;
    bvirt = bxcy7 - _m;
    avirt = bxcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxcy[6 as libc::c_int as usize] = around + bround;
    bxcy[7 as libc::c_int as usize] = bxcy7;
    negate = -bdy;
    negatetail = -bdytail;
    c = splitter * cdxtail;
    abig = c - cdxtail;
    a0hi = c - abig;
    a0lo = cdxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = cdxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cdx;
    abig = c - cdx;
    a1hi = c - abig;
    a1lo = cdx - a1hi;
    _j = cdx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = cdxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cdx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxby[5 as libc::c_int as usize] = around + bround;
    cxby7 = _m + _k;
    bvirt = cxby7 - _m;
    avirt = cxby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxby[6 as libc::c_int as usize] = around + bround;
    cxby[7 as libc::c_int as usize] = cxby7;
    c = splitter * cdxtail;
    abig = c - cdxtail;
    a0hi = c - abig;
    a0lo = cdxtail - a0hi;
    c = splitter * adytail;
    abig = c - adytail;
    bhi = c - abig;
    blo = adytail - bhi;
    _i = cdxtail * adytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cdx;
    abig = c - cdx;
    a1hi = c - abig;
    a1lo = cdx - a1hi;
    _j = cdx * adytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    _i = cdxtail * ady;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cdx * ady;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxay[5 as libc::c_int as usize] = around + bround;
    cxay7 = _m + _k;
    bvirt = cxay7 - _m;
    avirt = cxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxay[6 as libc::c_int as usize] = around + bround;
    cxay[7 as libc::c_int as usize] = cxay7;
    negate = -cdy;
    negatetail = -cdytail;
    c = splitter * adxtail;
    abig = c - adxtail;
    a0hi = c - abig;
    a0lo = adxtail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = adxtail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * adx;
    abig = c - adx;
    a1hi = c - abig;
    a1lo = adx - a1hi;
    _j = adx * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = adxtail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = adx * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axcy[5 as libc::c_int as usize] = around + bround;
    axcy7 = _m + _k;
    bvirt = axcy7 - _m;
    avirt = axcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axcy[6 as libc::c_int as usize] = around + bround;
    axcy[7 as libc::c_int as usize] = axcy7;
    temp16len = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        bxcy.as_mut_ptr(),
        8 as libc::c_int,
        cxby.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        adx,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), adx, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        adxtail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        adx,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        adxtail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        ady,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), ady, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        adytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        ady,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        adytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        cxay.as_mut_ptr(),
        8 as libc::c_int,
        axcy.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        bdx,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), bdx, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        bdxtail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        bdx,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        bdxtail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        bdy,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), bdy, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        bdytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        bdy,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        bdytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        axby.as_mut_ptr(),
        8 as libc::c_int,
        bxay.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        cdx,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), cdx, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        cdxtail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        cdx,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        cdxtail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        cdy,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), cdy, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp16len,
        temp16.as_mut_ptr(),
        cdytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        cdy,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        cdytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    deterlen = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        clen,
        cdet.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn incircleadapt(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut permanent: libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    let mut bdxcdy1: libc::c_double = 0.;
    let mut cdxbdy1: libc::c_double = 0.;
    let mut cdxady1: libc::c_double = 0.;
    let mut adxcdy1: libc::c_double = 0.;
    let mut adxbdy1: libc::c_double = 0.;
    let mut bdxady1: libc::c_double = 0.;
    let mut bdxcdy0: libc::c_double = 0.;
    let mut cdxbdy0: libc::c_double = 0.;
    let mut cdxady0: libc::c_double = 0.;
    let mut adxcdy0: libc::c_double = 0.;
    let mut adxbdy0: libc::c_double = 0.;
    let mut bdxady0: libc::c_double = 0.;
    let mut bc: [libc::c_double; 4] = [0.; 4];
    let mut ca: [libc::c_double; 4] = [0.; 4];
    let mut ab: [libc::c_double; 4] = [0.; 4];
    let mut bc3: libc::c_double = 0.;
    let mut ca3: libc::c_double = 0.;
    let mut ab3: libc::c_double = 0.;
    let mut axbc: [libc::c_double; 8] = [0.; 8];
    let mut axxbc: [libc::c_double; 16] = [0.; 16];
    let mut aybc: [libc::c_double; 8] = [0.; 8];
    let mut ayybc: [libc::c_double; 16] = [0.; 16];
    let mut adet: [libc::c_double; 32] = [0.; 32];
    let mut axbclen: libc::c_int = 0;
    let mut axxbclen: libc::c_int = 0;
    let mut aybclen: libc::c_int = 0;
    let mut ayybclen: libc::c_int = 0;
    let mut alen: libc::c_int = 0;
    let mut bxca: [libc::c_double; 8] = [0.; 8];
    let mut bxxca: [libc::c_double; 16] = [0.; 16];
    let mut byca: [libc::c_double; 8] = [0.; 8];
    let mut byyca: [libc::c_double; 16] = [0.; 16];
    let mut bdet: [libc::c_double; 32] = [0.; 32];
    let mut bxcalen: libc::c_int = 0;
    let mut bxxcalen: libc::c_int = 0;
    let mut bycalen: libc::c_int = 0;
    let mut byycalen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut cxab: [libc::c_double; 8] = [0.; 8];
    let mut cxxab: [libc::c_double; 16] = [0.; 16];
    let mut cyab: [libc::c_double; 8] = [0.; 8];
    let mut cyyab: [libc::c_double; 16] = [0.; 16];
    let mut cdet: [libc::c_double; 32] = [0.; 32];
    let mut cxablen: libc::c_int = 0;
    let mut cxxablen: libc::c_int = 0;
    let mut cyablen: libc::c_int = 0;
    let mut cyyablen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 64] = [0.; 64];
    let mut ablen: libc::c_int = 0;
    let mut fin1: [libc::c_double; 1152] = [0.; 1152];
    let mut fin2: [libc::c_double; 1152] = [0.; 1152];
    let mut finnow: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut finother: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut finswap: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut finlength: libc::c_int = 0;
    let mut adxtail: libc::c_double = 0.;
    let mut bdxtail: libc::c_double = 0.;
    let mut cdxtail: libc::c_double = 0.;
    let mut adytail: libc::c_double = 0.;
    let mut bdytail: libc::c_double = 0.;
    let mut cdytail: libc::c_double = 0.;
    let mut adxadx1: libc::c_double = 0.;
    let mut adyady1: libc::c_double = 0.;
    let mut bdxbdx1: libc::c_double = 0.;
    let mut bdybdy1: libc::c_double = 0.;
    let mut cdxcdx1: libc::c_double = 0.;
    let mut cdycdy1: libc::c_double = 0.;
    let mut adxadx0: libc::c_double = 0.;
    let mut adyady0: libc::c_double = 0.;
    let mut bdxbdx0: libc::c_double = 0.;
    let mut bdybdy0: libc::c_double = 0.;
    let mut cdxcdx0: libc::c_double = 0.;
    let mut cdycdy0: libc::c_double = 0.;
    let mut aa: [libc::c_double; 4] = [0.; 4];
    let mut bb: [libc::c_double; 4] = [0.; 4];
    let mut cc: [libc::c_double; 4] = [0.; 4];
    let mut aa3: libc::c_double = 0.;
    let mut bb3: libc::c_double = 0.;
    let mut cc3: libc::c_double = 0.;
    let mut ti1: libc::c_double = 0.;
    let mut tj1: libc::c_double = 0.;
    let mut ti0: libc::c_double = 0.;
    let mut tj0: libc::c_double = 0.;
    let mut u: [libc::c_double; 4] = [0.; 4];
    let mut v: [libc::c_double; 4] = [0.; 4];
    let mut u3: libc::c_double = 0.;
    let mut v3: libc::c_double = 0.;
    let mut temp8: [libc::c_double; 8] = [0.; 8];
    let mut temp16a: [libc::c_double; 16] = [0.; 16];
    let mut temp16b: [libc::c_double; 16] = [0.; 16];
    let mut temp16c: [libc::c_double; 16] = [0.; 16];
    let mut temp32a: [libc::c_double; 32] = [0.; 32];
    let mut temp32b: [libc::c_double; 32] = [0.; 32];
    let mut temp48: [libc::c_double; 48] = [0.; 48];
    let mut temp64: [libc::c_double; 64] = [0.; 64];
    let mut temp8len: libc::c_int = 0;
    let mut temp16alen: libc::c_int = 0;
    let mut temp16blen: libc::c_int = 0;
    let mut temp16clen: libc::c_int = 0;
    let mut temp32alen: libc::c_int = 0;
    let mut temp32blen: libc::c_int = 0;
    let mut temp48len: libc::c_int = 0;
    let mut temp64len: libc::c_int = 0;
    let mut axtbb: [libc::c_double; 8] = [0.; 8];
    let mut axtcc: [libc::c_double; 8] = [0.; 8];
    let mut aytbb: [libc::c_double; 8] = [0.; 8];
    let mut aytcc: [libc::c_double; 8] = [0.; 8];
    let mut axtbblen: libc::c_int = 0;
    let mut axtcclen: libc::c_int = 0;
    let mut aytbblen: libc::c_int = 0;
    let mut aytcclen: libc::c_int = 0;
    let mut bxtaa: [libc::c_double; 8] = [0.; 8];
    let mut bxtcc: [libc::c_double; 8] = [0.; 8];
    let mut bytaa: [libc::c_double; 8] = [0.; 8];
    let mut bytcc: [libc::c_double; 8] = [0.; 8];
    let mut bxtaalen: libc::c_int = 0;
    let mut bxtcclen: libc::c_int = 0;
    let mut bytaalen: libc::c_int = 0;
    let mut bytcclen: libc::c_int = 0;
    let mut cxtaa: [libc::c_double; 8] = [0.; 8];
    let mut cxtbb: [libc::c_double; 8] = [0.; 8];
    let mut cytaa: [libc::c_double; 8] = [0.; 8];
    let mut cytbb: [libc::c_double; 8] = [0.; 8];
    let mut cxtaalen: libc::c_int = 0;
    let mut cxtbblen: libc::c_int = 0;
    let mut cytaalen: libc::c_int = 0;
    let mut cytbblen: libc::c_int = 0;
    let mut axtbc: [libc::c_double; 8] = [0.; 8];
    let mut aytbc: [libc::c_double; 8] = [0.; 8];
    let mut bxtca: [libc::c_double; 8] = [0.; 8];
    let mut bytca: [libc::c_double; 8] = [0.; 8];
    let mut cxtab: [libc::c_double; 8] = [0.; 8];
    let mut cytab: [libc::c_double; 8] = [0.; 8];
    let mut axtbclen: libc::c_int = 0;
    let mut aytbclen: libc::c_int = 0;
    let mut bxtcalen: libc::c_int = 0;
    let mut bytcalen: libc::c_int = 0;
    let mut cxtablen: libc::c_int = 0;
    let mut cytablen: libc::c_int = 0;
    let mut axtbct: [libc::c_double; 16] = [0.; 16];
    let mut aytbct: [libc::c_double; 16] = [0.; 16];
    let mut bxtcat: [libc::c_double; 16] = [0.; 16];
    let mut bytcat: [libc::c_double; 16] = [0.; 16];
    let mut cxtabt: [libc::c_double; 16] = [0.; 16];
    let mut cytabt: [libc::c_double; 16] = [0.; 16];
    let mut axtbctlen: libc::c_int = 0;
    let mut aytbctlen: libc::c_int = 0;
    let mut bxtcatlen: libc::c_int = 0;
    let mut bytcatlen: libc::c_int = 0;
    let mut cxtabtlen: libc::c_int = 0;
    let mut cytabtlen: libc::c_int = 0;
    let mut axtbctt: [libc::c_double; 8] = [0.; 8];
    let mut aytbctt: [libc::c_double; 8] = [0.; 8];
    let mut bxtcatt: [libc::c_double; 8] = [0.; 8];
    let mut bytcatt: [libc::c_double; 8] = [0.; 8];
    let mut cxtabtt: [libc::c_double; 8] = [0.; 8];
    let mut cytabtt: [libc::c_double; 8] = [0.; 8];
    let mut axtbcttlen: libc::c_int = 0;
    let mut aytbcttlen: libc::c_int = 0;
    let mut bxtcattlen: libc::c_int = 0;
    let mut bytcattlen: libc::c_int = 0;
    let mut cxtabttlen: libc::c_int = 0;
    let mut cytabttlen: libc::c_int = 0;
    let mut abt: [libc::c_double; 8] = [0.; 8];
    let mut bct: [libc::c_double; 8] = [0.; 8];
    let mut cat: [libc::c_double; 8] = [0.; 8];
    let mut abtlen: libc::c_int = 0;
    let mut bctlen: libc::c_int = 0;
    let mut catlen: libc::c_int = 0;
    let mut abtt: [libc::c_double; 4] = [0.; 4];
    let mut bctt: [libc::c_double; 4] = [0.; 4];
    let mut catt: [libc::c_double; 4] = [0.; 4];
    let mut abttlen: libc::c_int = 0;
    let mut bcttlen: libc::c_int = 0;
    let mut cattlen: libc::c_int = 0;
    let mut abtt3: libc::c_double = 0.;
    let mut bctt3: libc::c_double = 0.;
    let mut catt3: libc::c_double = 0.;
    let mut negate: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdxcdy1 = bdx * cdy;
    c = splitter * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = bdxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bdxcdy0 = alo * blo - err3;
    cdxbdy1 = cdx * bdy;
    c = splitter * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = cdxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cdxbdy0 = alo * blo - err3;
    _i = bdxcdy0 - cdxbdy0;
    bvirt = bdxcdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy0;
    around = bdxcdy0 - avirt;
    bc[0 as libc::c_int as usize] = around + bround;
    _j = bdxcdy1 + _i;
    bvirt = _j - bdxcdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bdxcdy1 - avirt;
    _0 = around + bround;
    _i = _0 - cdxbdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy1;
    around = _0 - avirt;
    bc[1 as libc::c_int as usize] = around + bround;
    bc3 = _j + _i;
    bvirt = bc3 - _j;
    avirt = bc3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as libc::c_int as usize] = around + bround;
    bc[3 as libc::c_int as usize] = bc3;
    axbclen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        adx,
        axbc.as_mut_ptr(),
    );
    axxbclen = scale_expansion_zeroelim(
        axbclen,
        axbc.as_mut_ptr(),
        adx,
        axxbc.as_mut_ptr(),
    );
    aybclen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        ady,
        aybc.as_mut_ptr(),
    );
    ayybclen = scale_expansion_zeroelim(
        aybclen,
        aybc.as_mut_ptr(),
        ady,
        ayybc.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        axxbclen,
        axxbc.as_mut_ptr(),
        ayybclen,
        ayybc.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    cdxady1 = cdx * ady;
    c = splitter * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = cdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cdxady0 = alo * blo - err3;
    adxcdy1 = adx * cdy;
    c = splitter * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = adxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    adxcdy0 = alo * blo - err3;
    _i = cdxady0 - adxcdy0;
    bvirt = cdxady0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy0;
    around = cdxady0 - avirt;
    ca[0 as libc::c_int as usize] = around + bround;
    _j = cdxady1 + _i;
    bvirt = _j - cdxady1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cdxady1 - avirt;
    _0 = around + bround;
    _i = _0 - adxcdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy1;
    around = _0 - avirt;
    ca[1 as libc::c_int as usize] = around + bround;
    ca3 = _j + _i;
    bvirt = ca3 - _j;
    avirt = ca3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ca[2 as libc::c_int as usize] = around + bround;
    ca[3 as libc::c_int as usize] = ca3;
    bxcalen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ca.as_mut_ptr(),
        bdx,
        bxca.as_mut_ptr(),
    );
    bxxcalen = scale_expansion_zeroelim(
        bxcalen,
        bxca.as_mut_ptr(),
        bdx,
        bxxca.as_mut_ptr(),
    );
    bycalen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ca.as_mut_ptr(),
        bdy,
        byca.as_mut_ptr(),
    );
    byycalen = scale_expansion_zeroelim(
        bycalen,
        byca.as_mut_ptr(),
        bdy,
        byyca.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        bxxcalen,
        bxxca.as_mut_ptr(),
        byycalen,
        byyca.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    adxbdy1 = adx * bdy;
    c = splitter * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = adxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    adxbdy0 = alo * blo - err3;
    bdxady1 = bdx * ady;
    c = splitter * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = bdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bdxady0 = alo * blo - err3;
    _i = adxbdy0 - bdxady0;
    bvirt = adxbdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady0;
    around = adxbdy0 - avirt;
    ab[0 as libc::c_int as usize] = around + bround;
    _j = adxbdy1 + _i;
    bvirt = _j - adxbdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = adxbdy1 - avirt;
    _0 = around + bround;
    _i = _0 - bdxady1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady1;
    around = _0 - avirt;
    ab[1 as libc::c_int as usize] = around + bround;
    ab3 = _j + _i;
    bvirt = ab3 - _j;
    avirt = ab3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as libc::c_int as usize] = around + bround;
    ab[3 as libc::c_int as usize] = ab3;
    cxablen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        cdx,
        cxab.as_mut_ptr(),
    );
    cxxablen = scale_expansion_zeroelim(
        cxablen,
        cxab.as_mut_ptr(),
        cdx,
        cxxab.as_mut_ptr(),
    );
    cyablen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        cdy,
        cyab.as_mut_ptr(),
    );
    cyyablen = scale_expansion_zeroelim(
        cyablen,
        cyab.as_mut_ptr(),
        cdy,
        cyyab.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        cxxablen,
        cxxab.as_mut_ptr(),
        cyyablen,
        cyyab.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    finlength = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        clen,
        cdet.as_mut_ptr(),
        fin1.as_mut_ptr(),
    );
    det = estimate(finlength, fin1.as_mut_ptr());
    errbound = iccerrboundB * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = *pa.offset(0 as libc::c_int as isize) - adx;
    avirt = adx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    adxtail = around + bround;
    bvirt = *pa.offset(1 as libc::c_int as isize) - ady;
    avirt = ady + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    adytail = around + bround;
    bvirt = *pb.offset(0 as libc::c_int as isize) - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bdxtail = around + bround;
    bvirt = *pb.offset(1 as libc::c_int as isize) - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    bdytail = around + bround;
    bvirt = *pc.offset(0 as libc::c_int as isize) - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - *pd.offset(0 as libc::c_int as isize);
    around = *pc.offset(0 as libc::c_int as isize) - avirt;
    cdxtail = around + bround;
    bvirt = *pc.offset(1 as libc::c_int as isize) - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - *pd.offset(1 as libc::c_int as isize);
    around = *pc.offset(1 as libc::c_int as isize) - avirt;
    cdytail = around + bround;
    if adxtail == 0.0f64 && bdxtail == 0.0f64 && cdxtail == 0.0f64 && adytail == 0.0f64
        && bdytail == 0.0f64 && cdytail == 0.0f64
    {
        return det;
    }
    errbound = iccerrboundC * permanent
        + resulterrbound * (if det >= 0.0f64 { det } else { -det });
    det
        += (adx * adx + ady * ady)
            * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail))
            + 2.0f64 * (adx * adxtail + ady * adytail) * (bdx * cdy - bdy * cdx)
            + ((bdx * bdx + bdy * bdy)
                * (cdx * adytail + ady * cdxtail - (cdy * adxtail + adx * cdytail))
                + 2.0f64 * (bdx * bdxtail + bdy * bdytail) * (cdx * ady - cdy * adx))
            + ((cdx * cdx + cdy * cdy)
                * (adx * bdytail + bdy * adxtail - (ady * bdxtail + bdx * adytail))
                + 2.0f64 * (cdx * cdxtail + cdy * cdytail) * (adx * bdy - ady * bdx));
    if det >= errbound || -det >= errbound {
        return det;
    }
    finnow = fin1.as_mut_ptr();
    finother = fin2.as_mut_ptr();
    if bdxtail != 0.0f64 || bdytail != 0.0f64 || cdxtail != 0.0f64 || cdytail != 0.0f64 {
        adxadx1 = adx * adx;
        c = splitter * adx;
        abig = c - adx;
        ahi = c - abig;
        alo = adx - ahi;
        err1 = adxadx1 - ahi * ahi;
        err3 = err1 - (ahi + ahi) * alo;
        adxadx0 = alo * alo - err3;
        adyady1 = ady * ady;
        c = splitter * ady;
        abig = c - ady;
        ahi = c - abig;
        alo = ady - ahi;
        err1 = adyady1 - ahi * ahi;
        err3 = err1 - (ahi + ahi) * alo;
        adyady0 = alo * alo - err3;
        _i = adxadx0 + adyady0;
        bvirt = _i - adxadx0;
        avirt = _i - bvirt;
        bround = adyady0 - bvirt;
        around = adxadx0 - avirt;
        aa[0 as libc::c_int as usize] = around + bround;
        _j = adxadx1 + _i;
        bvirt = _j - adxadx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = adxadx1 - avirt;
        _0 = around + bround;
        _i = _0 + adyady1;
        bvirt = _i - _0;
        avirt = _i - bvirt;
        bround = adyady1 - bvirt;
        around = _0 - avirt;
        aa[1 as libc::c_int as usize] = around + bround;
        aa3 = _j + _i;
        bvirt = aa3 - _j;
        avirt = aa3 - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        aa[2 as libc::c_int as usize] = around + bround;
        aa[3 as libc::c_int as usize] = aa3;
    }
    if cdxtail != 0.0f64 || cdytail != 0.0f64 || adxtail != 0.0f64 || adytail != 0.0f64 {
        bdxbdx1 = bdx * bdx;
        c = splitter * bdx;
        abig = c - bdx;
        ahi = c - abig;
        alo = bdx - ahi;
        err1 = bdxbdx1 - ahi * ahi;
        err3 = err1 - (ahi + ahi) * alo;
        bdxbdx0 = alo * alo - err3;
        bdybdy1 = bdy * bdy;
        c = splitter * bdy;
        abig = c - bdy;
        ahi = c - abig;
        alo = bdy - ahi;
        err1 = bdybdy1 - ahi * ahi;
        err3 = err1 - (ahi + ahi) * alo;
        bdybdy0 = alo * alo - err3;
        _i = bdxbdx0 + bdybdy0;
        bvirt = _i - bdxbdx0;
        avirt = _i - bvirt;
        bround = bdybdy0 - bvirt;
        around = bdxbdx0 - avirt;
        bb[0 as libc::c_int as usize] = around + bround;
        _j = bdxbdx1 + _i;
        bvirt = _j - bdxbdx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = bdxbdx1 - avirt;
        _0 = around + bround;
        _i = _0 + bdybdy1;
        bvirt = _i - _0;
        avirt = _i - bvirt;
        bround = bdybdy1 - bvirt;
        around = _0 - avirt;
        bb[1 as libc::c_int as usize] = around + bround;
        bb3 = _j + _i;
        bvirt = bb3 - _j;
        avirt = bb3 - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bb[2 as libc::c_int as usize] = around + bround;
        bb[3 as libc::c_int as usize] = bb3;
    }
    if adxtail != 0.0f64 || adytail != 0.0f64 || bdxtail != 0.0f64 || bdytail != 0.0f64 {
        cdxcdx1 = cdx * cdx;
        c = splitter * cdx;
        abig = c - cdx;
        ahi = c - abig;
        alo = cdx - ahi;
        err1 = cdxcdx1 - ahi * ahi;
        err3 = err1 - (ahi + ahi) * alo;
        cdxcdx0 = alo * alo - err3;
        cdycdy1 = cdy * cdy;
        c = splitter * cdy;
        abig = c - cdy;
        ahi = c - abig;
        alo = cdy - ahi;
        err1 = cdycdy1 - ahi * ahi;
        err3 = err1 - (ahi + ahi) * alo;
        cdycdy0 = alo * alo - err3;
        _i = cdxcdx0 + cdycdy0;
        bvirt = _i - cdxcdx0;
        avirt = _i - bvirt;
        bround = cdycdy0 - bvirt;
        around = cdxcdx0 - avirt;
        cc[0 as libc::c_int as usize] = around + bround;
        _j = cdxcdx1 + _i;
        bvirt = _j - cdxcdx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = cdxcdx1 - avirt;
        _0 = around + bround;
        _i = _0 + cdycdy1;
        bvirt = _i - _0;
        avirt = _i - bvirt;
        bround = cdycdy1 - bvirt;
        around = _0 - avirt;
        cc[1 as libc::c_int as usize] = around + bround;
        cc3 = _j + _i;
        bvirt = cc3 - _j;
        avirt = cc3 - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        cc[2 as libc::c_int as usize] = around + bround;
        cc[3 as libc::c_int as usize] = cc3;
    }
    if adxtail != 0.0f64 {
        axtbclen = scale_expansion_zeroelim(
            4 as libc::c_int,
            bc.as_mut_ptr(),
            adxtail,
            axtbc.as_mut_ptr(),
        );
        temp16alen = scale_expansion_zeroelim(
            axtbclen,
            axtbc.as_mut_ptr(),
            2.0f64 * adx,
            temp16a.as_mut_ptr(),
        );
        axtcclen = scale_expansion_zeroelim(
            4 as libc::c_int,
            cc.as_mut_ptr(),
            adxtail,
            axtcc.as_mut_ptr(),
        );
        temp16blen = scale_expansion_zeroelim(
            axtcclen,
            axtcc.as_mut_ptr(),
            bdy,
            temp16b.as_mut_ptr(),
        );
        axtbblen = scale_expansion_zeroelim(
            4 as libc::c_int,
            bb.as_mut_ptr(),
            adxtail,
            axtbb.as_mut_ptr(),
        );
        temp16clen = scale_expansion_zeroelim(
            axtbblen,
            axtbb.as_mut_ptr(),
            -cdy,
            temp16c.as_mut_ptr(),
        );
        temp32alen = fast_expansion_sum_zeroelim(
            temp16alen,
            temp16a.as_mut_ptr(),
            temp16blen,
            temp16b.as_mut_ptr(),
            temp32a.as_mut_ptr(),
        );
        temp48len = fast_expansion_sum_zeroelim(
            temp16clen,
            temp16c.as_mut_ptr(),
            temp32alen,
            temp32a.as_mut_ptr(),
            temp48.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            temp48len,
            temp48.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if adytail != 0.0f64 {
        aytbclen = scale_expansion_zeroelim(
            4 as libc::c_int,
            bc.as_mut_ptr(),
            adytail,
            aytbc.as_mut_ptr(),
        );
        temp16alen = scale_expansion_zeroelim(
            aytbclen,
            aytbc.as_mut_ptr(),
            2.0f64 * ady,
            temp16a.as_mut_ptr(),
        );
        aytbblen = scale_expansion_zeroelim(
            4 as libc::c_int,
            bb.as_mut_ptr(),
            adytail,
            aytbb.as_mut_ptr(),
        );
        temp16blen = scale_expansion_zeroelim(
            aytbblen,
            aytbb.as_mut_ptr(),
            cdx,
            temp16b.as_mut_ptr(),
        );
        aytcclen = scale_expansion_zeroelim(
            4 as libc::c_int,
            cc.as_mut_ptr(),
            adytail,
            aytcc.as_mut_ptr(),
        );
        temp16clen = scale_expansion_zeroelim(
            aytcclen,
            aytcc.as_mut_ptr(),
            -bdx,
            temp16c.as_mut_ptr(),
        );
        temp32alen = fast_expansion_sum_zeroelim(
            temp16alen,
            temp16a.as_mut_ptr(),
            temp16blen,
            temp16b.as_mut_ptr(),
            temp32a.as_mut_ptr(),
        );
        temp48len = fast_expansion_sum_zeroelim(
            temp16clen,
            temp16c.as_mut_ptr(),
            temp32alen,
            temp32a.as_mut_ptr(),
            temp48.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            temp48len,
            temp48.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if bdxtail != 0.0f64 {
        bxtcalen = scale_expansion_zeroelim(
            4 as libc::c_int,
            ca.as_mut_ptr(),
            bdxtail,
            bxtca.as_mut_ptr(),
        );
        temp16alen = scale_expansion_zeroelim(
            bxtcalen,
            bxtca.as_mut_ptr(),
            2.0f64 * bdx,
            temp16a.as_mut_ptr(),
        );
        bxtaalen = scale_expansion_zeroelim(
            4 as libc::c_int,
            aa.as_mut_ptr(),
            bdxtail,
            bxtaa.as_mut_ptr(),
        );
        temp16blen = scale_expansion_zeroelim(
            bxtaalen,
            bxtaa.as_mut_ptr(),
            cdy,
            temp16b.as_mut_ptr(),
        );
        bxtcclen = scale_expansion_zeroelim(
            4 as libc::c_int,
            cc.as_mut_ptr(),
            bdxtail,
            bxtcc.as_mut_ptr(),
        );
        temp16clen = scale_expansion_zeroelim(
            bxtcclen,
            bxtcc.as_mut_ptr(),
            -ady,
            temp16c.as_mut_ptr(),
        );
        temp32alen = fast_expansion_sum_zeroelim(
            temp16alen,
            temp16a.as_mut_ptr(),
            temp16blen,
            temp16b.as_mut_ptr(),
            temp32a.as_mut_ptr(),
        );
        temp48len = fast_expansion_sum_zeroelim(
            temp16clen,
            temp16c.as_mut_ptr(),
            temp32alen,
            temp32a.as_mut_ptr(),
            temp48.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            temp48len,
            temp48.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if bdytail != 0.0f64 {
        bytcalen = scale_expansion_zeroelim(
            4 as libc::c_int,
            ca.as_mut_ptr(),
            bdytail,
            bytca.as_mut_ptr(),
        );
        temp16alen = scale_expansion_zeroelim(
            bytcalen,
            bytca.as_mut_ptr(),
            2.0f64 * bdy,
            temp16a.as_mut_ptr(),
        );
        bytcclen = scale_expansion_zeroelim(
            4 as libc::c_int,
            cc.as_mut_ptr(),
            bdytail,
            bytcc.as_mut_ptr(),
        );
        temp16blen = scale_expansion_zeroelim(
            bytcclen,
            bytcc.as_mut_ptr(),
            adx,
            temp16b.as_mut_ptr(),
        );
        bytaalen = scale_expansion_zeroelim(
            4 as libc::c_int,
            aa.as_mut_ptr(),
            bdytail,
            bytaa.as_mut_ptr(),
        );
        temp16clen = scale_expansion_zeroelim(
            bytaalen,
            bytaa.as_mut_ptr(),
            -cdx,
            temp16c.as_mut_ptr(),
        );
        temp32alen = fast_expansion_sum_zeroelim(
            temp16alen,
            temp16a.as_mut_ptr(),
            temp16blen,
            temp16b.as_mut_ptr(),
            temp32a.as_mut_ptr(),
        );
        temp48len = fast_expansion_sum_zeroelim(
            temp16clen,
            temp16c.as_mut_ptr(),
            temp32alen,
            temp32a.as_mut_ptr(),
            temp48.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            temp48len,
            temp48.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if cdxtail != 0.0f64 {
        cxtablen = scale_expansion_zeroelim(
            4 as libc::c_int,
            ab.as_mut_ptr(),
            cdxtail,
            cxtab.as_mut_ptr(),
        );
        temp16alen = scale_expansion_zeroelim(
            cxtablen,
            cxtab.as_mut_ptr(),
            2.0f64 * cdx,
            temp16a.as_mut_ptr(),
        );
        cxtbblen = scale_expansion_zeroelim(
            4 as libc::c_int,
            bb.as_mut_ptr(),
            cdxtail,
            cxtbb.as_mut_ptr(),
        );
        temp16blen = scale_expansion_zeroelim(
            cxtbblen,
            cxtbb.as_mut_ptr(),
            ady,
            temp16b.as_mut_ptr(),
        );
        cxtaalen = scale_expansion_zeroelim(
            4 as libc::c_int,
            aa.as_mut_ptr(),
            cdxtail,
            cxtaa.as_mut_ptr(),
        );
        temp16clen = scale_expansion_zeroelim(
            cxtaalen,
            cxtaa.as_mut_ptr(),
            -bdy,
            temp16c.as_mut_ptr(),
        );
        temp32alen = fast_expansion_sum_zeroelim(
            temp16alen,
            temp16a.as_mut_ptr(),
            temp16blen,
            temp16b.as_mut_ptr(),
            temp32a.as_mut_ptr(),
        );
        temp48len = fast_expansion_sum_zeroelim(
            temp16clen,
            temp16c.as_mut_ptr(),
            temp32alen,
            temp32a.as_mut_ptr(),
            temp48.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            temp48len,
            temp48.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if cdytail != 0.0f64 {
        cytablen = scale_expansion_zeroelim(
            4 as libc::c_int,
            ab.as_mut_ptr(),
            cdytail,
            cytab.as_mut_ptr(),
        );
        temp16alen = scale_expansion_zeroelim(
            cytablen,
            cytab.as_mut_ptr(),
            2.0f64 * cdy,
            temp16a.as_mut_ptr(),
        );
        cytaalen = scale_expansion_zeroelim(
            4 as libc::c_int,
            aa.as_mut_ptr(),
            cdytail,
            cytaa.as_mut_ptr(),
        );
        temp16blen = scale_expansion_zeroelim(
            cytaalen,
            cytaa.as_mut_ptr(),
            bdx,
            temp16b.as_mut_ptr(),
        );
        cytbblen = scale_expansion_zeroelim(
            4 as libc::c_int,
            bb.as_mut_ptr(),
            cdytail,
            cytbb.as_mut_ptr(),
        );
        temp16clen = scale_expansion_zeroelim(
            cytbblen,
            cytbb.as_mut_ptr(),
            -adx,
            temp16c.as_mut_ptr(),
        );
        temp32alen = fast_expansion_sum_zeroelim(
            temp16alen,
            temp16a.as_mut_ptr(),
            temp16blen,
            temp16b.as_mut_ptr(),
            temp32a.as_mut_ptr(),
        );
        temp48len = fast_expansion_sum_zeroelim(
            temp16clen,
            temp16c.as_mut_ptr(),
            temp32alen,
            temp32a.as_mut_ptr(),
            temp48.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            temp48len,
            temp48.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if adxtail != 0.0f64 || adytail != 0.0f64 {
        if bdxtail != 0.0f64 || bdytail != 0.0f64 || cdxtail != 0.0f64
            || cdytail != 0.0f64
        {
            ti1 = bdxtail * cdy;
            c = splitter * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = splitter * cdy;
            abig = c - cdy;
            bhi = c - abig;
            blo = cdy - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            tj1 = bdx * cdytail;
            c = splitter * bdx;
            abig = c - bdx;
            ahi = c - abig;
            alo = bdx - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 + tj0;
            bvirt = _i - ti0;
            avirt = _i - bvirt;
            bround = tj0 - bvirt;
            around = ti0 - avirt;
            u[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 + tj1;
            bvirt = _i - _0;
            avirt = _i - bvirt;
            bround = tj1 - bvirt;
            around = _0 - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _i;
            bvirt = u3 - _j;
            avirt = u3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            u[2 as libc::c_int as usize] = around + bround;
            u[3 as libc::c_int as usize] = u3;
            negate = -bdy;
            ti1 = cdxtail * negate;
            c = splitter * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = splitter * negate;
            abig = c - negate;
            bhi = c - abig;
            blo = negate - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            negate = -bdytail;
            tj1 = cdx * negate;
            c = splitter * cdx;
            abig = c - cdx;
            ahi = c - abig;
            alo = cdx - ahi;
            c = splitter * negate;
            abig = c - negate;
            bhi = c - abig;
            blo = negate - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 + tj0;
            bvirt = _i - ti0;
            avirt = _i - bvirt;
            bround = tj0 - bvirt;
            around = ti0 - avirt;
            v[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 + tj1;
            bvirt = _i - _0;
            avirt = _i - bvirt;
            bround = tj1 - bvirt;
            around = _0 - avirt;
            v[1 as libc::c_int as usize] = around + bround;
            v3 = _j + _i;
            bvirt = v3 - _j;
            avirt = v3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            v[2 as libc::c_int as usize] = around + bround;
            v[3 as libc::c_int as usize] = v3;
            bctlen = fast_expansion_sum_zeroelim(
                4 as libc::c_int,
                u.as_mut_ptr(),
                4 as libc::c_int,
                v.as_mut_ptr(),
                bct.as_mut_ptr(),
            );
            ti1 = bdxtail * cdytail;
            c = splitter * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            tj1 = cdxtail * bdytail;
            c = splitter * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 - tj0;
            bvirt = ti0 - _i;
            avirt = _i + bvirt;
            bround = bvirt - tj0;
            around = ti0 - avirt;
            bctt[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 - tj1;
            bvirt = _0 - _i;
            avirt = _i + bvirt;
            bround = bvirt - tj1;
            around = _0 - avirt;
            bctt[1 as libc::c_int as usize] = around + bround;
            bctt3 = _j + _i;
            bvirt = bctt3 - _j;
            avirt = bctt3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            bctt[2 as libc::c_int as usize] = around + bround;
            bctt[3 as libc::c_int as usize] = bctt3;
            bcttlen = 4 as libc::c_int;
        } else {
            bct[0 as libc::c_int as usize] = 0.0f64;
            bctlen = 1 as libc::c_int;
            bctt[0 as libc::c_int as usize] = 0.0f64;
            bcttlen = 1 as libc::c_int;
        }
        if adxtail != 0.0f64 {
            temp16alen = scale_expansion_zeroelim(
                axtbclen,
                axtbc.as_mut_ptr(),
                adxtail,
                temp16a.as_mut_ptr(),
            );
            axtbctlen = scale_expansion_zeroelim(
                bctlen,
                bct.as_mut_ptr(),
                adxtail,
                axtbct.as_mut_ptr(),
            );
            temp32alen = scale_expansion_zeroelim(
                axtbctlen,
                axtbct.as_mut_ptr(),
                2.0f64 * adx,
                temp32a.as_mut_ptr(),
            );
            temp48len = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp32alen,
                temp32a.as_mut_ptr(),
                temp48.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp48len,
                temp48.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdytail != 0.0f64 {
                temp8len = scale_expansion_zeroelim(
                    4 as libc::c_int,
                    cc.as_mut_ptr(),
                    adxtail,
                    temp8.as_mut_ptr(),
                );
                temp16alen = scale_expansion_zeroelim(
                    temp8len,
                    temp8.as_mut_ptr(),
                    bdytail,
                    temp16a.as_mut_ptr(),
                );
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    temp16alen,
                    temp16a.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
            if cdytail != 0.0f64 {
                temp8len = scale_expansion_zeroelim(
                    4 as libc::c_int,
                    bb.as_mut_ptr(),
                    -adxtail,
                    temp8.as_mut_ptr(),
                );
                temp16alen = scale_expansion_zeroelim(
                    temp8len,
                    temp8.as_mut_ptr(),
                    cdytail,
                    temp16a.as_mut_ptr(),
                );
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    temp16alen,
                    temp16a.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
            temp32alen = scale_expansion_zeroelim(
                axtbctlen,
                axtbct.as_mut_ptr(),
                adxtail,
                temp32a.as_mut_ptr(),
            );
            axtbcttlen = scale_expansion_zeroelim(
                bcttlen,
                bctt.as_mut_ptr(),
                adxtail,
                axtbctt.as_mut_ptr(),
            );
            temp16alen = scale_expansion_zeroelim(
                axtbcttlen,
                axtbctt.as_mut_ptr(),
                2.0f64 * adx,
                temp16a.as_mut_ptr(),
            );
            temp16blen = scale_expansion_zeroelim(
                axtbcttlen,
                axtbctt.as_mut_ptr(),
                adxtail,
                temp16b.as_mut_ptr(),
            );
            temp32blen = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp16blen,
                temp16b.as_mut_ptr(),
                temp32b.as_mut_ptr(),
            );
            temp64len = fast_expansion_sum_zeroelim(
                temp32alen,
                temp32a.as_mut_ptr(),
                temp32blen,
                temp32b.as_mut_ptr(),
                temp64.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp64len,
                temp64.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
        }
        if adytail != 0.0f64 {
            temp16alen = scale_expansion_zeroelim(
                aytbclen,
                aytbc.as_mut_ptr(),
                adytail,
                temp16a.as_mut_ptr(),
            );
            aytbctlen = scale_expansion_zeroelim(
                bctlen,
                bct.as_mut_ptr(),
                adytail,
                aytbct.as_mut_ptr(),
            );
            temp32alen = scale_expansion_zeroelim(
                aytbctlen,
                aytbct.as_mut_ptr(),
                2.0f64 * ady,
                temp32a.as_mut_ptr(),
            );
            temp48len = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp32alen,
                temp32a.as_mut_ptr(),
                temp48.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp48len,
                temp48.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            temp32alen = scale_expansion_zeroelim(
                aytbctlen,
                aytbct.as_mut_ptr(),
                adytail,
                temp32a.as_mut_ptr(),
            );
            aytbcttlen = scale_expansion_zeroelim(
                bcttlen,
                bctt.as_mut_ptr(),
                adytail,
                aytbctt.as_mut_ptr(),
            );
            temp16alen = scale_expansion_zeroelim(
                aytbcttlen,
                aytbctt.as_mut_ptr(),
                2.0f64 * ady,
                temp16a.as_mut_ptr(),
            );
            temp16blen = scale_expansion_zeroelim(
                aytbcttlen,
                aytbctt.as_mut_ptr(),
                adytail,
                temp16b.as_mut_ptr(),
            );
            temp32blen = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp16blen,
                temp16b.as_mut_ptr(),
                temp32b.as_mut_ptr(),
            );
            temp64len = fast_expansion_sum_zeroelim(
                temp32alen,
                temp32a.as_mut_ptr(),
                temp32blen,
                temp32b.as_mut_ptr(),
                temp64.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp64len,
                temp64.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
        }
    }
    if bdxtail != 0.0f64 || bdytail != 0.0f64 {
        if cdxtail != 0.0f64 || cdytail != 0.0f64 || adxtail != 0.0f64
            || adytail != 0.0f64
        {
            ti1 = cdxtail * ady;
            c = splitter * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = splitter * ady;
            abig = c - ady;
            bhi = c - abig;
            blo = ady - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            tj1 = cdx * adytail;
            c = splitter * cdx;
            abig = c - cdx;
            ahi = c - abig;
            alo = cdx - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 + tj0;
            bvirt = _i - ti0;
            avirt = _i - bvirt;
            bround = tj0 - bvirt;
            around = ti0 - avirt;
            u[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 + tj1;
            bvirt = _i - _0;
            avirt = _i - bvirt;
            bround = tj1 - bvirt;
            around = _0 - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _i;
            bvirt = u3 - _j;
            avirt = u3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            u[2 as libc::c_int as usize] = around + bround;
            u[3 as libc::c_int as usize] = u3;
            negate = -cdy;
            ti1 = adxtail * negate;
            c = splitter * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = splitter * negate;
            abig = c - negate;
            bhi = c - abig;
            blo = negate - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            negate = -cdytail;
            tj1 = adx * negate;
            c = splitter * adx;
            abig = c - adx;
            ahi = c - abig;
            alo = adx - ahi;
            c = splitter * negate;
            abig = c - negate;
            bhi = c - abig;
            blo = negate - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 + tj0;
            bvirt = _i - ti0;
            avirt = _i - bvirt;
            bround = tj0 - bvirt;
            around = ti0 - avirt;
            v[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 + tj1;
            bvirt = _i - _0;
            avirt = _i - bvirt;
            bround = tj1 - bvirt;
            around = _0 - avirt;
            v[1 as libc::c_int as usize] = around + bround;
            v3 = _j + _i;
            bvirt = v3 - _j;
            avirt = v3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            v[2 as libc::c_int as usize] = around + bround;
            v[3 as libc::c_int as usize] = v3;
            catlen = fast_expansion_sum_zeroelim(
                4 as libc::c_int,
                u.as_mut_ptr(),
                4 as libc::c_int,
                v.as_mut_ptr(),
                cat.as_mut_ptr(),
            );
            ti1 = cdxtail * adytail;
            c = splitter * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            tj1 = adxtail * cdytail;
            c = splitter * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 - tj0;
            bvirt = ti0 - _i;
            avirt = _i + bvirt;
            bround = bvirt - tj0;
            around = ti0 - avirt;
            catt[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 - tj1;
            bvirt = _0 - _i;
            avirt = _i + bvirt;
            bround = bvirt - tj1;
            around = _0 - avirt;
            catt[1 as libc::c_int as usize] = around + bround;
            catt3 = _j + _i;
            bvirt = catt3 - _j;
            avirt = catt3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            catt[2 as libc::c_int as usize] = around + bround;
            catt[3 as libc::c_int as usize] = catt3;
            cattlen = 4 as libc::c_int;
        } else {
            cat[0 as libc::c_int as usize] = 0.0f64;
            catlen = 1 as libc::c_int;
            catt[0 as libc::c_int as usize] = 0.0f64;
            cattlen = 1 as libc::c_int;
        }
        if bdxtail != 0.0f64 {
            temp16alen = scale_expansion_zeroelim(
                bxtcalen,
                bxtca.as_mut_ptr(),
                bdxtail,
                temp16a.as_mut_ptr(),
            );
            bxtcatlen = scale_expansion_zeroelim(
                catlen,
                cat.as_mut_ptr(),
                bdxtail,
                bxtcat.as_mut_ptr(),
            );
            temp32alen = scale_expansion_zeroelim(
                bxtcatlen,
                bxtcat.as_mut_ptr(),
                2.0f64 * bdx,
                temp32a.as_mut_ptr(),
            );
            temp48len = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp32alen,
                temp32a.as_mut_ptr(),
                temp48.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp48len,
                temp48.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdytail != 0.0f64 {
                temp8len = scale_expansion_zeroelim(
                    4 as libc::c_int,
                    aa.as_mut_ptr(),
                    bdxtail,
                    temp8.as_mut_ptr(),
                );
                temp16alen = scale_expansion_zeroelim(
                    temp8len,
                    temp8.as_mut_ptr(),
                    cdytail,
                    temp16a.as_mut_ptr(),
                );
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    temp16alen,
                    temp16a.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
            if adytail != 0.0f64 {
                temp8len = scale_expansion_zeroelim(
                    4 as libc::c_int,
                    cc.as_mut_ptr(),
                    -bdxtail,
                    temp8.as_mut_ptr(),
                );
                temp16alen = scale_expansion_zeroelim(
                    temp8len,
                    temp8.as_mut_ptr(),
                    adytail,
                    temp16a.as_mut_ptr(),
                );
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    temp16alen,
                    temp16a.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
            temp32alen = scale_expansion_zeroelim(
                bxtcatlen,
                bxtcat.as_mut_ptr(),
                bdxtail,
                temp32a.as_mut_ptr(),
            );
            bxtcattlen = scale_expansion_zeroelim(
                cattlen,
                catt.as_mut_ptr(),
                bdxtail,
                bxtcatt.as_mut_ptr(),
            );
            temp16alen = scale_expansion_zeroelim(
                bxtcattlen,
                bxtcatt.as_mut_ptr(),
                2.0f64 * bdx,
                temp16a.as_mut_ptr(),
            );
            temp16blen = scale_expansion_zeroelim(
                bxtcattlen,
                bxtcatt.as_mut_ptr(),
                bdxtail,
                temp16b.as_mut_ptr(),
            );
            temp32blen = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp16blen,
                temp16b.as_mut_ptr(),
                temp32b.as_mut_ptr(),
            );
            temp64len = fast_expansion_sum_zeroelim(
                temp32alen,
                temp32a.as_mut_ptr(),
                temp32blen,
                temp32b.as_mut_ptr(),
                temp64.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp64len,
                temp64.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
        }
        if bdytail != 0.0f64 {
            temp16alen = scale_expansion_zeroelim(
                bytcalen,
                bytca.as_mut_ptr(),
                bdytail,
                temp16a.as_mut_ptr(),
            );
            bytcatlen = scale_expansion_zeroelim(
                catlen,
                cat.as_mut_ptr(),
                bdytail,
                bytcat.as_mut_ptr(),
            );
            temp32alen = scale_expansion_zeroelim(
                bytcatlen,
                bytcat.as_mut_ptr(),
                2.0f64 * bdy,
                temp32a.as_mut_ptr(),
            );
            temp48len = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp32alen,
                temp32a.as_mut_ptr(),
                temp48.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp48len,
                temp48.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            temp32alen = scale_expansion_zeroelim(
                bytcatlen,
                bytcat.as_mut_ptr(),
                bdytail,
                temp32a.as_mut_ptr(),
            );
            bytcattlen = scale_expansion_zeroelim(
                cattlen,
                catt.as_mut_ptr(),
                bdytail,
                bytcatt.as_mut_ptr(),
            );
            temp16alen = scale_expansion_zeroelim(
                bytcattlen,
                bytcatt.as_mut_ptr(),
                2.0f64 * bdy,
                temp16a.as_mut_ptr(),
            );
            temp16blen = scale_expansion_zeroelim(
                bytcattlen,
                bytcatt.as_mut_ptr(),
                bdytail,
                temp16b.as_mut_ptr(),
            );
            temp32blen = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp16blen,
                temp16b.as_mut_ptr(),
                temp32b.as_mut_ptr(),
            );
            temp64len = fast_expansion_sum_zeroelim(
                temp32alen,
                temp32a.as_mut_ptr(),
                temp32blen,
                temp32b.as_mut_ptr(),
                temp64.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp64len,
                temp64.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
        }
    }
    if cdxtail != 0.0f64 || cdytail != 0.0f64 {
        if adxtail != 0.0f64 || adytail != 0.0f64 || bdxtail != 0.0f64
            || bdytail != 0.0f64
        {
            ti1 = adxtail * bdy;
            c = splitter * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = splitter * bdy;
            abig = c - bdy;
            bhi = c - abig;
            blo = bdy - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            tj1 = adx * bdytail;
            c = splitter * adx;
            abig = c - adx;
            ahi = c - abig;
            alo = adx - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 + tj0;
            bvirt = _i - ti0;
            avirt = _i - bvirt;
            bround = tj0 - bvirt;
            around = ti0 - avirt;
            u[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 + tj1;
            bvirt = _i - _0;
            avirt = _i - bvirt;
            bround = tj1 - bvirt;
            around = _0 - avirt;
            u[1 as libc::c_int as usize] = around + bround;
            u3 = _j + _i;
            bvirt = u3 - _j;
            avirt = u3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            u[2 as libc::c_int as usize] = around + bround;
            u[3 as libc::c_int as usize] = u3;
            negate = -ady;
            ti1 = bdxtail * negate;
            c = splitter * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = splitter * negate;
            abig = c - negate;
            bhi = c - abig;
            blo = negate - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            negate = -adytail;
            tj1 = bdx * negate;
            c = splitter * bdx;
            abig = c - bdx;
            ahi = c - abig;
            alo = bdx - ahi;
            c = splitter * negate;
            abig = c - negate;
            bhi = c - abig;
            blo = negate - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 + tj0;
            bvirt = _i - ti0;
            avirt = _i - bvirt;
            bround = tj0 - bvirt;
            around = ti0 - avirt;
            v[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 + tj1;
            bvirt = _i - _0;
            avirt = _i - bvirt;
            bround = tj1 - bvirt;
            around = _0 - avirt;
            v[1 as libc::c_int as usize] = around + bround;
            v3 = _j + _i;
            bvirt = v3 - _j;
            avirt = v3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            v[2 as libc::c_int as usize] = around + bround;
            v[3 as libc::c_int as usize] = v3;
            abtlen = fast_expansion_sum_zeroelim(
                4 as libc::c_int,
                u.as_mut_ptr(),
                4 as libc::c_int,
                v.as_mut_ptr(),
                abt.as_mut_ptr(),
            );
            ti1 = adxtail * bdytail;
            c = splitter * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = ti1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ti0 = alo * blo - err3;
            tj1 = bdxtail * adytail;
            c = splitter * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = tj1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            tj0 = alo * blo - err3;
            _i = ti0 - tj0;
            bvirt = ti0 - _i;
            avirt = _i + bvirt;
            bround = bvirt - tj0;
            around = ti0 - avirt;
            abtt[0 as libc::c_int as usize] = around + bround;
            _j = ti1 + _i;
            bvirt = _j - ti1;
            avirt = _j - bvirt;
            bround = _i - bvirt;
            around = ti1 - avirt;
            _0 = around + bround;
            _i = _0 - tj1;
            bvirt = _0 - _i;
            avirt = _i + bvirt;
            bround = bvirt - tj1;
            around = _0 - avirt;
            abtt[1 as libc::c_int as usize] = around + bround;
            abtt3 = _j + _i;
            bvirt = abtt3 - _j;
            avirt = abtt3 - bvirt;
            bround = _i - bvirt;
            around = _j - avirt;
            abtt[2 as libc::c_int as usize] = around + bround;
            abtt[3 as libc::c_int as usize] = abtt3;
            abttlen = 4 as libc::c_int;
        } else {
            abt[0 as libc::c_int as usize] = 0.0f64;
            abtlen = 1 as libc::c_int;
            abtt[0 as libc::c_int as usize] = 0.0f64;
            abttlen = 1 as libc::c_int;
        }
        if cdxtail != 0.0f64 {
            temp16alen = scale_expansion_zeroelim(
                cxtablen,
                cxtab.as_mut_ptr(),
                cdxtail,
                temp16a.as_mut_ptr(),
            );
            cxtabtlen = scale_expansion_zeroelim(
                abtlen,
                abt.as_mut_ptr(),
                cdxtail,
                cxtabt.as_mut_ptr(),
            );
            temp32alen = scale_expansion_zeroelim(
                cxtabtlen,
                cxtabt.as_mut_ptr(),
                2.0f64 * cdx,
                temp32a.as_mut_ptr(),
            );
            temp48len = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp32alen,
                temp32a.as_mut_ptr(),
                temp48.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp48len,
                temp48.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adytail != 0.0f64 {
                temp8len = scale_expansion_zeroelim(
                    4 as libc::c_int,
                    bb.as_mut_ptr(),
                    cdxtail,
                    temp8.as_mut_ptr(),
                );
                temp16alen = scale_expansion_zeroelim(
                    temp8len,
                    temp8.as_mut_ptr(),
                    adytail,
                    temp16a.as_mut_ptr(),
                );
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    temp16alen,
                    temp16a.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
            if bdytail != 0.0f64 {
                temp8len = scale_expansion_zeroelim(
                    4 as libc::c_int,
                    aa.as_mut_ptr(),
                    -cdxtail,
                    temp8.as_mut_ptr(),
                );
                temp16alen = scale_expansion_zeroelim(
                    temp8len,
                    temp8.as_mut_ptr(),
                    bdytail,
                    temp16a.as_mut_ptr(),
                );
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    temp16alen,
                    temp16a.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
            temp32alen = scale_expansion_zeroelim(
                cxtabtlen,
                cxtabt.as_mut_ptr(),
                cdxtail,
                temp32a.as_mut_ptr(),
            );
            cxtabttlen = scale_expansion_zeroelim(
                abttlen,
                abtt.as_mut_ptr(),
                cdxtail,
                cxtabtt.as_mut_ptr(),
            );
            temp16alen = scale_expansion_zeroelim(
                cxtabttlen,
                cxtabtt.as_mut_ptr(),
                2.0f64 * cdx,
                temp16a.as_mut_ptr(),
            );
            temp16blen = scale_expansion_zeroelim(
                cxtabttlen,
                cxtabtt.as_mut_ptr(),
                cdxtail,
                temp16b.as_mut_ptr(),
            );
            temp32blen = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp16blen,
                temp16b.as_mut_ptr(),
                temp32b.as_mut_ptr(),
            );
            temp64len = fast_expansion_sum_zeroelim(
                temp32alen,
                temp32a.as_mut_ptr(),
                temp32blen,
                temp32b.as_mut_ptr(),
                temp64.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp64len,
                temp64.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
        }
        if cdytail != 0.0f64 {
            temp16alen = scale_expansion_zeroelim(
                cytablen,
                cytab.as_mut_ptr(),
                cdytail,
                temp16a.as_mut_ptr(),
            );
            cytabtlen = scale_expansion_zeroelim(
                abtlen,
                abt.as_mut_ptr(),
                cdytail,
                cytabt.as_mut_ptr(),
            );
            temp32alen = scale_expansion_zeroelim(
                cytabtlen,
                cytabt.as_mut_ptr(),
                2.0f64 * cdy,
                temp32a.as_mut_ptr(),
            );
            temp48len = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp32alen,
                temp32a.as_mut_ptr(),
                temp48.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp48len,
                temp48.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            temp32alen = scale_expansion_zeroelim(
                cytabtlen,
                cytabt.as_mut_ptr(),
                cdytail,
                temp32a.as_mut_ptr(),
            );
            cytabttlen = scale_expansion_zeroelim(
                abttlen,
                abtt.as_mut_ptr(),
                cdytail,
                cytabtt.as_mut_ptr(),
            );
            temp16alen = scale_expansion_zeroelim(
                cytabttlen,
                cytabtt.as_mut_ptr(),
                2.0f64 * cdy,
                temp16a.as_mut_ptr(),
            );
            temp16blen = scale_expansion_zeroelim(
                cytabttlen,
                cytabtt.as_mut_ptr(),
                cdytail,
                temp16b.as_mut_ptr(),
            );
            temp32blen = fast_expansion_sum_zeroelim(
                temp16alen,
                temp16a.as_mut_ptr(),
                temp16blen,
                temp16b.as_mut_ptr(),
                temp32b.as_mut_ptr(),
            );
            temp64len = fast_expansion_sum_zeroelim(
                temp32alen,
                temp32a.as_mut_ptr(),
                temp32blen,
                temp32b.as_mut_ptr(),
                temp64.as_mut_ptr(),
            );
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                temp64len,
                temp64.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
        }
    }
    return *finnow.offset((finlength - 1 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn incircle(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
) -> libc::c_double {
    let mut adx: libc::c_double = 0.;
    let mut bdx: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut bdxcdy: libc::c_double = 0.;
    let mut cdxbdy: libc::c_double = 0.;
    let mut cdxady: libc::c_double = 0.;
    let mut adxcdy: libc::c_double = 0.;
    let mut adxbdy: libc::c_double = 0.;
    let mut bdxady: libc::c_double = 0.;
    let mut alift: libc::c_double = 0.;
    let mut blift: libc::c_double = 0.;
    let mut clift: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut permanent: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    adx = *pa.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    bdx = *pb.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    cdx = *pc.offset(0 as libc::c_int as isize) - *pd.offset(0 as libc::c_int as isize);
    ady = *pa.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdy = *pb.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    cdy = *pc.offset(1 as libc::c_int as isize) - *pd.offset(1 as libc::c_int as isize);
    bdxcdy = bdx * cdy;
    cdxbdy = cdx * bdy;
    alift = adx * adx + ady * ady;
    cdxady = cdx * ady;
    adxcdy = adx * cdy;
    blift = bdx * bdx + bdy * bdy;
    adxbdy = adx * bdy;
    bdxady = bdx * ady;
    clift = cdx * cdx + cdy * cdy;
    det = alift * (bdxcdy - cdxbdy) + blift * (cdxady - adxcdy)
        + clift * (adxbdy - bdxady);
    permanent = ((if bdxcdy >= 0.0f64 { bdxcdy } else { -bdxcdy })
        + (if cdxbdy >= 0.0f64 { cdxbdy } else { -cdxbdy })) * alift
        + ((if cdxady >= 0.0f64 { cdxady } else { -cdxady })
            + (if adxcdy >= 0.0f64 { adxcdy } else { -adxcdy })) * blift
        + ((if adxbdy >= 0.0f64 { adxbdy } else { -adxbdy })
            + (if bdxady >= 0.0f64 { bdxady } else { -bdxady })) * clift;
    errbound = iccerrboundA * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    return incircleadapt(pa, pb, pc, pd, permanent);
}
#[no_mangle]
pub unsafe extern "C" fn inspherefast(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut pe: *mut libc::c_double,
) -> libc::c_double {
    let mut aex: libc::c_double = 0.;
    let mut bex: libc::c_double = 0.;
    let mut cex: libc::c_double = 0.;
    let mut dex: libc::c_double = 0.;
    let mut aey: libc::c_double = 0.;
    let mut bey: libc::c_double = 0.;
    let mut cey: libc::c_double = 0.;
    let mut dey: libc::c_double = 0.;
    let mut aez: libc::c_double = 0.;
    let mut bez: libc::c_double = 0.;
    let mut cez: libc::c_double = 0.;
    let mut dez: libc::c_double = 0.;
    let mut alift: libc::c_double = 0.;
    let mut blift: libc::c_double = 0.;
    let mut clift: libc::c_double = 0.;
    let mut dlift: libc::c_double = 0.;
    let mut ab: libc::c_double = 0.;
    let mut bc: libc::c_double = 0.;
    let mut cd: libc::c_double = 0.;
    let mut da: libc::c_double = 0.;
    let mut ac: libc::c_double = 0.;
    let mut bd: libc::c_double = 0.;
    let mut abc: libc::c_double = 0.;
    let mut bcd: libc::c_double = 0.;
    let mut cda: libc::c_double = 0.;
    let mut dab: libc::c_double = 0.;
    aex = *pa.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bex = *pb.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    cex = *pc.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    dex = *pd.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    aey = *pa.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bey = *pb.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    cey = *pc.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    dey = *pd.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    aez = *pa.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bez = *pb.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    cez = *pc.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    dez = *pd.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    ab = aex * bey - bex * aey;
    bc = bex * cey - cex * bey;
    cd = cex * dey - dex * cey;
    da = dex * aey - aex * dey;
    ac = aex * cey - cex * aey;
    bd = bex * dey - dex * bey;
    abc = aez * bc - bez * ac + cez * ab;
    bcd = bez * cd - cez * bd + dez * bc;
    cda = cez * da + dez * ac + aez * cd;
    dab = dez * ab + aez * bd + bez * da;
    alift = aex * aex + aey * aey + aez * aez;
    blift = bex * bex + bey * bey + bez * bez;
    clift = cex * cex + cey * cey + cez * cez;
    dlift = dex * dex + dey * dey + dez * dez;
    return dlift * abc - clift * dab + (blift * cda - alift * bcd);
}
#[no_mangle]
pub unsafe extern "C" fn insphereexact(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut pe: *mut libc::c_double,
) -> libc::c_double {
    let mut axby1: libc::c_double = 0.;
    let mut bxcy1: libc::c_double = 0.;
    let mut cxdy1: libc::c_double = 0.;
    let mut dxey1: libc::c_double = 0.;
    let mut exay1: libc::c_double = 0.;
    let mut bxay1: libc::c_double = 0.;
    let mut cxby1: libc::c_double = 0.;
    let mut dxcy1: libc::c_double = 0.;
    let mut exdy1: libc::c_double = 0.;
    let mut axey1: libc::c_double = 0.;
    let mut axcy1: libc::c_double = 0.;
    let mut bxdy1: libc::c_double = 0.;
    let mut cxey1: libc::c_double = 0.;
    let mut dxay1: libc::c_double = 0.;
    let mut exby1: libc::c_double = 0.;
    let mut cxay1: libc::c_double = 0.;
    let mut dxby1: libc::c_double = 0.;
    let mut excy1: libc::c_double = 0.;
    let mut axdy1: libc::c_double = 0.;
    let mut bxey1: libc::c_double = 0.;
    let mut axby0: libc::c_double = 0.;
    let mut bxcy0: libc::c_double = 0.;
    let mut cxdy0: libc::c_double = 0.;
    let mut dxey0: libc::c_double = 0.;
    let mut exay0: libc::c_double = 0.;
    let mut bxay0: libc::c_double = 0.;
    let mut cxby0: libc::c_double = 0.;
    let mut dxcy0: libc::c_double = 0.;
    let mut exdy0: libc::c_double = 0.;
    let mut axey0: libc::c_double = 0.;
    let mut axcy0: libc::c_double = 0.;
    let mut bxdy0: libc::c_double = 0.;
    let mut cxey0: libc::c_double = 0.;
    let mut dxay0: libc::c_double = 0.;
    let mut exby0: libc::c_double = 0.;
    let mut cxay0: libc::c_double = 0.;
    let mut dxby0: libc::c_double = 0.;
    let mut excy0: libc::c_double = 0.;
    let mut axdy0: libc::c_double = 0.;
    let mut bxey0: libc::c_double = 0.;
    let mut ab: [libc::c_double; 4] = [0.; 4];
    let mut bc: [libc::c_double; 4] = [0.; 4];
    let mut cd: [libc::c_double; 4] = [0.; 4];
    let mut de: [libc::c_double; 4] = [0.; 4];
    let mut ea: [libc::c_double; 4] = [0.; 4];
    let mut ac: [libc::c_double; 4] = [0.; 4];
    let mut bd: [libc::c_double; 4] = [0.; 4];
    let mut ce: [libc::c_double; 4] = [0.; 4];
    let mut da: [libc::c_double; 4] = [0.; 4];
    let mut eb: [libc::c_double; 4] = [0.; 4];
    let mut temp8a: [libc::c_double; 8] = [0.; 8];
    let mut temp8b: [libc::c_double; 8] = [0.; 8];
    let mut temp16: [libc::c_double; 16] = [0.; 16];
    let mut temp8alen: libc::c_int = 0;
    let mut temp8blen: libc::c_int = 0;
    let mut temp16len: libc::c_int = 0;
    let mut abc: [libc::c_double; 24] = [0.; 24];
    let mut bcd: [libc::c_double; 24] = [0.; 24];
    let mut cde: [libc::c_double; 24] = [0.; 24];
    let mut dea: [libc::c_double; 24] = [0.; 24];
    let mut eab: [libc::c_double; 24] = [0.; 24];
    let mut abd: [libc::c_double; 24] = [0.; 24];
    let mut bce: [libc::c_double; 24] = [0.; 24];
    let mut cda: [libc::c_double; 24] = [0.; 24];
    let mut deb: [libc::c_double; 24] = [0.; 24];
    let mut eac: [libc::c_double; 24] = [0.; 24];
    let mut abclen: libc::c_int = 0;
    let mut bcdlen: libc::c_int = 0;
    let mut cdelen: libc::c_int = 0;
    let mut dealen: libc::c_int = 0;
    let mut eablen: libc::c_int = 0;
    let mut abdlen: libc::c_int = 0;
    let mut bcelen: libc::c_int = 0;
    let mut cdalen: libc::c_int = 0;
    let mut deblen: libc::c_int = 0;
    let mut eaclen: libc::c_int = 0;
    let mut temp48a: [libc::c_double; 48] = [0.; 48];
    let mut temp48b: [libc::c_double; 48] = [0.; 48];
    let mut temp48alen: libc::c_int = 0;
    let mut temp48blen: libc::c_int = 0;
    let mut abcd: [libc::c_double; 96] = [0.; 96];
    let mut bcde: [libc::c_double; 96] = [0.; 96];
    let mut cdea: [libc::c_double; 96] = [0.; 96];
    let mut deab: [libc::c_double; 96] = [0.; 96];
    let mut eabc: [libc::c_double; 96] = [0.; 96];
    let mut abcdlen: libc::c_int = 0;
    let mut bcdelen: libc::c_int = 0;
    let mut cdealen: libc::c_int = 0;
    let mut deablen: libc::c_int = 0;
    let mut eabclen: libc::c_int = 0;
    let mut temp192: [libc::c_double; 192] = [0.; 192];
    let mut det384x: [libc::c_double; 384] = [0.; 384];
    let mut det384y: [libc::c_double; 384] = [0.; 384];
    let mut det384z: [libc::c_double; 384] = [0.; 384];
    let mut xlen: libc::c_int = 0;
    let mut ylen: libc::c_int = 0;
    let mut zlen: libc::c_int = 0;
    let mut detxy: [libc::c_double; 768] = [0.; 768];
    let mut xylen: libc::c_int = 0;
    let mut adet: [libc::c_double; 1152] = [0.; 1152];
    let mut bdet: [libc::c_double; 1152] = [0.; 1152];
    let mut cdet: [libc::c_double; 1152] = [0.; 1152];
    let mut ddet: [libc::c_double; 1152] = [0.; 1152];
    let mut edet: [libc::c_double; 1152] = [0.; 1152];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut elen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 2304] = [0.; 2304];
    let mut cddet: [libc::c_double; 2304] = [0.; 2304];
    let mut cdedet: [libc::c_double; 3456] = [0.; 3456];
    let mut ablen: libc::c_int = 0;
    let mut cdlen: libc::c_int = 0;
    let mut deter: [libc::c_double; 5760] = [0.; 5760];
    let mut deterlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    axby1 = *pa.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = axby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axby0 = alo * blo - err3;
    bxay1 = *pb.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxay0 = alo * blo - err3;
    _i = axby0 - bxay0;
    bvirt = axby0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay0;
    around = axby0 - avirt;
    ab[0 as libc::c_int as usize] = around + bround;
    _j = axby1 + _i;
    bvirt = _j - axby1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axby1 - avirt;
    _0 = around + bround;
    _i = _0 - bxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxay1;
    around = _0 - avirt;
    ab[1 as libc::c_int as usize] = around + bround;
    ab[3 as libc::c_int as usize] = _j + _i;
    bvirt = ab[3 as libc::c_int as usize] - _j;
    avirt = ab[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as libc::c_int as usize] = around + bround;
    bxcy1 = *pb.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxcy0 = alo * blo - err3;
    cxby1 = *pc.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxby0 = alo * blo - err3;
    _i = bxcy0 - cxby0;
    bvirt = bxcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby0;
    around = bxcy0 - avirt;
    bc[0 as libc::c_int as usize] = around + bround;
    _j = bxcy1 + _i;
    bvirt = _j - bxcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxcy1 - avirt;
    _0 = around + bround;
    _i = _0 - cxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxby1;
    around = _0 - avirt;
    bc[1 as libc::c_int as usize] = around + bround;
    bc[3 as libc::c_int as usize] = _j + _i;
    bvirt = bc[3 as libc::c_int as usize] - _j;
    avirt = bc[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as libc::c_int as usize] = around + bround;
    cxdy1 = *pc.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxdy0 = alo * blo - err3;
    dxcy1 = *pd.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxcy0 = alo * blo - err3;
    _i = cxdy0 - dxcy0;
    bvirt = cxdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxcy0;
    around = cxdy0 - avirt;
    cd[0 as libc::c_int as usize] = around + bround;
    _j = cxdy1 + _i;
    bvirt = _j - cxdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cxdy1 - avirt;
    _0 = around + bround;
    _i = _0 - dxcy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxcy1;
    around = _0 - avirt;
    cd[1 as libc::c_int as usize] = around + bround;
    cd[3 as libc::c_int as usize] = _j + _i;
    bvirt = cd[3 as libc::c_int as usize] - _j;
    avirt = cd[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    cd[2 as libc::c_int as usize] = around + bround;
    dxey1 = *pd.offset(0 as libc::c_int as isize)
        * *pe.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pe.offset(1 as libc::c_int as isize);
    abig = c - *pe.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pe.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxey0 = alo * blo - err3;
    exdy1 = *pe.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pe.offset(0 as libc::c_int as isize);
    abig = c - *pe.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pe.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = exdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    exdy0 = alo * blo - err3;
    _i = dxey0 - exdy0;
    bvirt = dxey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - exdy0;
    around = dxey0 - avirt;
    de[0 as libc::c_int as usize] = around + bround;
    _j = dxey1 + _i;
    bvirt = _j - dxey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = dxey1 - avirt;
    _0 = around + bround;
    _i = _0 - exdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - exdy1;
    around = _0 - avirt;
    de[1 as libc::c_int as usize] = around + bround;
    de[3 as libc::c_int as usize] = _j + _i;
    bvirt = de[3 as libc::c_int as usize] - _j;
    avirt = de[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    de[2 as libc::c_int as usize] = around + bround;
    exay1 = *pe.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pe.offset(0 as libc::c_int as isize);
    abig = c - *pe.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pe.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = exay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    exay0 = alo * blo - err3;
    axey1 = *pa.offset(0 as libc::c_int as isize)
        * *pe.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pe.offset(1 as libc::c_int as isize);
    abig = c - *pe.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pe.offset(1 as libc::c_int as isize) - bhi;
    err1 = axey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axey0 = alo * blo - err3;
    _i = exay0 - axey0;
    bvirt = exay0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axey0;
    around = exay0 - avirt;
    ea[0 as libc::c_int as usize] = around + bround;
    _j = exay1 + _i;
    bvirt = _j - exay1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = exay1 - avirt;
    _0 = around + bround;
    _i = _0 - axey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axey1;
    around = _0 - avirt;
    ea[1 as libc::c_int as usize] = around + bround;
    ea[3 as libc::c_int as usize] = _j + _i;
    bvirt = ea[3 as libc::c_int as usize] - _j;
    avirt = ea[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ea[2 as libc::c_int as usize] = around + bround;
    axcy1 = *pa.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = axcy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axcy0 = alo * blo - err3;
    cxay1 = *pc.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxay0 = alo * blo - err3;
    _i = axcy0 - cxay0;
    bvirt = axcy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxay0;
    around = axcy0 - avirt;
    ac[0 as libc::c_int as usize] = around + bround;
    _j = axcy1 + _i;
    bvirt = _j - axcy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = axcy1 - avirt;
    _0 = around + bround;
    _i = _0 - cxay1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cxay1;
    around = _0 - avirt;
    ac[1 as libc::c_int as usize] = around + bround;
    ac[3 as libc::c_int as usize] = _j + _i;
    bvirt = ac[3 as libc::c_int as usize] - _j;
    avirt = ac[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ac[2 as libc::c_int as usize] = around + bround;
    bxdy1 = *pb.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxdy0 = alo * blo - err3;
    dxby1 = *pd.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxby0 = alo * blo - err3;
    _i = bxdy0 - dxby0;
    bvirt = bxdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxby0;
    around = bxdy0 - avirt;
    bd[0 as libc::c_int as usize] = around + bround;
    _j = bxdy1 + _i;
    bvirt = _j - bxdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bxdy1 - avirt;
    _0 = around + bround;
    _i = _0 - dxby1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dxby1;
    around = _0 - avirt;
    bd[1 as libc::c_int as usize] = around + bround;
    bd[3 as libc::c_int as usize] = _j + _i;
    bvirt = bd[3 as libc::c_int as usize] - _j;
    avirt = bd[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bd[2 as libc::c_int as usize] = around + bround;
    cxey1 = *pc.offset(0 as libc::c_int as isize)
        * *pe.offset(1 as libc::c_int as isize);
    c = splitter * *pc.offset(0 as libc::c_int as isize);
    abig = c - *pc.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pc.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pe.offset(1 as libc::c_int as isize);
    abig = c - *pe.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pe.offset(1 as libc::c_int as isize) - bhi;
    err1 = cxey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cxey0 = alo * blo - err3;
    excy1 = *pe.offset(0 as libc::c_int as isize)
        * *pc.offset(1 as libc::c_int as isize);
    c = splitter * *pe.offset(0 as libc::c_int as isize);
    abig = c - *pe.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pe.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pc.offset(1 as libc::c_int as isize);
    abig = c - *pc.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pc.offset(1 as libc::c_int as isize) - bhi;
    err1 = excy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    excy0 = alo * blo - err3;
    _i = cxey0 - excy0;
    bvirt = cxey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - excy0;
    around = cxey0 - avirt;
    ce[0 as libc::c_int as usize] = around + bround;
    _j = cxey1 + _i;
    bvirt = _j - cxey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cxey1 - avirt;
    _0 = around + bround;
    _i = _0 - excy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - excy1;
    around = _0 - avirt;
    ce[1 as libc::c_int as usize] = around + bround;
    ce[3 as libc::c_int as usize] = _j + _i;
    bvirt = ce[3 as libc::c_int as usize] - _j;
    avirt = ce[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ce[2 as libc::c_int as usize] = around + bround;
    dxay1 = *pd.offset(0 as libc::c_int as isize)
        * *pa.offset(1 as libc::c_int as isize);
    c = splitter * *pd.offset(0 as libc::c_int as isize);
    abig = c - *pd.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pd.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pa.offset(1 as libc::c_int as isize);
    abig = c - *pa.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pa.offset(1 as libc::c_int as isize) - bhi;
    err1 = dxay1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dxay0 = alo * blo - err3;
    axdy1 = *pa.offset(0 as libc::c_int as isize)
        * *pd.offset(1 as libc::c_int as isize);
    c = splitter * *pa.offset(0 as libc::c_int as isize);
    abig = c - *pa.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pa.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pd.offset(1 as libc::c_int as isize);
    abig = c - *pd.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pd.offset(1 as libc::c_int as isize) - bhi;
    err1 = axdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    axdy0 = alo * blo - err3;
    _i = dxay0 - axdy0;
    bvirt = dxay0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axdy0;
    around = dxay0 - avirt;
    da[0 as libc::c_int as usize] = around + bround;
    _j = dxay1 + _i;
    bvirt = _j - dxay1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = dxay1 - avirt;
    _0 = around + bround;
    _i = _0 - axdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - axdy1;
    around = _0 - avirt;
    da[1 as libc::c_int as usize] = around + bround;
    da[3 as libc::c_int as usize] = _j + _i;
    bvirt = da[3 as libc::c_int as usize] - _j;
    avirt = da[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    da[2 as libc::c_int as usize] = around + bround;
    exby1 = *pe.offset(0 as libc::c_int as isize)
        * *pb.offset(1 as libc::c_int as isize);
    c = splitter * *pe.offset(0 as libc::c_int as isize);
    abig = c - *pe.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pe.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pb.offset(1 as libc::c_int as isize);
    abig = c - *pb.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pb.offset(1 as libc::c_int as isize) - bhi;
    err1 = exby1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    exby0 = alo * blo - err3;
    bxey1 = *pb.offset(0 as libc::c_int as isize)
        * *pe.offset(1 as libc::c_int as isize);
    c = splitter * *pb.offset(0 as libc::c_int as isize);
    abig = c - *pb.offset(0 as libc::c_int as isize);
    ahi = c - abig;
    alo = *pb.offset(0 as libc::c_int as isize) - ahi;
    c = splitter * *pe.offset(1 as libc::c_int as isize);
    abig = c - *pe.offset(1 as libc::c_int as isize);
    bhi = c - abig;
    blo = *pe.offset(1 as libc::c_int as isize) - bhi;
    err1 = bxey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bxey0 = alo * blo - err3;
    _i = exby0 - bxey0;
    bvirt = exby0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxey0;
    around = exby0 - avirt;
    eb[0 as libc::c_int as usize] = around + bround;
    _j = exby1 + _i;
    bvirt = _j - exby1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = exby1 - avirt;
    _0 = around + bround;
    _i = _0 - bxey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bxey1;
    around = _0 - avirt;
    eb[1 as libc::c_int as usize] = around + bround;
    eb[3 as libc::c_int as usize] = _j + _i;
    bvirt = eb[3 as libc::c_int as usize] - _j;
    avirt = eb[3 as libc::c_int as usize] - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    eb[2 as libc::c_int as usize] = around + bround;
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ac.as_mut_ptr(),
        -*pb.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    abclen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        abc.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bd.as_mut_ptr(),
        -*pc.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    bcdlen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        bcd.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        de.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ce.as_mut_ptr(),
        -*pd.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    cdelen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        cde.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ea.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        -*pe.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        de.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    dealen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        dea.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        eb.as_mut_ptr(),
        -*pa.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ea.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    eablen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        eab.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bd.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    abdlen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        abd.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ce.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        eb.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    bcelen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        bce.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ac.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    cdalen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        cda.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        eb.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bd.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        de.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    deblen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        deb.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ac.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ce.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        temp8b.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ea.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        temp8a.as_mut_ptr(),
    );
    eaclen = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        eac.as_mut_ptr(),
    );
    temp48alen = fast_expansion_sum_zeroelim(
        cdelen,
        cde.as_mut_ptr(),
        bcelen,
        bce.as_mut_ptr(),
        temp48a.as_mut_ptr(),
    );
    temp48blen = fast_expansion_sum_zeroelim(
        deblen,
        deb.as_mut_ptr(),
        bcdlen,
        bcd.as_mut_ptr(),
        temp48b.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < temp48blen {
        temp48b[i as usize] = -temp48b[i as usize];
        i += 1;
    }
    bcdelen = fast_expansion_sum_zeroelim(
        temp48alen,
        temp48a.as_mut_ptr(),
        temp48blen,
        temp48b.as_mut_ptr(),
        bcde.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        bcdelen,
        bcde.as_mut_ptr(),
        *pa.offset(0 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        temp192.as_mut_ptr(),
        *pa.offset(0 as libc::c_int as isize),
        det384x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        bcdelen,
        bcde.as_mut_ptr(),
        *pa.offset(1 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        temp192.as_mut_ptr(),
        *pa.offset(1 as libc::c_int as isize),
        det384y.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        bcdelen,
        bcde.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        zlen,
        temp192.as_mut_ptr(),
        *pa.offset(2 as libc::c_int as isize),
        det384z.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        det384x.as_mut_ptr(),
        ylen,
        det384y.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        xylen,
        detxy.as_mut_ptr(),
        zlen,
        det384z.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    temp48alen = fast_expansion_sum_zeroelim(
        dealen,
        dea.as_mut_ptr(),
        cdalen,
        cda.as_mut_ptr(),
        temp48a.as_mut_ptr(),
    );
    temp48blen = fast_expansion_sum_zeroelim(
        eaclen,
        eac.as_mut_ptr(),
        cdelen,
        cde.as_mut_ptr(),
        temp48b.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < temp48blen {
        temp48b[i as usize] = -temp48b[i as usize];
        i += 1;
    }
    cdealen = fast_expansion_sum_zeroelim(
        temp48alen,
        temp48a.as_mut_ptr(),
        temp48blen,
        temp48b.as_mut_ptr(),
        cdea.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        cdealen,
        cdea.as_mut_ptr(),
        *pb.offset(0 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        temp192.as_mut_ptr(),
        *pb.offset(0 as libc::c_int as isize),
        det384x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        cdealen,
        cdea.as_mut_ptr(),
        *pb.offset(1 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        temp192.as_mut_ptr(),
        *pb.offset(1 as libc::c_int as isize),
        det384y.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        cdealen,
        cdea.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        zlen,
        temp192.as_mut_ptr(),
        *pb.offset(2 as libc::c_int as isize),
        det384z.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        det384x.as_mut_ptr(),
        ylen,
        det384y.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        xylen,
        detxy.as_mut_ptr(),
        zlen,
        det384z.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    temp48alen = fast_expansion_sum_zeroelim(
        eablen,
        eab.as_mut_ptr(),
        deblen,
        deb.as_mut_ptr(),
        temp48a.as_mut_ptr(),
    );
    temp48blen = fast_expansion_sum_zeroelim(
        abdlen,
        abd.as_mut_ptr(),
        dealen,
        dea.as_mut_ptr(),
        temp48b.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < temp48blen {
        temp48b[i as usize] = -temp48b[i as usize];
        i += 1;
    }
    deablen = fast_expansion_sum_zeroelim(
        temp48alen,
        temp48a.as_mut_ptr(),
        temp48blen,
        temp48b.as_mut_ptr(),
        deab.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        deablen,
        deab.as_mut_ptr(),
        *pc.offset(0 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        temp192.as_mut_ptr(),
        *pc.offset(0 as libc::c_int as isize),
        det384x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        deablen,
        deab.as_mut_ptr(),
        *pc.offset(1 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        temp192.as_mut_ptr(),
        *pc.offset(1 as libc::c_int as isize),
        det384y.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        deablen,
        deab.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        zlen,
        temp192.as_mut_ptr(),
        *pc.offset(2 as libc::c_int as isize),
        det384z.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        det384x.as_mut_ptr(),
        ylen,
        det384y.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        xylen,
        detxy.as_mut_ptr(),
        zlen,
        det384z.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    temp48alen = fast_expansion_sum_zeroelim(
        abclen,
        abc.as_mut_ptr(),
        eaclen,
        eac.as_mut_ptr(),
        temp48a.as_mut_ptr(),
    );
    temp48blen = fast_expansion_sum_zeroelim(
        bcelen,
        bce.as_mut_ptr(),
        eablen,
        eab.as_mut_ptr(),
        temp48b.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < temp48blen {
        temp48b[i as usize] = -temp48b[i as usize];
        i += 1;
    }
    eabclen = fast_expansion_sum_zeroelim(
        temp48alen,
        temp48a.as_mut_ptr(),
        temp48blen,
        temp48b.as_mut_ptr(),
        eabc.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        eabclen,
        eabc.as_mut_ptr(),
        *pd.offset(0 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        temp192.as_mut_ptr(),
        *pd.offset(0 as libc::c_int as isize),
        det384x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        eabclen,
        eabc.as_mut_ptr(),
        *pd.offset(1 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        temp192.as_mut_ptr(),
        *pd.offset(1 as libc::c_int as isize),
        det384y.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        eabclen,
        eabc.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        zlen,
        temp192.as_mut_ptr(),
        *pd.offset(2 as libc::c_int as isize),
        det384z.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        det384x.as_mut_ptr(),
        ylen,
        det384y.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    dlen = fast_expansion_sum_zeroelim(
        xylen,
        detxy.as_mut_ptr(),
        zlen,
        det384z.as_mut_ptr(),
        ddet.as_mut_ptr(),
    );
    temp48alen = fast_expansion_sum_zeroelim(
        bcdlen,
        bcd.as_mut_ptr(),
        abdlen,
        abd.as_mut_ptr(),
        temp48a.as_mut_ptr(),
    );
    temp48blen = fast_expansion_sum_zeroelim(
        cdalen,
        cda.as_mut_ptr(),
        abclen,
        abc.as_mut_ptr(),
        temp48b.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < temp48blen {
        temp48b[i as usize] = -temp48b[i as usize];
        i += 1;
    }
    abcdlen = fast_expansion_sum_zeroelim(
        temp48alen,
        temp48a.as_mut_ptr(),
        temp48blen,
        temp48b.as_mut_ptr(),
        abcd.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        abcdlen,
        abcd.as_mut_ptr(),
        *pe.offset(0 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        xlen,
        temp192.as_mut_ptr(),
        *pe.offset(0 as libc::c_int as isize),
        det384x.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        abcdlen,
        abcd.as_mut_ptr(),
        *pe.offset(1 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        ylen,
        temp192.as_mut_ptr(),
        *pe.offset(1 as libc::c_int as isize),
        det384y.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        abcdlen,
        abcd.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        temp192.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        zlen,
        temp192.as_mut_ptr(),
        *pe.offset(2 as libc::c_int as isize),
        det384z.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        det384x.as_mut_ptr(),
        ylen,
        det384y.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    elen = fast_expansion_sum_zeroelim(
        xylen,
        detxy.as_mut_ptr(),
        zlen,
        det384z.as_mut_ptr(),
        edet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    cdlen = fast_expansion_sum_zeroelim(
        clen,
        cdet.as_mut_ptr(),
        dlen,
        ddet.as_mut_ptr(),
        cddet.as_mut_ptr(),
    );
    cdelen = fast_expansion_sum_zeroelim(
        cdlen,
        cddet.as_mut_ptr(),
        elen,
        edet.as_mut_ptr(),
        cdedet.as_mut_ptr(),
    );
    deterlen = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        cdelen,
        cdedet.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn insphereslow(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut pe: *mut libc::c_double,
) -> libc::c_double {
    let mut aex: libc::c_double = 0.;
    let mut bex: libc::c_double = 0.;
    let mut cex: libc::c_double = 0.;
    let mut dex: libc::c_double = 0.;
    let mut aey: libc::c_double = 0.;
    let mut bey: libc::c_double = 0.;
    let mut cey: libc::c_double = 0.;
    let mut dey: libc::c_double = 0.;
    let mut aez: libc::c_double = 0.;
    let mut bez: libc::c_double = 0.;
    let mut cez: libc::c_double = 0.;
    let mut dez: libc::c_double = 0.;
    let mut aextail: libc::c_double = 0.;
    let mut bextail: libc::c_double = 0.;
    let mut cextail: libc::c_double = 0.;
    let mut dextail: libc::c_double = 0.;
    let mut aeytail: libc::c_double = 0.;
    let mut beytail: libc::c_double = 0.;
    let mut ceytail: libc::c_double = 0.;
    let mut deytail: libc::c_double = 0.;
    let mut aeztail: libc::c_double = 0.;
    let mut beztail: libc::c_double = 0.;
    let mut ceztail: libc::c_double = 0.;
    let mut deztail: libc::c_double = 0.;
    let mut negate: libc::c_double = 0.;
    let mut negatetail: libc::c_double = 0.;
    let mut axby7: libc::c_double = 0.;
    let mut bxcy7: libc::c_double = 0.;
    let mut cxdy7: libc::c_double = 0.;
    let mut dxay7: libc::c_double = 0.;
    let mut axcy7: libc::c_double = 0.;
    let mut bxdy7: libc::c_double = 0.;
    let mut bxay7: libc::c_double = 0.;
    let mut cxby7: libc::c_double = 0.;
    let mut dxcy7: libc::c_double = 0.;
    let mut axdy7: libc::c_double = 0.;
    let mut cxay7: libc::c_double = 0.;
    let mut dxby7: libc::c_double = 0.;
    let mut axby: [libc::c_double; 8] = [0.; 8];
    let mut bxcy: [libc::c_double; 8] = [0.; 8];
    let mut cxdy: [libc::c_double; 8] = [0.; 8];
    let mut dxay: [libc::c_double; 8] = [0.; 8];
    let mut axcy: [libc::c_double; 8] = [0.; 8];
    let mut bxdy: [libc::c_double; 8] = [0.; 8];
    let mut bxay: [libc::c_double; 8] = [0.; 8];
    let mut cxby: [libc::c_double; 8] = [0.; 8];
    let mut dxcy: [libc::c_double; 8] = [0.; 8];
    let mut axdy: [libc::c_double; 8] = [0.; 8];
    let mut cxay: [libc::c_double; 8] = [0.; 8];
    let mut dxby: [libc::c_double; 8] = [0.; 8];
    let mut ab: [libc::c_double; 16] = [0.; 16];
    let mut bc: [libc::c_double; 16] = [0.; 16];
    let mut cd: [libc::c_double; 16] = [0.; 16];
    let mut da: [libc::c_double; 16] = [0.; 16];
    let mut ac: [libc::c_double; 16] = [0.; 16];
    let mut bd: [libc::c_double; 16] = [0.; 16];
    let mut ablen: libc::c_int = 0;
    let mut bclen: libc::c_int = 0;
    let mut cdlen: libc::c_int = 0;
    let mut dalen: libc::c_int = 0;
    let mut aclen: libc::c_int = 0;
    let mut bdlen: libc::c_int = 0;
    let mut temp32a: [libc::c_double; 32] = [0.; 32];
    let mut temp32b: [libc::c_double; 32] = [0.; 32];
    let mut temp64a: [libc::c_double; 64] = [0.; 64];
    let mut temp64b: [libc::c_double; 64] = [0.; 64];
    let mut temp64c: [libc::c_double; 64] = [0.; 64];
    let mut temp32alen: libc::c_int = 0;
    let mut temp32blen: libc::c_int = 0;
    let mut temp64alen: libc::c_int = 0;
    let mut temp64blen: libc::c_int = 0;
    let mut temp64clen: libc::c_int = 0;
    let mut temp128: [libc::c_double; 128] = [0.; 128];
    let mut temp192: [libc::c_double; 192] = [0.; 192];
    let mut temp128len: libc::c_int = 0;
    let mut temp192len: libc::c_int = 0;
    let mut detx: [libc::c_double; 384] = [0.; 384];
    let mut detxx: [libc::c_double; 768] = [0.; 768];
    let mut detxt: [libc::c_double; 384] = [0.; 384];
    let mut detxxt: [libc::c_double; 768] = [0.; 768];
    let mut detxtxt: [libc::c_double; 768] = [0.; 768];
    let mut xlen: libc::c_int = 0;
    let mut xxlen: libc::c_int = 0;
    let mut xtlen: libc::c_int = 0;
    let mut xxtlen: libc::c_int = 0;
    let mut xtxtlen: libc::c_int = 0;
    let mut x1: [libc::c_double; 1536] = [0.; 1536];
    let mut x2: [libc::c_double; 2304] = [0.; 2304];
    let mut x1len: libc::c_int = 0;
    let mut x2len: libc::c_int = 0;
    let mut dety: [libc::c_double; 384] = [0.; 384];
    let mut detyy: [libc::c_double; 768] = [0.; 768];
    let mut detyt: [libc::c_double; 384] = [0.; 384];
    let mut detyyt: [libc::c_double; 768] = [0.; 768];
    let mut detytyt: [libc::c_double; 768] = [0.; 768];
    let mut ylen: libc::c_int = 0;
    let mut yylen: libc::c_int = 0;
    let mut ytlen: libc::c_int = 0;
    let mut yytlen: libc::c_int = 0;
    let mut ytytlen: libc::c_int = 0;
    let mut y1: [libc::c_double; 1536] = [0.; 1536];
    let mut y2: [libc::c_double; 2304] = [0.; 2304];
    let mut y1len: libc::c_int = 0;
    let mut y2len: libc::c_int = 0;
    let mut detz: [libc::c_double; 384] = [0.; 384];
    let mut detzz: [libc::c_double; 768] = [0.; 768];
    let mut detzt: [libc::c_double; 384] = [0.; 384];
    let mut detzzt: [libc::c_double; 768] = [0.; 768];
    let mut detztzt: [libc::c_double; 768] = [0.; 768];
    let mut zlen: libc::c_int = 0;
    let mut zzlen: libc::c_int = 0;
    let mut ztlen: libc::c_int = 0;
    let mut zztlen: libc::c_int = 0;
    let mut ztztlen: libc::c_int = 0;
    let mut z1: [libc::c_double; 1536] = [0.; 1536];
    let mut z2: [libc::c_double; 2304] = [0.; 2304];
    let mut z1len: libc::c_int = 0;
    let mut z2len: libc::c_int = 0;
    let mut detxy: [libc::c_double; 4608] = [0.; 4608];
    let mut xylen: libc::c_int = 0;
    let mut adet: [libc::c_double; 6912] = [0.; 6912];
    let mut bdet: [libc::c_double; 6912] = [0.; 6912];
    let mut cdet: [libc::c_double; 6912] = [0.; 6912];
    let mut ddet: [libc::c_double; 6912] = [0.; 6912];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 13824] = [0.; 13824];
    let mut cddet: [libc::c_double; 13824] = [0.; 13824];
    let mut deter: [libc::c_double; 27648] = [0.; 27648];
    let mut deterlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut a0hi: libc::c_double = 0.;
    let mut a0lo: libc::c_double = 0.;
    let mut a1hi: libc::c_double = 0.;
    let mut a1lo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _k: libc::c_double = 0.;
    let mut _l: libc::c_double = 0.;
    let mut _m: libc::c_double = 0.;
    let mut _n: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    let mut _1: libc::c_double = 0.;
    let mut _2: libc::c_double = 0.;
    aex = *pa.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bvirt = *pa.offset(0 as libc::c_int as isize) - aex;
    avirt = aex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    aextail = around + bround;
    aey = *pa.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bvirt = *pa.offset(1 as libc::c_int as isize) - aey;
    avirt = aey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    aeytail = around + bround;
    aez = *pa.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bvirt = *pa.offset(2 as libc::c_int as isize) - aez;
    avirt = aez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pa.offset(2 as libc::c_int as isize) - avirt;
    aeztail = around + bround;
    bex = *pb.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bvirt = *pb.offset(0 as libc::c_int as isize) - bex;
    avirt = bex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bextail = around + bround;
    bey = *pb.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bvirt = *pb.offset(1 as libc::c_int as isize) - bey;
    avirt = bey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    beytail = around + bround;
    bez = *pb.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bvirt = *pb.offset(2 as libc::c_int as isize) - bez;
    avirt = bez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pb.offset(2 as libc::c_int as isize) - avirt;
    beztail = around + bround;
    cex = *pc.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bvirt = *pc.offset(0 as libc::c_int as isize) - cex;
    avirt = cex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pc.offset(0 as libc::c_int as isize) - avirt;
    cextail = around + bround;
    cey = *pc.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bvirt = *pc.offset(1 as libc::c_int as isize) - cey;
    avirt = cey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pc.offset(1 as libc::c_int as isize) - avirt;
    ceytail = around + bround;
    cez = *pc.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bvirt = *pc.offset(2 as libc::c_int as isize) - cez;
    avirt = cez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pc.offset(2 as libc::c_int as isize) - avirt;
    ceztail = around + bround;
    dex = *pd.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bvirt = *pd.offset(0 as libc::c_int as isize) - dex;
    avirt = dex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pd.offset(0 as libc::c_int as isize) - avirt;
    dextail = around + bround;
    dey = *pd.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bvirt = *pd.offset(1 as libc::c_int as isize) - dey;
    avirt = dey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pd.offset(1 as libc::c_int as isize) - avirt;
    deytail = around + bround;
    dez = *pd.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bvirt = *pd.offset(2 as libc::c_int as isize) - dez;
    avirt = dez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pd.offset(2 as libc::c_int as isize) - avirt;
    deztail = around + bround;
    c = splitter * aextail;
    abig = c - aextail;
    a0hi = c - abig;
    a0lo = aextail - a0hi;
    c = splitter * beytail;
    abig = c - beytail;
    bhi = c - abig;
    blo = beytail - bhi;
    _i = aextail * beytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * aex;
    abig = c - aex;
    a1hi = c - abig;
    a1lo = aex - a1hi;
    _j = aex * beytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * bey;
    abig = c - bey;
    bhi = c - abig;
    blo = bey - bhi;
    _i = aextail * bey;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = aex * bey;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axby[5 as libc::c_int as usize] = around + bround;
    axby7 = _m + _k;
    bvirt = axby7 - _m;
    avirt = axby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axby[6 as libc::c_int as usize] = around + bround;
    axby[7 as libc::c_int as usize] = axby7;
    negate = -aey;
    negatetail = -aeytail;
    c = splitter * bextail;
    abig = c - bextail;
    a0hi = c - abig;
    a0lo = bextail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = bextail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bex;
    abig = c - bex;
    a1hi = c - abig;
    a1lo = bex - a1hi;
    _j = bex * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = bextail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bex * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxay[5 as libc::c_int as usize] = around + bround;
    bxay7 = _m + _k;
    bvirt = bxay7 - _m;
    avirt = bxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxay[6 as libc::c_int as usize] = around + bround;
    bxay[7 as libc::c_int as usize] = bxay7;
    ablen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        axby.as_mut_ptr(),
        8 as libc::c_int,
        bxay.as_mut_ptr(),
        ab.as_mut_ptr(),
    );
    c = splitter * bextail;
    abig = c - bextail;
    a0hi = c - abig;
    a0lo = bextail - a0hi;
    c = splitter * ceytail;
    abig = c - ceytail;
    bhi = c - abig;
    blo = ceytail - bhi;
    _i = bextail * ceytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bex;
    abig = c - bex;
    a1hi = c - abig;
    a1lo = bex - a1hi;
    _j = bex * ceytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * cey;
    abig = c - cey;
    bhi = c - abig;
    blo = cey - bhi;
    _i = bextail * cey;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bex * cey;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxcy[5 as libc::c_int as usize] = around + bround;
    bxcy7 = _m + _k;
    bvirt = bxcy7 - _m;
    avirt = bxcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxcy[6 as libc::c_int as usize] = around + bround;
    bxcy[7 as libc::c_int as usize] = bxcy7;
    negate = -bey;
    negatetail = -beytail;
    c = splitter * cextail;
    abig = c - cextail;
    a0hi = c - abig;
    a0lo = cextail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = cextail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cex;
    abig = c - cex;
    a1hi = c - abig;
    a1lo = cex - a1hi;
    _j = cex * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = cextail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cex * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxby[5 as libc::c_int as usize] = around + bround;
    cxby7 = _m + _k;
    bvirt = cxby7 - _m;
    avirt = cxby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxby[6 as libc::c_int as usize] = around + bround;
    cxby[7 as libc::c_int as usize] = cxby7;
    bclen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        bxcy.as_mut_ptr(),
        8 as libc::c_int,
        cxby.as_mut_ptr(),
        bc.as_mut_ptr(),
    );
    c = splitter * cextail;
    abig = c - cextail;
    a0hi = c - abig;
    a0lo = cextail - a0hi;
    c = splitter * deytail;
    abig = c - deytail;
    bhi = c - abig;
    blo = deytail - bhi;
    _i = cextail * deytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxdy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cex;
    abig = c - cex;
    a1hi = c - abig;
    a1lo = cex - a1hi;
    _j = cex * deytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * dey;
    abig = c - dey;
    bhi = c - abig;
    blo = dey - bhi;
    _i = cextail * dey;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxdy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cex * dey;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxdy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxdy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxdy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxdy[5 as libc::c_int as usize] = around + bround;
    cxdy7 = _m + _k;
    bvirt = cxdy7 - _m;
    avirt = cxdy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxdy[6 as libc::c_int as usize] = around + bround;
    cxdy[7 as libc::c_int as usize] = cxdy7;
    negate = -cey;
    negatetail = -ceytail;
    c = splitter * dextail;
    abig = c - dextail;
    a0hi = c - abig;
    a0lo = dextail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = dextail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    dxcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * dex;
    abig = c - dex;
    a1hi = c - abig;
    a1lo = dex - a1hi;
    _j = dex * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = dextail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = dex * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    dxcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    dxcy[5 as libc::c_int as usize] = around + bround;
    dxcy7 = _m + _k;
    bvirt = dxcy7 - _m;
    avirt = dxcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    dxcy[6 as libc::c_int as usize] = around + bround;
    dxcy[7 as libc::c_int as usize] = dxcy7;
    cdlen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        cxdy.as_mut_ptr(),
        8 as libc::c_int,
        dxcy.as_mut_ptr(),
        cd.as_mut_ptr(),
    );
    c = splitter * dextail;
    abig = c - dextail;
    a0hi = c - abig;
    a0lo = dextail - a0hi;
    c = splitter * aeytail;
    abig = c - aeytail;
    bhi = c - abig;
    blo = aeytail - bhi;
    _i = dextail * aeytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    dxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * dex;
    abig = c - dex;
    a1hi = c - abig;
    a1lo = dex - a1hi;
    _j = dex * aeytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * aey;
    abig = c - aey;
    bhi = c - abig;
    blo = aey - bhi;
    _i = dextail * aey;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = dex * aey;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    dxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    dxay[5 as libc::c_int as usize] = around + bround;
    dxay7 = _m + _k;
    bvirt = dxay7 - _m;
    avirt = dxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    dxay[6 as libc::c_int as usize] = around + bround;
    dxay[7 as libc::c_int as usize] = dxay7;
    negate = -dey;
    negatetail = -deytail;
    c = splitter * aextail;
    abig = c - aextail;
    a0hi = c - abig;
    a0lo = aextail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = aextail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axdy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * aex;
    abig = c - aex;
    a1hi = c - abig;
    a1lo = aex - a1hi;
    _j = aex * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = aextail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axdy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = aex * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axdy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axdy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axdy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axdy[5 as libc::c_int as usize] = around + bround;
    axdy7 = _m + _k;
    bvirt = axdy7 - _m;
    avirt = axdy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axdy[6 as libc::c_int as usize] = around + bround;
    axdy[7 as libc::c_int as usize] = axdy7;
    dalen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        dxay.as_mut_ptr(),
        8 as libc::c_int,
        axdy.as_mut_ptr(),
        da.as_mut_ptr(),
    );
    c = splitter * aextail;
    abig = c - aextail;
    a0hi = c - abig;
    a0lo = aextail - a0hi;
    c = splitter * ceytail;
    abig = c - ceytail;
    bhi = c - abig;
    blo = ceytail - bhi;
    _i = aextail * ceytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    axcy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * aex;
    abig = c - aex;
    a1hi = c - abig;
    a1lo = aex - a1hi;
    _j = aex * ceytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * cey;
    abig = c - cey;
    bhi = c - abig;
    blo = cey - bhi;
    _i = aextail * cey;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = aex * cey;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    axcy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    axcy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    axcy[5 as libc::c_int as usize] = around + bround;
    axcy7 = _m + _k;
    bvirt = axcy7 - _m;
    avirt = axcy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    axcy[6 as libc::c_int as usize] = around + bround;
    axcy[7 as libc::c_int as usize] = axcy7;
    negate = -aey;
    negatetail = -aeytail;
    c = splitter * cextail;
    abig = c - cextail;
    a0hi = c - abig;
    a0lo = cextail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = cextail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    cxay[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * cex;
    abig = c - cex;
    a1hi = c - abig;
    a1lo = cex - a1hi;
    _j = cex * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = cextail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = cex * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    cxay[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    cxay[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    cxay[5 as libc::c_int as usize] = around + bround;
    cxay7 = _m + _k;
    bvirt = cxay7 - _m;
    avirt = cxay7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    cxay[6 as libc::c_int as usize] = around + bround;
    cxay[7 as libc::c_int as usize] = cxay7;
    aclen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        axcy.as_mut_ptr(),
        8 as libc::c_int,
        cxay.as_mut_ptr(),
        ac.as_mut_ptr(),
    );
    c = splitter * bextail;
    abig = c - bextail;
    a0hi = c - abig;
    a0lo = bextail - a0hi;
    c = splitter * deytail;
    abig = c - deytail;
    bhi = c - abig;
    blo = deytail - bhi;
    _i = bextail * deytail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    bxdy[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * bex;
    abig = c - bex;
    a1hi = c - abig;
    a1lo = bex - a1hi;
    _j = bex * deytail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * dey;
    abig = c - dey;
    bhi = c - abig;
    blo = dey - bhi;
    _i = bextail * dey;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxdy[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = bex * dey;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxdy[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    bxdy[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    bxdy[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    bxdy[5 as libc::c_int as usize] = around + bround;
    bxdy7 = _m + _k;
    bvirt = bxdy7 - _m;
    avirt = bxdy7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    bxdy[6 as libc::c_int as usize] = around + bround;
    bxdy[7 as libc::c_int as usize] = bxdy7;
    negate = -bey;
    negatetail = -beytail;
    c = splitter * dextail;
    abig = c - dextail;
    a0hi = c - abig;
    a0lo = dextail - a0hi;
    c = splitter * negatetail;
    abig = c - negatetail;
    bhi = c - abig;
    blo = negatetail - bhi;
    _i = dextail * negatetail;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    dxby[0 as libc::c_int as usize] = a0lo * blo - err3;
    c = splitter * dex;
    abig = c - dex;
    a1hi = c - abig;
    a1lo = dex - a1hi;
    _j = dex * negatetail;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _k = _i + _0;
    bvirt = _k - _i;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _1 = around + bround;
    _l = _j + _k;
    bvirt = _l - _j;
    _2 = _k - bvirt;
    c = splitter * negate;
    abig = c - negate;
    bhi = c - abig;
    blo = negate - bhi;
    _i = dextail * negate;
    err1 = _i - a0hi * bhi;
    err2 = err1 - a0lo * bhi;
    err3 = err2 - a0hi * blo;
    _0 = a0lo * blo - err3;
    _k = _1 + _0;
    bvirt = _k - _1;
    avirt = _k - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxby[1 as libc::c_int as usize] = around + bround;
    _j = _2 + _k;
    bvirt = _j - _2;
    avirt = _j - bvirt;
    bround = _k - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _j;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _j - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _j = dex * negate;
    err1 = _j - a1hi * bhi;
    err2 = err1 - a1lo * bhi;
    err3 = err2 - a1hi * blo;
    _0 = a1lo * blo - err3;
    _n = _i + _0;
    bvirt = _n - _i;
    avirt = _n - bvirt;
    bround = _0 - bvirt;
    around = _i - avirt;
    _0 = around + bround;
    _i = _1 + _0;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxby[2 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _l = _m + _k;
    bvirt = _l - _m;
    avirt = _l - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    _2 = around + bround;
    _k = _j + _n;
    bvirt = _k - _j;
    avirt = _k - bvirt;
    bround = _n - bvirt;
    around = _j - avirt;
    _0 = around + bround;
    _j = _1 + _0;
    bvirt = _j - _1;
    avirt = _j - bvirt;
    bround = _0 - bvirt;
    around = _1 - avirt;
    dxby[3 as libc::c_int as usize] = around + bround;
    _i = _2 + _j;
    bvirt = _i - _2;
    avirt = _i - bvirt;
    bround = _j - bvirt;
    around = _2 - avirt;
    _1 = around + bround;
    _m = _l + _i;
    bvirt = _m - _l;
    avirt = _m - bvirt;
    bround = _i - bvirt;
    around = _l - avirt;
    _2 = around + bround;
    _i = _1 + _k;
    bvirt = _i - _1;
    avirt = _i - bvirt;
    bround = _k - bvirt;
    around = _1 - avirt;
    dxby[4 as libc::c_int as usize] = around + bround;
    _k = _2 + _i;
    bvirt = _k - _2;
    avirt = _k - bvirt;
    bround = _i - bvirt;
    around = _2 - avirt;
    dxby[5 as libc::c_int as usize] = around + bround;
    dxby7 = _m + _k;
    bvirt = dxby7 - _m;
    avirt = dxby7 - bvirt;
    bround = _k - bvirt;
    around = _m - avirt;
    dxby[6 as libc::c_int as usize] = around + bround;
    dxby[7 as libc::c_int as usize] = dxby7;
    bdlen = fast_expansion_sum_zeroelim(
        8 as libc::c_int,
        bxdy.as_mut_ptr(),
        8 as libc::c_int,
        dxby.as_mut_ptr(),
        bd.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        cdlen,
        cd.as_mut_ptr(),
        -bez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        cdlen,
        cd.as_mut_ptr(),
        -beztail,
        temp32b.as_mut_ptr(),
    );
    temp64alen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64a.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        bdlen,
        bd.as_mut_ptr(),
        cez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        bdlen,
        bd.as_mut_ptr(),
        ceztail,
        temp32b.as_mut_ptr(),
    );
    temp64blen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64b.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        bclen,
        bc.as_mut_ptr(),
        -dez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        bclen,
        bc.as_mut_ptr(),
        -deztail,
        temp32b.as_mut_ptr(),
    );
    temp64clen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64c.as_mut_ptr(),
    );
    temp128len = fast_expansion_sum_zeroelim(
        temp64alen,
        temp64a.as_mut_ptr(),
        temp64blen,
        temp64b.as_mut_ptr(),
        temp128.as_mut_ptr(),
    );
    temp192len = fast_expansion_sum_zeroelim(
        temp64clen,
        temp64c.as_mut_ptr(),
        temp128len,
        temp128.as_mut_ptr(),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        aex,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), aex, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        aextail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        aex,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        aextail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        aey,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), aey, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        aeytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        aey,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        aeytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        aez,
        detz.as_mut_ptr(),
    );
    zzlen = scale_expansion_zeroelim(zlen, detz.as_mut_ptr(), aez, detzz.as_mut_ptr());
    ztlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        aeztail,
        detzt.as_mut_ptr(),
    );
    zztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        aez,
        detzzt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < zztlen {
        detzzt[i as usize] *= 2.0f64;
        i += 1;
    }
    ztztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        aeztail,
        detztzt.as_mut_ptr(),
    );
    z1len = fast_expansion_sum_zeroelim(
        zzlen,
        detzz.as_mut_ptr(),
        zztlen,
        detzzt.as_mut_ptr(),
        z1.as_mut_ptr(),
    );
    z2len = fast_expansion_sum_zeroelim(
        z1len,
        z1.as_mut_ptr(),
        ztztlen,
        detztzt.as_mut_ptr(),
        z2.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        z2len,
        z2.as_mut_ptr(),
        xylen,
        detxy.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        dalen,
        da.as_mut_ptr(),
        cez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        dalen,
        da.as_mut_ptr(),
        ceztail,
        temp32b.as_mut_ptr(),
    );
    temp64alen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64a.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        aclen,
        ac.as_mut_ptr(),
        dez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        aclen,
        ac.as_mut_ptr(),
        deztail,
        temp32b.as_mut_ptr(),
    );
    temp64blen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64b.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        cdlen,
        cd.as_mut_ptr(),
        aez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        cdlen,
        cd.as_mut_ptr(),
        aeztail,
        temp32b.as_mut_ptr(),
    );
    temp64clen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64c.as_mut_ptr(),
    );
    temp128len = fast_expansion_sum_zeroelim(
        temp64alen,
        temp64a.as_mut_ptr(),
        temp64blen,
        temp64b.as_mut_ptr(),
        temp128.as_mut_ptr(),
    );
    temp192len = fast_expansion_sum_zeroelim(
        temp64clen,
        temp64c.as_mut_ptr(),
        temp128len,
        temp128.as_mut_ptr(),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        bex,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), bex, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        bextail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        bex,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        bextail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        bey,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), bey, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        beytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        bey,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        beytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        bez,
        detz.as_mut_ptr(),
    );
    zzlen = scale_expansion_zeroelim(zlen, detz.as_mut_ptr(), bez, detzz.as_mut_ptr());
    ztlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        beztail,
        detzt.as_mut_ptr(),
    );
    zztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        bez,
        detzzt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < zztlen {
        detzzt[i as usize] *= 2.0f64;
        i += 1;
    }
    ztztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        beztail,
        detztzt.as_mut_ptr(),
    );
    z1len = fast_expansion_sum_zeroelim(
        zzlen,
        detzz.as_mut_ptr(),
        zztlen,
        detzzt.as_mut_ptr(),
        z1.as_mut_ptr(),
    );
    z2len = fast_expansion_sum_zeroelim(
        z1len,
        z1.as_mut_ptr(),
        ztztlen,
        detztzt.as_mut_ptr(),
        z2.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        z2len,
        z2.as_mut_ptr(),
        xylen,
        detxy.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        ablen,
        ab.as_mut_ptr(),
        -dez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        ablen,
        ab.as_mut_ptr(),
        -deztail,
        temp32b.as_mut_ptr(),
    );
    temp64alen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64a.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        bdlen,
        bd.as_mut_ptr(),
        -aez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        bdlen,
        bd.as_mut_ptr(),
        -aeztail,
        temp32b.as_mut_ptr(),
    );
    temp64blen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64b.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        dalen,
        da.as_mut_ptr(),
        -bez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        dalen,
        da.as_mut_ptr(),
        -beztail,
        temp32b.as_mut_ptr(),
    );
    temp64clen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64c.as_mut_ptr(),
    );
    temp128len = fast_expansion_sum_zeroelim(
        temp64alen,
        temp64a.as_mut_ptr(),
        temp64blen,
        temp64b.as_mut_ptr(),
        temp128.as_mut_ptr(),
    );
    temp192len = fast_expansion_sum_zeroelim(
        temp64clen,
        temp64c.as_mut_ptr(),
        temp128len,
        temp128.as_mut_ptr(),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        cex,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), cex, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        cextail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        cex,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        cextail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        cey,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), cey, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        ceytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        cey,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        ceytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        cez,
        detz.as_mut_ptr(),
    );
    zzlen = scale_expansion_zeroelim(zlen, detz.as_mut_ptr(), cez, detzz.as_mut_ptr());
    ztlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        ceztail,
        detzt.as_mut_ptr(),
    );
    zztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        cez,
        detzzt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < zztlen {
        detzzt[i as usize] *= 2.0f64;
        i += 1;
    }
    ztztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        ceztail,
        detztzt.as_mut_ptr(),
    );
    z1len = fast_expansion_sum_zeroelim(
        zzlen,
        detzz.as_mut_ptr(),
        zztlen,
        detzzt.as_mut_ptr(),
        z1.as_mut_ptr(),
    );
    z2len = fast_expansion_sum_zeroelim(
        z1len,
        z1.as_mut_ptr(),
        ztztlen,
        detztzt.as_mut_ptr(),
        z2.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        z2len,
        z2.as_mut_ptr(),
        xylen,
        detxy.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        bclen,
        bc.as_mut_ptr(),
        aez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        bclen,
        bc.as_mut_ptr(),
        aeztail,
        temp32b.as_mut_ptr(),
    );
    temp64alen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64a.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        aclen,
        ac.as_mut_ptr(),
        -bez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        aclen,
        ac.as_mut_ptr(),
        -beztail,
        temp32b.as_mut_ptr(),
    );
    temp64blen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64b.as_mut_ptr(),
    );
    temp32alen = scale_expansion_zeroelim(
        ablen,
        ab.as_mut_ptr(),
        cez,
        temp32a.as_mut_ptr(),
    );
    temp32blen = scale_expansion_zeroelim(
        ablen,
        ab.as_mut_ptr(),
        ceztail,
        temp32b.as_mut_ptr(),
    );
    temp64clen = fast_expansion_sum_zeroelim(
        temp32alen,
        temp32a.as_mut_ptr(),
        temp32blen,
        temp32b.as_mut_ptr(),
        temp64c.as_mut_ptr(),
    );
    temp128len = fast_expansion_sum_zeroelim(
        temp64alen,
        temp64a.as_mut_ptr(),
        temp64blen,
        temp64b.as_mut_ptr(),
        temp128.as_mut_ptr(),
    );
    temp192len = fast_expansion_sum_zeroelim(
        temp64clen,
        temp64c.as_mut_ptr(),
        temp128len,
        temp128.as_mut_ptr(),
        temp192.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        dex,
        detx.as_mut_ptr(),
    );
    xxlen = scale_expansion_zeroelim(xlen, detx.as_mut_ptr(), dex, detxx.as_mut_ptr());
    xtlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        dextail,
        detxt.as_mut_ptr(),
    );
    xxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        dex,
        detxxt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < xxtlen {
        detxxt[i as usize] *= 2.0f64;
        i += 1;
    }
    xtxtlen = scale_expansion_zeroelim(
        xtlen,
        detxt.as_mut_ptr(),
        dextail,
        detxtxt.as_mut_ptr(),
    );
    x1len = fast_expansion_sum_zeroelim(
        xxlen,
        detxx.as_mut_ptr(),
        xxtlen,
        detxxt.as_mut_ptr(),
        x1.as_mut_ptr(),
    );
    x2len = fast_expansion_sum_zeroelim(
        x1len,
        x1.as_mut_ptr(),
        xtxtlen,
        detxtxt.as_mut_ptr(),
        x2.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        dey,
        dety.as_mut_ptr(),
    );
    yylen = scale_expansion_zeroelim(ylen, dety.as_mut_ptr(), dey, detyy.as_mut_ptr());
    ytlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        deytail,
        detyt.as_mut_ptr(),
    );
    yytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        dey,
        detyyt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < yytlen {
        detyyt[i as usize] *= 2.0f64;
        i += 1;
    }
    ytytlen = scale_expansion_zeroelim(
        ytlen,
        detyt.as_mut_ptr(),
        deytail,
        detytyt.as_mut_ptr(),
    );
    y1len = fast_expansion_sum_zeroelim(
        yylen,
        detyy.as_mut_ptr(),
        yytlen,
        detyyt.as_mut_ptr(),
        y1.as_mut_ptr(),
    );
    y2len = fast_expansion_sum_zeroelim(
        y1len,
        y1.as_mut_ptr(),
        ytytlen,
        detytyt.as_mut_ptr(),
        y2.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        dez,
        detz.as_mut_ptr(),
    );
    zzlen = scale_expansion_zeroelim(zlen, detz.as_mut_ptr(), dez, detzz.as_mut_ptr());
    ztlen = scale_expansion_zeroelim(
        temp192len,
        temp192.as_mut_ptr(),
        deztail,
        detzt.as_mut_ptr(),
    );
    zztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        dez,
        detzzt.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < zztlen {
        detzzt[i as usize] *= 2.0f64;
        i += 1;
    }
    ztztlen = scale_expansion_zeroelim(
        ztlen,
        detzt.as_mut_ptr(),
        deztail,
        detztzt.as_mut_ptr(),
    );
    z1len = fast_expansion_sum_zeroelim(
        zzlen,
        detzz.as_mut_ptr(),
        zztlen,
        detzzt.as_mut_ptr(),
        z1.as_mut_ptr(),
    );
    z2len = fast_expansion_sum_zeroelim(
        z1len,
        z1.as_mut_ptr(),
        ztztlen,
        detztzt.as_mut_ptr(),
        z2.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        x2len,
        x2.as_mut_ptr(),
        y2len,
        y2.as_mut_ptr(),
        detxy.as_mut_ptr(),
    );
    dlen = fast_expansion_sum_zeroelim(
        z2len,
        z2.as_mut_ptr(),
        xylen,
        detxy.as_mut_ptr(),
        ddet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    cdlen = fast_expansion_sum_zeroelim(
        clen,
        cdet.as_mut_ptr(),
        dlen,
        ddet.as_mut_ptr(),
        cddet.as_mut_ptr(),
    );
    deterlen = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        cdlen,
        cddet.as_mut_ptr(),
        deter.as_mut_ptr(),
    );
    return deter[(deterlen - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn insphereadapt(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut pe: *mut libc::c_double,
    mut permanent: libc::c_double,
) -> libc::c_double {
    let mut aex: libc::c_double = 0.;
    let mut bex: libc::c_double = 0.;
    let mut cex: libc::c_double = 0.;
    let mut dex: libc::c_double = 0.;
    let mut aey: libc::c_double = 0.;
    let mut bey: libc::c_double = 0.;
    let mut cey: libc::c_double = 0.;
    let mut dey: libc::c_double = 0.;
    let mut aez: libc::c_double = 0.;
    let mut bez: libc::c_double = 0.;
    let mut cez: libc::c_double = 0.;
    let mut dez: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    let mut aexbey1: libc::c_double = 0.;
    let mut bexaey1: libc::c_double = 0.;
    let mut bexcey1: libc::c_double = 0.;
    let mut cexbey1: libc::c_double = 0.;
    let mut cexdey1: libc::c_double = 0.;
    let mut dexcey1: libc::c_double = 0.;
    let mut dexaey1: libc::c_double = 0.;
    let mut aexdey1: libc::c_double = 0.;
    let mut aexcey1: libc::c_double = 0.;
    let mut cexaey1: libc::c_double = 0.;
    let mut bexdey1: libc::c_double = 0.;
    let mut dexbey1: libc::c_double = 0.;
    let mut aexbey0: libc::c_double = 0.;
    let mut bexaey0: libc::c_double = 0.;
    let mut bexcey0: libc::c_double = 0.;
    let mut cexbey0: libc::c_double = 0.;
    let mut cexdey0: libc::c_double = 0.;
    let mut dexcey0: libc::c_double = 0.;
    let mut dexaey0: libc::c_double = 0.;
    let mut aexdey0: libc::c_double = 0.;
    let mut aexcey0: libc::c_double = 0.;
    let mut cexaey0: libc::c_double = 0.;
    let mut bexdey0: libc::c_double = 0.;
    let mut dexbey0: libc::c_double = 0.;
    let mut ab: [libc::c_double; 4] = [0.; 4];
    let mut bc: [libc::c_double; 4] = [0.; 4];
    let mut cd: [libc::c_double; 4] = [0.; 4];
    let mut da: [libc::c_double; 4] = [0.; 4];
    let mut ac: [libc::c_double; 4] = [0.; 4];
    let mut bd: [libc::c_double; 4] = [0.; 4];
    let mut ab3: libc::c_double = 0.;
    let mut bc3: libc::c_double = 0.;
    let mut cd3: libc::c_double = 0.;
    let mut da3: libc::c_double = 0.;
    let mut ac3: libc::c_double = 0.;
    let mut bd3: libc::c_double = 0.;
    let mut abeps: libc::c_double = 0.;
    let mut bceps: libc::c_double = 0.;
    let mut cdeps: libc::c_double = 0.;
    let mut daeps: libc::c_double = 0.;
    let mut aceps: libc::c_double = 0.;
    let mut bdeps: libc::c_double = 0.;
    let mut temp8a: [libc::c_double; 8] = [0.; 8];
    let mut temp8b: [libc::c_double; 8] = [0.; 8];
    let mut temp8c: [libc::c_double; 8] = [0.; 8];
    let mut temp16: [libc::c_double; 16] = [0.; 16];
    let mut temp24: [libc::c_double; 24] = [0.; 24];
    let mut temp48: [libc::c_double; 48] = [0.; 48];
    let mut temp8alen: libc::c_int = 0;
    let mut temp8blen: libc::c_int = 0;
    let mut temp8clen: libc::c_int = 0;
    let mut temp16len: libc::c_int = 0;
    let mut temp24len: libc::c_int = 0;
    let mut temp48len: libc::c_int = 0;
    let mut xdet: [libc::c_double; 96] = [0.; 96];
    let mut ydet: [libc::c_double; 96] = [0.; 96];
    let mut zdet: [libc::c_double; 96] = [0.; 96];
    let mut xydet: [libc::c_double; 192] = [0.; 192];
    let mut xlen: libc::c_int = 0;
    let mut ylen: libc::c_int = 0;
    let mut zlen: libc::c_int = 0;
    let mut xylen: libc::c_int = 0;
    let mut adet: [libc::c_double; 288] = [0.; 288];
    let mut bdet: [libc::c_double; 288] = [0.; 288];
    let mut cdet: [libc::c_double; 288] = [0.; 288];
    let mut ddet: [libc::c_double; 288] = [0.; 288];
    let mut alen: libc::c_int = 0;
    let mut blen: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut abdet: [libc::c_double; 576] = [0.; 576];
    let mut cddet: [libc::c_double; 576] = [0.; 576];
    let mut ablen: libc::c_int = 0;
    let mut cdlen: libc::c_int = 0;
    let mut fin1: [libc::c_double; 1152] = [0.; 1152];
    let mut finlength: libc::c_int = 0;
    let mut aextail: libc::c_double = 0.;
    let mut bextail: libc::c_double = 0.;
    let mut cextail: libc::c_double = 0.;
    let mut dextail: libc::c_double = 0.;
    let mut aeytail: libc::c_double = 0.;
    let mut beytail: libc::c_double = 0.;
    let mut ceytail: libc::c_double = 0.;
    let mut deytail: libc::c_double = 0.;
    let mut aeztail: libc::c_double = 0.;
    let mut beztail: libc::c_double = 0.;
    let mut ceztail: libc::c_double = 0.;
    let mut deztail: libc::c_double = 0.;
    let mut bvirt: libc::c_double = 0.;
    let mut avirt: libc::c_double = 0.;
    let mut bround: libc::c_double = 0.;
    let mut around: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut abig: libc::c_double = 0.;
    let mut ahi: libc::c_double = 0.;
    let mut alo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut blo: libc::c_double = 0.;
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut err3: libc::c_double = 0.;
    let mut _i: libc::c_double = 0.;
    let mut _j: libc::c_double = 0.;
    let mut _0: libc::c_double = 0.;
    aex = *pa.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bex = *pb.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    cex = *pc.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    dex = *pd.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    aey = *pa.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bey = *pb.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    cey = *pc.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    dey = *pd.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    aez = *pa.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bez = *pb.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    cez = *pc.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    dez = *pd.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    aexbey1 = aex * bey;
    c = splitter * aex;
    abig = c - aex;
    ahi = c - abig;
    alo = aex - ahi;
    c = splitter * bey;
    abig = c - bey;
    bhi = c - abig;
    blo = bey - bhi;
    err1 = aexbey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    aexbey0 = alo * blo - err3;
    bexaey1 = bex * aey;
    c = splitter * bex;
    abig = c - bex;
    ahi = c - abig;
    alo = bex - ahi;
    c = splitter * aey;
    abig = c - aey;
    bhi = c - abig;
    blo = aey - bhi;
    err1 = bexaey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bexaey0 = alo * blo - err3;
    _i = aexbey0 - bexaey0;
    bvirt = aexbey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bexaey0;
    around = aexbey0 - avirt;
    ab[0 as libc::c_int as usize] = around + bround;
    _j = aexbey1 + _i;
    bvirt = _j - aexbey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = aexbey1 - avirt;
    _0 = around + bround;
    _i = _0 - bexaey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bexaey1;
    around = _0 - avirt;
    ab[1 as libc::c_int as usize] = around + bround;
    ab3 = _j + _i;
    bvirt = ab3 - _j;
    avirt = ab3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as libc::c_int as usize] = around + bround;
    ab[3 as libc::c_int as usize] = ab3;
    bexcey1 = bex * cey;
    c = splitter * bex;
    abig = c - bex;
    ahi = c - abig;
    alo = bex - ahi;
    c = splitter * cey;
    abig = c - cey;
    bhi = c - abig;
    blo = cey - bhi;
    err1 = bexcey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bexcey0 = alo * blo - err3;
    cexbey1 = cex * bey;
    c = splitter * cex;
    abig = c - cex;
    ahi = c - abig;
    alo = cex - ahi;
    c = splitter * bey;
    abig = c - bey;
    bhi = c - abig;
    blo = bey - bhi;
    err1 = cexbey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cexbey0 = alo * blo - err3;
    _i = bexcey0 - cexbey0;
    bvirt = bexcey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cexbey0;
    around = bexcey0 - avirt;
    bc[0 as libc::c_int as usize] = around + bround;
    _j = bexcey1 + _i;
    bvirt = _j - bexcey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bexcey1 - avirt;
    _0 = around + bround;
    _i = _0 - cexbey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cexbey1;
    around = _0 - avirt;
    bc[1 as libc::c_int as usize] = around + bround;
    bc3 = _j + _i;
    bvirt = bc3 - _j;
    avirt = bc3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as libc::c_int as usize] = around + bround;
    bc[3 as libc::c_int as usize] = bc3;
    cexdey1 = cex * dey;
    c = splitter * cex;
    abig = c - cex;
    ahi = c - abig;
    alo = cex - ahi;
    c = splitter * dey;
    abig = c - dey;
    bhi = c - abig;
    blo = dey - bhi;
    err1 = cexdey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cexdey0 = alo * blo - err3;
    dexcey1 = dex * cey;
    c = splitter * dex;
    abig = c - dex;
    ahi = c - abig;
    alo = dex - ahi;
    c = splitter * cey;
    abig = c - cey;
    bhi = c - abig;
    blo = cey - bhi;
    err1 = dexcey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dexcey0 = alo * blo - err3;
    _i = cexdey0 - dexcey0;
    bvirt = cexdey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dexcey0;
    around = cexdey0 - avirt;
    cd[0 as libc::c_int as usize] = around + bround;
    _j = cexdey1 + _i;
    bvirt = _j - cexdey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cexdey1 - avirt;
    _0 = around + bround;
    _i = _0 - dexcey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dexcey1;
    around = _0 - avirt;
    cd[1 as libc::c_int as usize] = around + bround;
    cd3 = _j + _i;
    bvirt = cd3 - _j;
    avirt = cd3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    cd[2 as libc::c_int as usize] = around + bround;
    cd[3 as libc::c_int as usize] = cd3;
    dexaey1 = dex * aey;
    c = splitter * dex;
    abig = c - dex;
    ahi = c - abig;
    alo = dex - ahi;
    c = splitter * aey;
    abig = c - aey;
    bhi = c - abig;
    blo = aey - bhi;
    err1 = dexaey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dexaey0 = alo * blo - err3;
    aexdey1 = aex * dey;
    c = splitter * aex;
    abig = c - aex;
    ahi = c - abig;
    alo = aex - ahi;
    c = splitter * dey;
    abig = c - dey;
    bhi = c - abig;
    blo = dey - bhi;
    err1 = aexdey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    aexdey0 = alo * blo - err3;
    _i = dexaey0 - aexdey0;
    bvirt = dexaey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - aexdey0;
    around = dexaey0 - avirt;
    da[0 as libc::c_int as usize] = around + bround;
    _j = dexaey1 + _i;
    bvirt = _j - dexaey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = dexaey1 - avirt;
    _0 = around + bround;
    _i = _0 - aexdey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - aexdey1;
    around = _0 - avirt;
    da[1 as libc::c_int as usize] = around + bround;
    da3 = _j + _i;
    bvirt = da3 - _j;
    avirt = da3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    da[2 as libc::c_int as usize] = around + bround;
    da[3 as libc::c_int as usize] = da3;
    aexcey1 = aex * cey;
    c = splitter * aex;
    abig = c - aex;
    ahi = c - abig;
    alo = aex - ahi;
    c = splitter * cey;
    abig = c - cey;
    bhi = c - abig;
    blo = cey - bhi;
    err1 = aexcey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    aexcey0 = alo * blo - err3;
    cexaey1 = cex * aey;
    c = splitter * cex;
    abig = c - cex;
    ahi = c - abig;
    alo = cex - ahi;
    c = splitter * aey;
    abig = c - aey;
    bhi = c - abig;
    blo = aey - bhi;
    err1 = cexaey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cexaey0 = alo * blo - err3;
    _i = aexcey0 - cexaey0;
    bvirt = aexcey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cexaey0;
    around = aexcey0 - avirt;
    ac[0 as libc::c_int as usize] = around + bround;
    _j = aexcey1 + _i;
    bvirt = _j - aexcey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = aexcey1 - avirt;
    _0 = around + bround;
    _i = _0 - cexaey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cexaey1;
    around = _0 - avirt;
    ac[1 as libc::c_int as usize] = around + bround;
    ac3 = _j + _i;
    bvirt = ac3 - _j;
    avirt = ac3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ac[2 as libc::c_int as usize] = around + bround;
    ac[3 as libc::c_int as usize] = ac3;
    bexdey1 = bex * dey;
    c = splitter * bex;
    abig = c - bex;
    ahi = c - abig;
    alo = bex - ahi;
    c = splitter * dey;
    abig = c - dey;
    bhi = c - abig;
    blo = dey - bhi;
    err1 = bexdey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bexdey0 = alo * blo - err3;
    dexbey1 = dex * bey;
    c = splitter * dex;
    abig = c - dex;
    ahi = c - abig;
    alo = dex - ahi;
    c = splitter * bey;
    abig = c - bey;
    bhi = c - abig;
    blo = bey - bhi;
    err1 = dexbey1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    dexbey0 = alo * blo - err3;
    _i = bexdey0 - dexbey0;
    bvirt = bexdey0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dexbey0;
    around = bexdey0 - avirt;
    bd[0 as libc::c_int as usize] = around + bround;
    _j = bexdey1 + _i;
    bvirt = _j - bexdey1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bexdey1 - avirt;
    _0 = around + bround;
    _i = _0 - dexbey1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - dexbey1;
    around = _0 - avirt;
    bd[1 as libc::c_int as usize] = around + bround;
    bd3 = _j + _i;
    bvirt = bd3 - _j;
    avirt = bd3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bd[2 as libc::c_int as usize] = around + bround;
    bd[3 as libc::c_int as usize] = bd3;
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        bez,
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bd.as_mut_ptr(),
        -cez,
        temp8b.as_mut_ptr(),
    );
    temp8clen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        dez,
        temp8c.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp24len = fast_expansion_sum_zeroelim(
        temp8clen,
        temp8c.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        temp24.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        aex,
        temp48.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        -aex,
        xdet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        aey,
        temp48.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        -aey,
        ydet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        aez,
        temp48.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        -aez,
        zdet.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        xdet.as_mut_ptr(),
        ylen,
        ydet.as_mut_ptr(),
        xydet.as_mut_ptr(),
    );
    alen = fast_expansion_sum_zeroelim(
        xylen,
        xydet.as_mut_ptr(),
        zlen,
        zdet.as_mut_ptr(),
        adet.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        cez,
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ac.as_mut_ptr(),
        dez,
        temp8b.as_mut_ptr(),
    );
    temp8clen = scale_expansion_zeroelim(
        4 as libc::c_int,
        cd.as_mut_ptr(),
        aez,
        temp8c.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp24len = fast_expansion_sum_zeroelim(
        temp8clen,
        temp8c.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        temp24.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        bex,
        temp48.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        bex,
        xdet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        bey,
        temp48.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        bey,
        ydet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        bez,
        temp48.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        bez,
        zdet.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        xdet.as_mut_ptr(),
        ylen,
        ydet.as_mut_ptr(),
        xydet.as_mut_ptr(),
    );
    blen = fast_expansion_sum_zeroelim(
        xylen,
        xydet.as_mut_ptr(),
        zlen,
        zdet.as_mut_ptr(),
        bdet.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        dez,
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bd.as_mut_ptr(),
        aez,
        temp8b.as_mut_ptr(),
    );
    temp8clen = scale_expansion_zeroelim(
        4 as libc::c_int,
        da.as_mut_ptr(),
        bez,
        temp8c.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp24len = fast_expansion_sum_zeroelim(
        temp8clen,
        temp8c.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        temp24.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        cex,
        temp48.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        -cex,
        xdet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        cey,
        temp48.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        -cey,
        ydet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        cez,
        temp48.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        -cez,
        zdet.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        xdet.as_mut_ptr(),
        ylen,
        ydet.as_mut_ptr(),
        xydet.as_mut_ptr(),
    );
    clen = fast_expansion_sum_zeroelim(
        xylen,
        xydet.as_mut_ptr(),
        zlen,
        zdet.as_mut_ptr(),
        cdet.as_mut_ptr(),
    );
    temp8alen = scale_expansion_zeroelim(
        4 as libc::c_int,
        bc.as_mut_ptr(),
        aez,
        temp8a.as_mut_ptr(),
    );
    temp8blen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ac.as_mut_ptr(),
        -bez,
        temp8b.as_mut_ptr(),
    );
    temp8clen = scale_expansion_zeroelim(
        4 as libc::c_int,
        ab.as_mut_ptr(),
        cez,
        temp8c.as_mut_ptr(),
    );
    temp16len = fast_expansion_sum_zeroelim(
        temp8alen,
        temp8a.as_mut_ptr(),
        temp8blen,
        temp8b.as_mut_ptr(),
        temp16.as_mut_ptr(),
    );
    temp24len = fast_expansion_sum_zeroelim(
        temp8clen,
        temp8c.as_mut_ptr(),
        temp16len,
        temp16.as_mut_ptr(),
        temp24.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        dex,
        temp48.as_mut_ptr(),
    );
    xlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        dex,
        xdet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        dey,
        temp48.as_mut_ptr(),
    );
    ylen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        dey,
        ydet.as_mut_ptr(),
    );
    temp48len = scale_expansion_zeroelim(
        temp24len,
        temp24.as_mut_ptr(),
        dez,
        temp48.as_mut_ptr(),
    );
    zlen = scale_expansion_zeroelim(
        temp48len,
        temp48.as_mut_ptr(),
        dez,
        zdet.as_mut_ptr(),
    );
    xylen = fast_expansion_sum_zeroelim(
        xlen,
        xdet.as_mut_ptr(),
        ylen,
        ydet.as_mut_ptr(),
        xydet.as_mut_ptr(),
    );
    dlen = fast_expansion_sum_zeroelim(
        xylen,
        xydet.as_mut_ptr(),
        zlen,
        zdet.as_mut_ptr(),
        ddet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    cdlen = fast_expansion_sum_zeroelim(
        clen,
        cdet.as_mut_ptr(),
        dlen,
        ddet.as_mut_ptr(),
        cddet.as_mut_ptr(),
    );
    finlength = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        cdlen,
        cddet.as_mut_ptr(),
        fin1.as_mut_ptr(),
    );
    det = estimate(finlength, fin1.as_mut_ptr());
    errbound = isperrboundB * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = *pa.offset(0 as libc::c_int as isize) - aex;
    avirt = aex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pa.offset(0 as libc::c_int as isize) - avirt;
    aextail = around + bround;
    bvirt = *pa.offset(1 as libc::c_int as isize) - aey;
    avirt = aey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pa.offset(1 as libc::c_int as isize) - avirt;
    aeytail = around + bround;
    bvirt = *pa.offset(2 as libc::c_int as isize) - aez;
    avirt = aez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pa.offset(2 as libc::c_int as isize) - avirt;
    aeztail = around + bround;
    bvirt = *pb.offset(0 as libc::c_int as isize) - bex;
    avirt = bex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pb.offset(0 as libc::c_int as isize) - avirt;
    bextail = around + bround;
    bvirt = *pb.offset(1 as libc::c_int as isize) - bey;
    avirt = bey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pb.offset(1 as libc::c_int as isize) - avirt;
    beytail = around + bround;
    bvirt = *pb.offset(2 as libc::c_int as isize) - bez;
    avirt = bez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pb.offset(2 as libc::c_int as isize) - avirt;
    beztail = around + bround;
    bvirt = *pc.offset(0 as libc::c_int as isize) - cex;
    avirt = cex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pc.offset(0 as libc::c_int as isize) - avirt;
    cextail = around + bround;
    bvirt = *pc.offset(1 as libc::c_int as isize) - cey;
    avirt = cey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pc.offset(1 as libc::c_int as isize) - avirt;
    ceytail = around + bround;
    bvirt = *pc.offset(2 as libc::c_int as isize) - cez;
    avirt = cez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pc.offset(2 as libc::c_int as isize) - avirt;
    ceztail = around + bround;
    bvirt = *pd.offset(0 as libc::c_int as isize) - dex;
    avirt = dex + bvirt;
    bround = bvirt - *pe.offset(0 as libc::c_int as isize);
    around = *pd.offset(0 as libc::c_int as isize) - avirt;
    dextail = around + bround;
    bvirt = *pd.offset(1 as libc::c_int as isize) - dey;
    avirt = dey + bvirt;
    bround = bvirt - *pe.offset(1 as libc::c_int as isize);
    around = *pd.offset(1 as libc::c_int as isize) - avirt;
    deytail = around + bround;
    bvirt = *pd.offset(2 as libc::c_int as isize) - dez;
    avirt = dez + bvirt;
    bround = bvirt - *pe.offset(2 as libc::c_int as isize);
    around = *pd.offset(2 as libc::c_int as isize) - avirt;
    deztail = around + bround;
    if aextail == 0.0f64 && aeytail == 0.0f64 && aeztail == 0.0f64 && bextail == 0.0f64
        && beytail == 0.0f64 && beztail == 0.0f64 && cextail == 0.0f64
        && ceytail == 0.0f64 && ceztail == 0.0f64 && dextail == 0.0f64
        && deytail == 0.0f64 && deztail == 0.0f64
    {
        return det;
    }
    errbound = isperrboundC * permanent
        + resulterrbound * (if det >= 0.0f64 { det } else { -det });
    abeps = aex * beytail + bey * aextail - (aey * bextail + bex * aeytail);
    bceps = bex * ceytail + cey * bextail - (bey * cextail + cex * beytail);
    cdeps = cex * deytail + dey * cextail - (cey * dextail + dex * ceytail);
    daeps = dex * aeytail + aey * dextail - (dey * aextail + aex * deytail);
    aceps = aex * ceytail + cey * aextail - (aey * cextail + cex * aeytail);
    bdeps = bex * deytail + dey * bextail - (bey * dextail + dex * beytail);
    det
        += (bex * bex + bey * bey + bez * bez)
            * (cez * daeps + dez * aceps + aez * cdeps
                + (ceztail * da3 + deztail * ac3 + aeztail * cd3))
            + (dex * dex + dey * dey + dez * dez)
                * (aez * bceps - bez * aceps + cez * abeps
                    + (aeztail * bc3 - beztail * ac3 + ceztail * ab3))
            - ((aex * aex + aey * aey + aez * aez)
                * (bez * cdeps - cez * bdeps + dez * bceps
                    + (beztail * cd3 - ceztail * bd3 + deztail * bc3))
                + (cex * cex + cey * cey + cez * cez)
                    * (dez * abeps + aez * bdeps + bez * daeps
                        + (deztail * ab3 + aeztail * bd3 + beztail * da3)))
            + 2.0f64
                * ((bex * bextail + bey * beytail + bez * beztail)
                    * (cez * da3 + dez * ac3 + aez * cd3)
                    + (dex * dextail + dey * deytail + dez * deztail)
                        * (aez * bc3 - bez * ac3 + cez * ab3)
                    - ((aex * aextail + aey * aeytail + aez * aeztail)
                        * (bez * cd3 - cez * bd3 + dez * bc3)
                        + (cex * cextail + cey * ceytail + cez * ceztail)
                            * (dez * ab3 + aez * bd3 + bez * da3)));
    if det >= errbound || -det >= errbound {
        return det;
    }
    return insphereexact(pa, pb, pc, pd, pe);
}
#[no_mangle]
pub unsafe extern "C" fn insphere(
    mut pa: *mut libc::c_double,
    mut pb: *mut libc::c_double,
    mut pc: *mut libc::c_double,
    mut pd: *mut libc::c_double,
    mut pe: *mut libc::c_double,
) -> libc::c_double {
    let mut aex: libc::c_double = 0.;
    let mut bex: libc::c_double = 0.;
    let mut cex: libc::c_double = 0.;
    let mut dex: libc::c_double = 0.;
    let mut aey: libc::c_double = 0.;
    let mut bey: libc::c_double = 0.;
    let mut cey: libc::c_double = 0.;
    let mut dey: libc::c_double = 0.;
    let mut aez: libc::c_double = 0.;
    let mut bez: libc::c_double = 0.;
    let mut cez: libc::c_double = 0.;
    let mut dez: libc::c_double = 0.;
    let mut aexbey: libc::c_double = 0.;
    let mut bexaey: libc::c_double = 0.;
    let mut bexcey: libc::c_double = 0.;
    let mut cexbey: libc::c_double = 0.;
    let mut cexdey: libc::c_double = 0.;
    let mut dexcey: libc::c_double = 0.;
    let mut dexaey: libc::c_double = 0.;
    let mut aexdey: libc::c_double = 0.;
    let mut aexcey: libc::c_double = 0.;
    let mut cexaey: libc::c_double = 0.;
    let mut bexdey: libc::c_double = 0.;
    let mut dexbey: libc::c_double = 0.;
    let mut alift: libc::c_double = 0.;
    let mut blift: libc::c_double = 0.;
    let mut clift: libc::c_double = 0.;
    let mut dlift: libc::c_double = 0.;
    let mut ab: libc::c_double = 0.;
    let mut bc: libc::c_double = 0.;
    let mut cd: libc::c_double = 0.;
    let mut da: libc::c_double = 0.;
    let mut ac: libc::c_double = 0.;
    let mut bd: libc::c_double = 0.;
    let mut abc: libc::c_double = 0.;
    let mut bcd: libc::c_double = 0.;
    let mut cda: libc::c_double = 0.;
    let mut dab: libc::c_double = 0.;
    let mut aezplus: libc::c_double = 0.;
    let mut bezplus: libc::c_double = 0.;
    let mut cezplus: libc::c_double = 0.;
    let mut dezplus: libc::c_double = 0.;
    let mut aexbeyplus: libc::c_double = 0.;
    let mut bexaeyplus: libc::c_double = 0.;
    let mut bexceyplus: libc::c_double = 0.;
    let mut cexbeyplus: libc::c_double = 0.;
    let mut cexdeyplus: libc::c_double = 0.;
    let mut dexceyplus: libc::c_double = 0.;
    let mut dexaeyplus: libc::c_double = 0.;
    let mut aexdeyplus: libc::c_double = 0.;
    let mut aexceyplus: libc::c_double = 0.;
    let mut cexaeyplus: libc::c_double = 0.;
    let mut bexdeyplus: libc::c_double = 0.;
    let mut dexbeyplus: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    let mut permanent: libc::c_double = 0.;
    let mut errbound: libc::c_double = 0.;
    aex = *pa.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    bex = *pb.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    cex = *pc.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    dex = *pd.offset(0 as libc::c_int as isize) - *pe.offset(0 as libc::c_int as isize);
    aey = *pa.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    bey = *pb.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    cey = *pc.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    dey = *pd.offset(1 as libc::c_int as isize) - *pe.offset(1 as libc::c_int as isize);
    aez = *pa.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    bez = *pb.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    cez = *pc.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    dez = *pd.offset(2 as libc::c_int as isize) - *pe.offset(2 as libc::c_int as isize);
    aexbey = aex * bey;
    bexaey = bex * aey;
    ab = aexbey - bexaey;
    bexcey = bex * cey;
    cexbey = cex * bey;
    bc = bexcey - cexbey;
    cexdey = cex * dey;
    dexcey = dex * cey;
    cd = cexdey - dexcey;
    dexaey = dex * aey;
    aexdey = aex * dey;
    da = dexaey - aexdey;
    aexcey = aex * cey;
    cexaey = cex * aey;
    ac = aexcey - cexaey;
    bexdey = bex * dey;
    dexbey = dex * bey;
    bd = bexdey - dexbey;
    abc = aez * bc - bez * ac + cez * ab;
    bcd = bez * cd - cez * bd + dez * bc;
    cda = cez * da + dez * ac + aez * cd;
    dab = dez * ab + aez * bd + bez * da;
    alift = aex * aex + aey * aey + aez * aez;
    blift = bex * bex + bey * bey + bez * bez;
    clift = cex * cex + cey * cey + cez * cez;
    dlift = dex * dex + dey * dey + dez * dez;
    det = dlift * abc - clift * dab + (blift * cda - alift * bcd);
    aezplus = if aez >= 0.0f64 { aez } else { -aez };
    bezplus = if bez >= 0.0f64 { bez } else { -bez };
    cezplus = if cez >= 0.0f64 { cez } else { -cez };
    dezplus = if dez >= 0.0f64 { dez } else { -dez };
    aexbeyplus = if aexbey >= 0.0f64 { aexbey } else { -aexbey };
    bexaeyplus = if bexaey >= 0.0f64 { bexaey } else { -bexaey };
    bexceyplus = if bexcey >= 0.0f64 { bexcey } else { -bexcey };
    cexbeyplus = if cexbey >= 0.0f64 { cexbey } else { -cexbey };
    cexdeyplus = if cexdey >= 0.0f64 { cexdey } else { -cexdey };
    dexceyplus = if dexcey >= 0.0f64 { dexcey } else { -dexcey };
    dexaeyplus = if dexaey >= 0.0f64 { dexaey } else { -dexaey };
    aexdeyplus = if aexdey >= 0.0f64 { aexdey } else { -aexdey };
    aexceyplus = if aexcey >= 0.0f64 { aexcey } else { -aexcey };
    cexaeyplus = if cexaey >= 0.0f64 { cexaey } else { -cexaey };
    bexdeyplus = if bexdey >= 0.0f64 { bexdey } else { -bexdey };
    dexbeyplus = if dexbey >= 0.0f64 { dexbey } else { -dexbey };
    permanent = ((cexdeyplus + dexceyplus) * bezplus
        + (dexbeyplus + bexdeyplus) * cezplus + (bexceyplus + cexbeyplus) * dezplus)
        * alift
        + ((dexaeyplus + aexdeyplus) * cezplus + (aexceyplus + cexaeyplus) * dezplus
            + (cexdeyplus + dexceyplus) * aezplus) * blift
        + ((aexbeyplus + bexaeyplus) * dezplus + (bexdeyplus + dexbeyplus) * aezplus
            + (dexaeyplus + aexdeyplus) * bezplus) * clift
        + ((bexceyplus + cexbeyplus) * aezplus + (cexaeyplus + aexceyplus) * bezplus
            + (aexbeyplus + bexaeyplus) * cezplus) * dlift;
    errbound = isperrboundA * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    return insphereadapt(pa, pb, pc, pd, pe, permanent);
}
