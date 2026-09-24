use crate::params::Q;

const M: i32 = ((1 << 26) + (Q as i32 >> 1)) / Q as i32;
const QINV: i16 = -3327;

pub(crate) fn barrett_reduction(x: i16) -> i16 {
    let t = ((x as i32 * M) + (1 << 25)) >> 26;
    (x as i32 - t * Q as i32) as i16
}

pub(crate) fn montgomery_reduction(x: i32) -> i16 {
    let m = (x as i16).wrapping_mul(QINV);
    ((x - m as i32 * Q as i32) >> 16) as i16
}

pub(crate) fn caddq(x: i16) -> i16 {
    let mask: i16 = (x >> 15) & Q;
    x + mask
}

pub(crate) fn csubq(x: i16) -> i16 {
    caddq(x - Q)
}

pub(crate) fn add(x: i16, y: i16) -> i16 {
    csubq(x + y)
}

pub(crate) fn sub(x: i16, y: i16) -> i16 {
    caddq(x - y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_barrett_reduction() {
        for x in i16::MIN..=i16::MAX {
            let res = barrett_reduction(x);
            assert!((-(Q - 1) / 2..=(Q - 1) / 2).contains(&res));
            assert_eq!(x.rem_euclid(Q), res.rem_euclid(Q));
        }
    }

    #[test]
    fn assert_montgomery_reduction() {
        let bound: i32 = Q as i32 * (1 << 15);
        for x in (-bound + 1)..bound {
            let res = montgomery_reduction(x);
            assert!((-(Q - 1)..=(Q - 1)).contains(&res), "{x} = {res}");
            assert_eq!(
                (res as i64 * 65536).rem_euclid(Q as i64),
                (x as i64).rem_euclid(Q as i64)
            );
        }
    }

    #[test]
    fn assert_caddq() {
        for x in -(Q - 1)..Q {
            let res = caddq(x);
            assert!((0..Q).contains(&res), "{x} = {res}");
            assert_eq!(x.rem_euclid(Q), res.rem_euclid(Q));
        }
    }

    #[test]
    fn assert_csubq() {
        for x in 0..2 * Q {
            let res = csubq(x);
            assert!((0..Q).contains(&res), "{x} = {res}");
            assert_eq!(x.rem_euclid(Q), res.rem_euclid(Q));
        }
    }

    #[test]
    fn assert_add() {
        for x in 0..Q {
            for y in 0..Q {
                let res = add(x, y);
                assert_eq!(res, (x + y).rem_euclid(Q));
            }
        }
    }

    #[test]
    fn assert_sub() {
        for x in 0..Q {
            for y in 0..Q {
                let res = sub(x, y);
                assert_eq!(res, (x - y).rem_euclid(Q));
            }
        }
    }
}
