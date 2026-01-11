use k256::{
    ProjectivePoint, Scalar, U256,
    elliptic_curve::{ops::Reduce, sec1::ToEncodedPoint},
};
use sha2::{Digest, Sha256};

pub struct Transcript {
    pub hasher: Sha256,
}

impl Transcript {
    pub fn new(delimitter: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(delimitter);
        Transcript { hasher }
    }

    pub fn append_point(&mut self, point: &ProjectivePoint) {
        self.hasher.update(point.to_affine().to_encoded_point(true));
    }

    pub fn append_scalar(&mut self, scalar: &Scalar) {
        self.hasher.update(scalar.to_bytes());
    }

    pub fn retrive_challenge(&mut self) -> Scalar {
        let res = self.hasher.clone().finalize();
        let challenge = <Scalar as Reduce<U256>>::reduce_bytes(&res);
        challenge
    }
}
