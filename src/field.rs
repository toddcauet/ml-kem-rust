use crate::params::Q;

const M: i32 = ((1 << 26) + (Q as i32 >> 1)) / Q as i32;

pub(crate) fn barrett_reduction(x: i16) -> i16 {
    let t = ((x as i32 * M) + (1 << 25)) >> 26;
    (x as i32 - t * Q as i32) as i16
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
}
