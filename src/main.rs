use ndarray;
use ndarray::*;

fn main() {
    // ndarrayのテスト

    // move test
    let v1: Array1<i16> = ndarray::arr1(&[2, 0, -2]);
    let v2: Array1<i16> = ndarray::arr1(&[2, 2, -2]);
    assert_eq!(v1, array![2, 0, -2]);
    // println!("{:?}", v1+v2); // consume v1 & v2
    // println!("{:?}", v1+&v2); // consume v1
    // println!("{:?}", &v1+v2); // compile error
    println!("{:?}", &v1+&v2); // no consume
    let v3: Array1<i16> = ndarray::arr1(&[2, 2, -2, 2]);
    let _ = v2+v3; // runtime error occurred
}
