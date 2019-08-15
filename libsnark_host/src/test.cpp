#include <stdlib.h>
#include <iostream>
#include <chrono>

#include "libff/algebra/fields/field_utils.hpp"
#include "libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp"
#include "libsnark/common/default_types/r1cs_ppzksnark_pp.hpp"
#include "libsnark/gadgetlib1/pb_variable.hpp"

#include "util.hpp"

using namespace libsnark;
using namespace std;
using namespace std::chrono;

int main()
{

  const size_t N = 50000;

  typedef libff::Fr<default_r1cs_ppzksnark_pp> FieldT;

  // Initialize the curve parameters

  default_r1cs_ppzksnark_pp::init_public_params();
  
  // Create protoboard

  protoboard<FieldT> pb;

  // Define variables

  pb_variable<FieldT> q;
  pb_variable<FieldT> p;
  std::vector<pb_variable<FieldT>> tmp(N);

  // Allocate variables to protoboard
  // The strings (like "x") are only for debugging purposes
  
  q.allocate(pb, "q");
  p.allocate(pb, "p");
  for (size_t n=0;n<N;n++) {
      std::ostringstream name;  
      name << "tmp " << n;
      tmp[n].allocate(pb,name.str());
  }

  // This sets up the protoboard variables
  // so that the first one (out) represents the public
  // input and the rest is private input
  pb.set_input_sizes(1);

  // Add R1CS constraints to protoboard

  // tmp[0] == 1
  pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, 1, tmp[0]));

  // tmp[n-1]*p == tmp[n] for n=1..N
  for (size_t n=1;n<N;n++) {
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(tmp[n-1], p, tmp[n]));
  }

  // q == tmp[N-1]
  pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, q, tmp[N-1]));

  // Add witness values
  pb.val(p) = 2;
  pb.val(tmp[0])= pb.val(p);

  // tmp[n-1]*p == tmp[n] for n=1..N
  for (size_t n=1;n<N;n++) {
    pb.val(tmp[n])= pb.val(tmp[n-1]) * pb.val(p);
  }
  pb.val(q) = pb.val(tmp[N-1]);

  const r1cs_constraint_system<FieldT> constraint_system = pb.get_constraint_system();

  const r1cs_gg_ppzksnark_keypair<default_r1cs_ppzksnark_pp> keypair = r1cs_gg_ppzksnark_generator<default_r1cs_ppzksnark_pp>(constraint_system);

  const milliseconds begin_generate_proof = duration_cast< milliseconds >(
      system_clock::now().time_since_epoch()
  );  
  const r1cs_gg_ppzksnark_proof<default_r1cs_ppzksnark_pp> proof = r1cs_gg_ppzksnark_prover<default_r1cs_ppzksnark_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
  const milliseconds end_generate_proof = duration_cast< milliseconds >(
      system_clock::now().time_since_epoch()
  );  

  bool verified = r1cs_gg_ppzksnark_verifier_strong_IC<default_r1cs_ppzksnark_pp>(keypair.vk, pb.primary_input(), proof);

  cout << "Number of R1CS constraints: " << constraint_system.num_constraints() << endl;
  cout << "Verification status: " << verified << endl;
  cout << "Proof generation time: " << (end_generate_proof - begin_generate_proof).count() << "ms" << endl;

  return 0;
}
