use std::ops::Add;

use num_bigint::BigUint;
use rand::thread_rng;
use pairing::bn256::{Bn256, Fr};
use ff_ce::{Field,PrimeField,PrimeFieldRepr};

use bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError,
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof
    }
};

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
    
        let start = std::time::SystemTime::now();

        // compute witness
        let mut tmp = Vec::new();

        let mut m = Fr::one();
        for _ in 0..self.n {
            tmp.push(m);
            m.mul_assign(&self.input_p);
        }
        
        let q = tmp[self.n-1].clone();

        // dump q
        let mut qbuff = Vec::new();
        q.clone().into_repr().write_be(&mut qbuff).unwrap();
        println!("q={}",BigUint::from_bytes_be(&qbuff).to_str_radix(10));

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

        println!("Witness time: {:?}",std::time::SystemTime::now().duration_since(start).unwrap());

        Ok(())
    }
}

fn generate() -> String {
    let rng = &mut thread_rng();
    let circuit = TestCircuit {
        n: 50000,
        input_p: Fr::from_str("2").unwrap(),
    };
    
    let proving_key = generate_random_parameters(circuit.clone(), rng).expect("cannot setup");

    let mut proving_key_buffer = Vec::new();
    proving_key.write(&mut proving_key_buffer).unwrap();

    base64::encode(&proving_key_buffer)
}

fn proof(pk : &str, self_verify : bool) -> String {

    let rng = &mut thread_rng();
    let circuit = TestCircuit {
        n: 50000,
        input_p: Fr::from_str("2").unwrap(),
    };

    let proving_key_buffer = base64::decode(pk).unwrap();
    let proving_key = bellman::groth16::Parameters::read(&proving_key_buffer[..],true).unwrap();
    let proof = create_random_proof(circuit, &proving_key, rng).expect("cannot create proof");

    if self_verify {
        let vk = prepare_verifying_key(&proving_key.vk);
        let public_inputs = [ Fr::from_str("19827351477252055523321940833559976079974453644711691740126133490779686437853").unwrap() ];
        let ok = verify_proof(&vk, &proof, &public_inputs).expect("cannot verify proof");

        println!("self_verify_ok={}",ok);
    }

    format!("proof={:?}",proof)
}

fn main() {
    let proving_key = std::fs::read_to_string("provingkey.txt")
        .expect("Something went wrong reading the file");
    println!("{:?}",proof(&proving_key,true));
}