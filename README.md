# Lagrange Interpolation for Trace Columns (Arkworks v0.5.0)
This project demonstrates manual Lagrange interpolation and a STARK-style transition check using the arkworks cryptographic library suite on the BN254 field.

## Overview
The example reconstructs two polynomials `poly_x(t)` and `poly_y(t)` from trace points over the BN254 finite field. These polynomials represent an iterative system governed by:

## Recurrence relations:
 ```bash
  xₜ₊₁ = 2·xₜ + yₜ
yₜ₊₁ = xₜ + 3·yₜ
```
Starting from the initial state (x₀, y₀) = (1, 0), the program performs interpolation and verifies that all transitions satisfy the recurrence correctly.

## Features
- Manual Lagrange interpolation (no built-in interpolation helpers)
- Construction of DensePolynomials using the arkworks v0.5 API
- Correct polynomial evaluation through trait imports
- Verification of consistency in the defined system transitions
## Dependencies
 ```toml
  ark-ff = "0.5.0"
 ark-poly = "0.5.0"
 ark-bn254 = "0.5.0"
 ```

## Run Instructions
1. Clone or create the project directory with the provided code and `Cargo.toml.`
2. In the terminal, run:
   ```bash
    cargo run
   ```
The output will display polynomial coefficients and transition checks:
 ```bash
  poly_x coefficients: [1, 0, 1]
poly_y coefficients: [0, 10944121435919637611123202872628637544274182200208017171849102093287904247808, 10944121435919637611123202872628637544274182200208017171849102093287904247810]
t=0 | x_next=2 vs 2*x_t+y_t=2, y_next=1 vs x_t+3*y_t=1
t=1 | x_next=5 vs 2*x_t+y_t=5, y_next=5 vs x_t+3*y_t=5
✅ All transitions satisfied.
```
