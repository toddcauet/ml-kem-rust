use crate::params::Q;

const M: i32 = (1 << 26) / Q as i32;

pub(crate) fn barrett_reduction(x: i16) -> i16 {
    let res = ((x as i32 * M) + (1 << 25)) >> 26;
    (x as i32 - res * Q as i32) as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_barrett_reduction() {
        for x in i16::MIN..=i16::MAX {
        }
    }
}
