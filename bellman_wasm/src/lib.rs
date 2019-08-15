use pairing::bn256::{Bn256, Fr};

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::create_random_proof;
use std::ops::Add;

use ff_ce::{Field,PrimeField};

use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct TestCircuit {
    n       : usize,
    input_p : Fr,
}

impl TestCircuit {}
impl Circuit<Bn256> for TestCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(
        self,
        cs: &mut CS,
    ) -> std::result::Result<(), SynthesisError> {        

        // compute witness
        let mut tmp = Vec::new();

        let mut m = Fr::one();
        for _ in 0..self.n {
            tmp.push(m);
            m.mul_assign(&self.input_p);
        }
        let q = tmp[self.n-1].clone();

        // register signals
        let mut signals = Vec::new();
        signals.push(cs.alloc(|| "", || Ok(self.input_p)).unwrap());
        signals.push(cs.alloc_input(|| "", || Ok(q)).unwrap());
        tmp.into_iter().enumerate().for_each(
            |(_,w)| 
            signals.push(cs.alloc(|| "", || Ok(w)).unwrap())
        );

        let mut minus_1 = Fr::one();
        minus_1.negate();
        let minus_1 = minus_1;

        // register constrains
        cs.enforce(
            || "1-tmp[0]=0",
            |lc| lc,
            |lc| lc,
            |lc| lc.add((Fr::one(),CS::one())).add((minus_1, signals[2+0]))
        );

        for i in 1..self.n {
            // p * tmp[i-1] - tmp[i] = 0
            cs.enforce(
                || format!("loop({})",i),
                |lc| lc.add((Fr::one(),signals[0])),
                |lc| lc.add((Fr::one(),signals[2+i-1])),
                |lc| lc.add((Fr::one(),signals[2+i])),
            );
        }

        cs.enforce(
            || "q-tmp[N-1]=0",
            |lc| lc,
            |lc| lc,
            |lc| lc.add((Fr::one(),signals[1])).add((minus_1,signals[2+self.n-1]))
        );

        Ok(())
    }
}

#[wasm_bindgen]
pub fn test_speed() {
    let input_p =  Fr::from_str("2").unwrap();
    let mut m = Fr::one();
    for _ in 0..50000 {
        m.mul_assign(&input_p);
    }
}

#[wasm_bindgen]
pub fn proof(proving_key_b64 : &str) -> String {
    let rng = &mut rand::XorShiftRng::new_unseeded();
    let circuit = TestCircuit {
        n: 50000,
        input_p: Fr::from_str("2").unwrap(),
    };
 
    match base64::decode(proving_key_b64) {
        Ok(proving_key_buffer) => {
            match bellman::groth16::Parameters::read(&proving_key_buffer[..],true) {
                Ok(proving_key) => {
                    match create_random_proof(circuit, &proving_key, rng) {
                        Ok(proof) => format!("ok:{:?}",proof),
                        Err(err) => format!("createprooferr:{:?}",err),
                    }
                }
                Err(err) => format!("provingkeyerr:{:?}",err)
            }
        }
        Err(err) => format!("base64err:{:?}",err)
    }
}
