use slice_get_many_poc::*;

#[test]
fn array_indices() {
    let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let items = array.get_many_mut_poc([2, 8, 5]).unwrap();
    assert_eq!(items, [&mut 2, &mut 8, &mut 5]);

    let [a, b, c] = items;

    *a *= 2;
    *b += 10;
    *c *= 5;

    assert_eq!(array, [0, 1, 4, 3, 4, 25, 6, 7, 18, 9, 10]);
}

#[test]
fn array_range_indices() {
    let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let items = array.get_many_mut_poc([1..4, 6..9]).unwrap();
    assert_eq!(items, [&mut [1, 2, 3], &mut [6, 7, 8]]);

    let [a, b] = items;

    for x in a.iter_mut() {
        *x *= 2;
    }

    for x in b.iter_mut() {
        *x += 5;
    }

    assert_eq!(array, [0, 2, 4, 6, 4, 5, 11, 12, 13, 9, 10]);
}

#[test]
fn tuple_indices() {
    let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let items = array.get_many_mut_poc((2, 5, 7..9)).unwrap();
    assert_eq!(items, (&mut 2, &mut 5, &mut [7, 8] as &mut [_]));

    let (a, b, c) = items;

    *a *= 3;
    *b += 10;

    for x in c.iter_mut() {
        *x *= 10;
    }

    assert_eq!(array, [0, 1, 6, 3, 4, 15, 6, 70, 80, 9, 10]);
}

#[test]
fn checked_distinct_indices() {
    let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    {
        let indices = DisjointIndices::new([0, 5]).unwrap();
        let items = array.get_many_mut_poc(indices).unwrap();
        assert_eq!(items, [&mut 0, &mut 5]);
    }

    {
        let indices = DisjointIndices::new((0, 5..=9)).unwrap();
        let items = array.get_many_mut_poc(indices).unwrap();
        assert_eq!(items, (&mut 0, &mut [5, 6, 7, 8, 9] as &mut [_]));
    }
}
