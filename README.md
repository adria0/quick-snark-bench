# Simple performance tests for bellman, libsnark and websnark

Made on a i7-8550U, for a ZKP on discrete logaritm for `public_q=secret_p^50000`

**results**

|        | bell-host | bell-wasm | websnark | libsnark-host |
| -------|-----------|-----------|----------|---------------|
| 1 core |   5.030   |  34.000   | 21.588   | 5.652         |
| 8 core |   1.580   |  -        |  6.503   | -             |

- belman-wasm and websnark done in chrome 75 on ubuntu 18.04
- to limit CPU for bell-host and websnark used `taskset -c 0` 

