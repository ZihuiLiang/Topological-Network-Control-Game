use tncg_verifier::{
    corollary3::corollary3_verifier, lemma1::lemma1_verifier, lemma2::lemma2_verifier,
    lemma5::lemma5_verifier, lemma7::lemma7_verifier, lemma8::lemma8_verifier,
    theorem1::theorem1_verifier, theorem2::theorem2_verifier, theorem3::theorem3_verifier,
    theorem4::theorem4_verifier,
};

fn main() {
    lemma1_verifier();
    lemma2_verifier();
    corollary3_verifier();
    theorem1_verifier();
    theorem2_verifier();
    lemma5_verifier();
    lemma7_verifier();
    lemma8_verifier();
    theorem3_verifier();
    theorem4_verifier();
}
