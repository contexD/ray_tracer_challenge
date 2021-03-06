use ray_tracer_challenge::tuples::{cross, dot, Color, Point, Tuple, Value, Vector};

#[test]
fn it_creates_a_tuple() {
    let p = Tuple::new(vec![1.0, 2.0, 3.0, 1.0]);
    let val: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0];
    assert_eq!(&val, p.value());
}

#[test]
fn it_creates_a_point() {
    let p = Point::new(vec![1.0, 2.0, 3.0]);
    let val: Vec<f32> = vec![1.0, 2.0, 3.0, 1.0];
    assert_eq!(&val, p.value());
}

#[test]
fn it_creates_a_vector() {
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    let val: Vec<f32> = vec![1.0, 2.0, 3.0, 0.0];
    assert_eq!(&val, v.value());
}

#[test]
fn it_compares_two_tuples_for_equality() {
    let p1 = Point::new(vec![1.0, 2.0, 3.0]);
    let p2 = Point::new(vec![1.00001, 2.0, 3.0]);
    let p3 = Point::new(vec![1.0 - 1.0 * 10.0 * f32::EPSILON, 2.0, 3.0]);
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![1.0, 2.0, 3.0]);

    assert_eq!(false, p1 == p2);
    assert_eq!(false, p1 == p3);
    assert_eq!(false, v1 != v2);
}

#[test]
fn it_adds_two_tuples() {
    let t1 = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let t2 = Tuple::new(vec![-2.0, 3.0, 1.0, 2.0]);
    let sum = t1 + t2;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 4.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_two_tuple_refs() {
    let t1 = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let t2 = Tuple::new(vec![-2.0, 3.0, 1.0, 2.0]);
    let sum = &t1 + &t2;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 4.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_and_a_tuple_ref() {
    let t1 = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let t2 = Tuple::new(vec![-2.0, 3.0, 1.0, 2.0]);
    let sum = t1 + &t2;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 4.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_ref_and_a_tuple() {
    let t1 = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let t2 = Tuple::new(vec![-2.0, 3.0, 1.0, 2.0]);
    let sum = &t1 + t2;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 4.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_and_a_point() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = t + p;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_ref_and_a_point_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &t + &p;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_and_a_point_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = t + &p;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_ref_and_a_point() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &t + p;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_and_a_tuple() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = p + t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_ref_and_a_tuple_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &p + &t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_and_a_tuple_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = p + &t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_ref_and_a_tuple() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &p + t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 3.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_and_a_vector() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = t + v;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_ref_and_a_vector_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = &t + &v;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_and_a_vector_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = t + &v;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_tuple_ref_and_a_vector() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = &t + v;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_and_a_tuple() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = v + t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_ref_and_a_tuple_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = &v + &t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_and_a_tuple_ref() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = v + &t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_ref_and_a_tuple() {
    let t = Tuple::new(vec![3.0, -2.0, 5.0, 2.0]);
    let v = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = &v + t;

    let result = Tuple::new(vec![1.0, 1.0, 6.0, 2.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_two_vectors() {
    let v1 = Vector::new(vec![3.0, -2.0, 5.0]);
    let v2 = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = v1 + v2;

    let result = Vector::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_two_vectors_refs() {
    let v1 = Vector::new(vec![3.0, -2.0, 5.0]);
    let v2 = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = &v1 + &v2;

    let result = Vector::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_and_a_vector_ref() {
    let v1 = Vector::new(vec![3.0, -2.0, 5.0]);
    let v2 = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = v1 + &v2;

    let result = Vector::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_ref_and_a_vector() {
    let v1 = Vector::new(vec![3.0, -2.0, 5.0]);
    let v2 = Vector::new(vec![-2.0, 3.0, 1.0]);
    let sum = &v1 + v2;

    let result = Vector::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_and_a_point() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = v + p;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_ref_and_a_point_ref() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &v + &p;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_and_a_point_ref() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = v + &p;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_vector_ref_and_a_point() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &v + p;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_and_a_vector() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = p + v;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_ref_and_a_vector_ref() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &p + &v;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_and_a_vector_ref() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = p + &v;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn it_adds_a_point_ref_and_a_vector() {
    let v = Vector::new(vec![3.0, -2.0, 5.0]);
    let p = Point::new(vec![-2.0, 3.0, 1.0]);
    let sum = &p + v;

    let result = Point::new(vec![1.0, 1.0, 6.0]);

    assert_eq!(result, sum);
}

#[test]
fn subtracting_two_points() {
    let p1 = Point::new(vec![3.0, 2.0, 1.0]);
    let p2 = Point::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, p1 - p2);
}

#[test]
fn subtracting_two_point_refs() {
    let p1 = Point::new(vec![3.0, 2.0, 1.0]);
    let p2 = Point::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, &p1 - &p2);
}

#[test]
fn subtracting_a_point_and_a_point_ref() {
    let p1 = Point::new(vec![3.0, 2.0, 1.0]);
    let p2 = Point::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, p1 - &p2);
}

#[test]
fn subtracting_a_point_ref_and_a_point() {
    let p1 = Point::new(vec![3.0, 2.0, 1.0]);
    let p2 = Point::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, &p1 - p2);
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Point::new(vec![3.0, 2.0, 1.0]);
    let v = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Point::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, p - v);
}

#[test]
fn subtracting_a_vector_ref_from_a_point_ref() {
    let p = Point::new(vec![3.0, 2.0, 1.0]);
    let v = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Point::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, &p - &v);
}

#[test]
fn subtracting_a_vector_ref_from_a_point() {
    let p = Point::new(vec![3.0, 2.0, 1.0]);
    let v = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Point::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, p - &v);
}

#[test]
fn subtracting_a_vector_from_a_point_ref() {
    let p = Point::new(vec![3.0, 2.0, 1.0]);
    let v = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Point::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, &p - v);
}

#[test]
fn subtracting_two_vectors() {
    let v1 = Vector::new(vec![3.0, 2.0, 1.0]);
    let v2 = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, v1 - v2);
}

#[test]
fn subtracting_two_vectors_refs() {
    let v1 = Vector::new(vec![3.0, 2.0, 1.0]);
    let v2 = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, &v1 - &v2);
}

#[test]
fn subtracting_a_vector_ref_from_a_vector() {
    let v1 = Vector::new(vec![3.0, 2.0, 1.0]);
    let v2 = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, v1 - &v2);
}

#[test]
fn subtracting_a_vector_from_a_vector_ref() {
    let v1 = Vector::new(vec![3.0, 2.0, 1.0]);
    let v2 = Vector::new(vec![5.0, 6.0, 7.0]);

    let result = Vector::new(vec![-2.0, -4.0, -6.0]);

    assert_eq!(result, &v1 - v2);
}

#[test]
fn negating_a_tuple() {
    let t = Tuple::new(vec![1.0, -2.0, 3.0, -4.0]);

    let result: Vec<f32> = vec![-1.0, 2.0, -3.0, 4.0];

    assert_eq!(&result, (-t).value());
}

#[test]
fn negating_a_vector() {
    let v = Vector::new(vec![1.0, -2.0, 3.0]);

    let result: Vec<f32> = vec![-1.0, 2.0, -3.0, 0.0];

    assert_eq!(&result, (-v).value());
}

#[test]
fn mulitplying_a_tuple_by_a_scalar() {
    let t = Tuple::new(vec![1.0, -2.0, 3.0, -4.0]);

    let result: Vec<f32> = vec![3.5, -7.0, 10.5, -14.0];

    assert_eq!(&result, (t * 3.5).value());
}

#[test]
fn mulitplying_a_vector_by_a_scalar() {
    let v = Vector::new(vec![1.0, -2.0, 3.0]);

    let result: Vec<f32> = vec![3.5, -7.0, 10.5, 0.0];

    assert_eq!(&result, (v * 3.5).value());
}

#[test]
fn mulitplying_a_tuple_by_a_fraction() {
    let t = Tuple::new(vec![1.0, -2.0, 3.0, -4.0]);

    let result: Vec<f32> = vec![0.5, -1.0, 1.5, -2.0];

    assert_eq!(&result, (t * 0.5).value());
}

#[test]
fn mulitplying_a_vector_by_a_fraction() {
    let v = Vector::new(vec![1.0, -2.0, 3.0]);

    let result: Vec<f32> = vec![0.5, -1.0, 1.5, 0.0];

    assert_eq!(&result, (v * 0.5).value());
}

#[test]
fn dividing_a_tuple_by_a_scalar() {
    let t = Tuple::new(vec![1.0, -2.0, 3.0, -4.0]);

    let result: Vec<f32> = vec![0.5, -1.0, 1.5, -2.0];

    assert_eq!(&result, (t / 2.0).value());
}

#[test]
fn dividing_a_vector_by_a_scalar() {
    let v = Vector::new(vec![1.0, -2.0, 3.0]);

    let result: Vec<f32> = vec![0.5, -1.0, 1.5, 0.0];

    assert_eq!(&result, (v / 2.0).value());
}

#[test]
fn computing_the_magnitude_of_a_vector() {
    let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
    let v2 = Vector::new(vec![0.0, 1.0, 0.0]);
    let v3 = Vector::new(vec![0.0, 0.0, 1.0]);
    let v4 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v5 = Vector::new(vec![-1.0, -2.0, -3.0]);
    let value: f32 = 14.0;

    assert_eq!(1.0, v1.magnitude());
    assert_eq!(1.0, v2.magnitude());
    assert_eq!(1.0, v3.magnitude());
    assert_eq!(value.sqrt(), v4.magnitude());
    assert_eq!(value.sqrt(), v5.magnitude());
}

#[test]
fn normalizing_a_vector() {
    let v1 = Vector::new(vec![4.0, 0.0, 0.0]);
    let v2 = Vector::new(vec![1.0, 2.0, 3.0]);

    let value: f32 = 14.0;
    let m = value.sqrt();

    let result_v1: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0];
    let result_v2: Vec<f32> = vec![1.0 / m, 2.0 / m, 3.0 / m, 0.0];

    assert_eq!(&result_v1, v1.normalize().value());
    assert_eq!(&result_v2, v2.normalize().value());
}

#[test]
fn the_magnitude_of_a_normalized_vector() {
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    let abs_diff = (1.0 - v.normalize().magnitude()).abs();

    assert_eq!(true, abs_diff < f32::EPSILON);
}

#[test]
fn the_dot_product_of_two_vectors() {
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![2.0, 3.0, 4.0]);

    assert_eq!(20.0, dot(&v1, &v2));
}

#[test]
fn the_cross_product_of_two_vectors() {
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![2.0, 3.0, 4.0]);

    let result1 = vec![-1.0, 2.0, -1.0, 0.0];
    let result2 = vec![1.0, -2.0, 1.0, 0.0];

    assert_eq!(&result1, cross(&v1, &v2).value());
    assert_eq!(&result2, cross(&v2, &v1).value());
}

#[test]
fn colors_are_red_green_blue_tuples() {
    let Color { value } = Color::new(vec![-0.5, 0.4, 1.7]);
    assert_eq!(-0.5, value[0]);
    assert_eq!(0.4, value[1]);
    assert_eq!(1.7, value[2]);
}

#[test]
fn adding_colors() {
    let c1 = Color::new(vec![0.9, 0.6, 0.75]);
    let c2 = Color::new(vec![0.7, 0.1, 0.25]);
    let c3 = c1 + c2;

    let r = Color::new(vec![1.6, 0.7, 1.0]);
    assert_eq!(r, c3);
}

#[test]
fn subtracting_colors() {
    let c1 = Color::new(vec![0.9, 0.6, 0.75]);
    let c2 = Color::new(vec![0.7, 0.1, 0.25]);
    let c3 = c1 - c2;

    let r = Color::new(vec![0.2, 0.5, 0.5]);
    assert_eq!(r, c3);
}

#[test]
fn multiplying_a_color_by_a_scalar() {
    let c1 = Color::new(vec![0.2, 0.3, 0.4]);
    let c2 = c1 * 2.0;

    let r = Color::new(vec![0.4, 0.6, 0.8]);
    assert_eq!(r, c2);
}

#[test]
fn multiplying_two_colors() {
    let c1 = Color::new(vec![1.0, 0.2, 0.4]);
    let c2 = Color::new(vec![0.9, 1.0, 0.1]);
    let c3 = c1 * c2;

    let r = Color::new(vec![0.9, 0.2, 0.04]);
    assert_eq!(r, c3);
}
