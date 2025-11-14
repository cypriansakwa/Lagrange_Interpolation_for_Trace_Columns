use ark_bn254::Fr;
use ark_ff::{AdditiveGroup, Field, One, Zero};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};

// Manual Lagrange interpolation returning coefficients
fn lagrange_interpolate(xs: &[Fr], ys: &[Fr]) -> DensePolynomial<Fr> {
    assert_eq!(xs.len(), ys.len());
    let n = xs.len();
    let mut coeffs = vec![Fr::zero(); n];
    for j in 0..n {
        let mut num = DensePolynomial::from_coefficients_vec(vec![Fr::one()]);
        let mut denom = Fr::one();
        for m in 0..n {
            if m != j {
                num = &num * &DensePolynomial::from_coefficients_vec(vec![-xs[m], Fr::one()]);
                denom *= xs[j] - xs[m];
            }
        }
        let scale = ys[j] * denom.inverse().unwrap();
        let term = num * &DensePolynomial::from_coefficients_vec(vec![scale]);
        for (i, c) in term.coeffs.iter().enumerate() {
            if i >= coeffs.len() {
                coeffs.push(*c);
            } else {
                coeffs[i] += c;
            }
        }
    }
    DensePolynomial::from_coefficients_vec(coeffs)
}

fn main() {
    let t_vals = vec![Fr::from(0u32), Fr::from(1u32), Fr::from(2u32)];
    let x_vals = vec![Fr::from(1u32), Fr::from(2u32), Fr::from(5u32)];
    let y_vals = vec![Fr::from(0u32), Fr::from(1u32), Fr::from(5u32)];

    let poly_x = lagrange_interpolate(&t_vals, &x_vals);
    let poly_y = lagrange_interpolate(&t_vals, &y_vals);

    println!("poly_x coefficients: {:?}", poly_x.coeffs);
    println!("poly_y coefficients: {:?}", poly_y.coeffs);

    for i in 0..t_vals.len() - 1 {
        let x_t = poly_x.evaluate(&t_vals[i]);
        let y_t = poly_y.evaluate(&t_vals[i]);
        let x_next = poly_x.evaluate(&t_vals[i + 1]);
        let y_next = poly_y.evaluate(&t_vals[i + 1]);

        let lhs_x = x_next;
        let rhs_x = x_t.double() + y_t;
        let lhs_y = y_next;
        let rhs_y = x_t + y_t * Fr::from(3u32);

        println!(
            "t={} | x_next={} vs 2*x_t+y_t={}, y_next={} vs x_t+3*y_t={}",
            i, lhs_x, rhs_x, lhs_y, rhs_y
        );

        assert_eq!(lhs_x, rhs_x, "x transition failed at t={}", i);
        assert_eq!(lhs_y, rhs_y, "y transition failed at t={}", i);
    }

    println!("✅ All transitions satisfied.");
}