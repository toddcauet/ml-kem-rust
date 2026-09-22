// from FIPS 203 parameter sets

pub(crate) const Q: i16 = 3329;
pub(crate) const N: usize = 256;

pub(crate) mod ml_kem_512 {
    pub(crate) const K: usize = 2;
    pub(crate) const ETA1: usize = 3;
    pub(crate) const ETA2: usize = 2;
    pub(crate) const DU: usize = 10;
    pub(crate) const DV: usize = 4;
    pub(crate) const EK_LEN: usize = 800;
    pub(crate) const DK_LEN: usize = 1632;
    pub(crate) const CT_LEN: usize = 768;
}

pub(crate) mod ml_kem_768 {
    pub(crate) const K: usize = 3;
    pub(crate) const ETA1: usize = 2;
    pub(crate) const ETA2: usize = 2;
    pub(crate) const DU: usize = 10;
    pub(crate) const DV: usize = 4;
    pub(crate) const EK_LEN: usize = 1184;
    pub(crate) const DK_LEN: usize = 2400;
    pub(crate) const CT_LEN: usize = 1088;
}

pub(crate) mod ml_kem_1024 {
    pub(crate) const K: usize = 4;
    pub(crate) const ETA1: usize = 2;
    pub(crate) const ETA2: usize = 2;
    pub(crate) const DU: usize = 11;
    pub(crate) const DV: usize = 5;
    pub(crate) const EK_LEN: usize = 1568;
    pub(crate) const DK_LEN: usize = 3168;
    pub(crate) const CT_LEN: usize = 1568;
}
