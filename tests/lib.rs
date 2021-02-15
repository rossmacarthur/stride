mod iter;
mod ops;

use stride::Stride;

#[test]
fn stride_debug() {
    let stride = Stride::<_, 1>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(format!("{:?}", stride), "[1, 2, 3, 4, 5]");

    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(format!("{:?}", stride), "[1, 3, 5]");

    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(format!("{:?}", stride), "[1, 4]");
}

#[test]
fn stride_default() {
    let stride: &Stride<i64, 3> = Default::default();
    assert_eq!(stride.len(), 0);
}

#[test]
fn stride_mut_default() {
    let stride: &mut Stride<i64, 3> = Default::default();
    assert_eq!(stride.len(), 0);
}

#[test]
fn stride_new_multiple() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride[0], 1);
    assert_eq!(stride[1], 4);
}

#[test]
fn stride_new_non_multiple() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4]);
    assert_eq!(stride[0], 1);
    assert_eq!(stride[1], 4);
}

#[test]
fn stride_len() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.len(), 2);
}

#[test]
fn stride_len_non_multiple() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(stride.len(), 2);
}

#[test]
fn stride_first() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.first(), Some(&1));

    let stride = <&Stride<i32, 2>>::default();
    assert_eq!(stride.first(), None);
}

#[test]
fn stride_first_mut() {
    let data = &mut [2, 2, 3, 4, 5, 6];
    let stride = Stride::<_, 2>::new_mut(data);
    *stride.first_mut().unwrap() = 1;
    assert_eq!(stride.first(), Some(&1));
}

#[test]
fn stride_first_mut_empty() {
    let stride = <&mut Stride<i32, 2>>::default();
    assert_eq!(stride.first_mut(), None);
}

#[test]
fn stride_last() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.last(), Some(&5));

    let stride = Stride::<_, 2>::new(&[1, 2, 3]);
    assert_eq!(stride.last(), Some(&3));

    let stride = Stride::<_, 2>::new(&[1]);
    assert_eq!(stride.last(), Some(&1));
}

#[test]
fn stride_last_empty() {
    let stride = <&Stride<i32, 2>>::default();
    assert_eq!(stride.last(), None);
}

#[test]
fn stride_last_mut() {
    let data = &mut [1, 2, 3, 4, 6, 6];
    let stride = Stride::<_, 2>::new_mut(data);
    *stride.last_mut().unwrap() = 5;
    assert_eq!(stride.last(), Some(&5));
}

#[test]
fn stride_last_mut_empty() {
    let stride = <&mut Stride<i32, 2>>::default();
    assert_eq!(stride.last_mut(), None);
}
