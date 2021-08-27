use ray_tracer_challenge::tuples::{Tuple, dot};


#[test]
fn it_creates_a_point() {
    let p = Tuple::new_point(1.0, 2.0, 3.0);
    assert_eq!(Tuple(1.0, 2.0, 3.0, 1.0), p);
}

#[test]
fn it_creates_a_vector() {
    let v = Tuple::new_vector(1.0, 2.0, 3.0);
    assert_eq!(Tuple(1.0, 2.0, 3.0, 0.0), v);
}

#[test]
fn it_detects_a_point() {
    let p = Tuple::new_point(1.0, 2.0, 3.0);
    assert_eq!(true, p.is_point());
}
#[test]
fn it_detects_a_vector() {
    let v = Tuple::new_vector(1.0, 2.0, 3.0);
    assert_eq!(true, v.is_vector());
}

#[test]
fn it_compares_two_tuples_for_equality() {
    let p1 = Tuple::new_point(1.0, 2.0, 3.0);
    let p2 = Tuple::new_point(1.00001, 2.0, 3.0);
    let p3 = Tuple::new_point(1.000001, 2.0, 3.0);
    let v = Tuple::new_vector(1.0, 2.0, 3.0);

    assert_eq!(false, p1 == p2);
    assert_eq!(false, p1 == v);
    assert_eq!(false, p1 != p3);
}

#[test]
fn it_adds_two_tuples() {
    let p = Tuple::new_point(3.0, -2.0, 5.0);
    let v = Tuple::new_vector(-2.0, 3.0, 1.0);

    assert_eq!(Tuple(1.0, 1.0, 6.0, 1.0), p + v);
}

#[test]
fn subtracting_two_points() {
    let p1 = Tuple::new_point(3.0, 2.0, 1.0);
    let p2 = Tuple::new_point(5.0, 6.0, 7.0);

    assert_eq!(Tuple(-2.0, -4.0, -6.0, 0.0), p1 - p2);
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Tuple::new_point(3.0, 2.0, 1.0);
    let v = Tuple::new_vector(5.0, 6.0, 7.0);

    assert_eq!(Tuple(-2.0, -4.0, -6.0, 1.0), p - v);
}

#[test]
fn subtracting_two_vectors() {
    let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
    let v2 = Tuple::new_vector(5.0, 6.0, 7.0);

    assert_eq!(Tuple(-2.0, -4.0, -6.0, 0.0), v1 - v2);
}

#[test]
fn negating_a_tuple() {
    let t = Tuple(1.0, -2.0, 3.0, -4.0);

    assert_eq!(Tuple(-1.0, 2.0, -3.0, 4.0), -t);
}

#[test]
fn mulitplying_a_tuple_by_a_scalar() {
    let t = Tuple(1.0, -2.0, 3.0, -4.0);

    assert_eq!(Tuple(3.5, -7.0, 10.5, -14.0), t * 3.5);
}

#[test]
fn mulitplying_a_tuple_by_a_fraction() {
    let t = Tuple(1.0, -2.0, 3.0, -4.0);

    assert_eq!(Tuple(0.5, -1.0, 1.5, -2.0), t * 0.5);
}

#[test]
fn dividing_a_tuple_by_a_scalar() {
    let t = Tuple(1.0, -2.0, 3.0, -4.0);

    assert_eq!(Tuple(0.5, -1.0, 1.5, -2.0), t / 2.0);
}

#[test]
fn computing_the_magnitude_of_a_vector() {
    let v1 = Tuple::new_vector(1.0, 0.0, 0.0);
    let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
    let v3 = Tuple::new_vector(0.0, 0.0, 1.0);
    let v4 = Tuple::new_vector(1.0, 2.0, 3.0);
    let v5 = Tuple::new_vector(-1.0, -2.0, -3.0);
    let value: f32 = 14.0;

    assert_eq!(1.0, v1.magnitude().unwrap());
    assert_eq!(1.0, v2.magnitude().unwrap());
    assert_eq!(1.0, v3.magnitude().unwrap());
    assert_eq!(value.sqrt(), v4.magnitude().unwrap());
    assert_eq!(value.sqrt(), v5.magnitude().unwrap());
}

#[test]
fn normalizing_a_vector() {
    let v1 = Tuple::new_vector(4.0, 0.0, 0.0);
    let v2 = Tuple::new_vector(1.0, 2.0, 3.0);
    let value: f32 = 14.0;
    let m = value.sqrt();

    assert_eq!(Tuple(1.0, 0.0, 0.0, 0.0), v1.normalize().unwrap());
    assert_eq!(Tuple(1.0 / m, 2.0 / m, 3.0 / m, 0.0), v2.normalize().unwrap());
}

#[test]
fn the_magnitude_of_a_normalized_vector() {
    let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
    let abs_diff = (1.0 - v1.normalize().unwrap().magnitude().unwrap()).abs();

    assert_eq!(true, abs_diff < f32::EPSILON);
}

#[test]
fn the_dot_product_of_two_vectors() {
    let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
    let v2 = Tuple::new_vector(2.0, 3.0, 4.0);

    assert_eq!(20.0, dot(&v1, &v2).unwrap());
}
