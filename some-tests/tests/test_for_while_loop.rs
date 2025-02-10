#[test]
fn for_test() {
    let mut sum = 0;
    for i in 0..5 {
        if i == 1 {
            continue;
        }
        if i == 3 {
            break;
        }
        sum += i;
    }
    assert_eq!(sum, 2);

    let mut sum = 0;
    // from 3 to 2, 2 is not included
    for i in -3..2 {
        sum += i;
    }
    assert_eq!(sum, -5);

    // from 3 to 2, 2 is  included
    let mut sum = 0;
    for i in -3..=2 {
        sum += i;
    }
    assert_eq!(sum, -3);

    let mut sum = 0;
    for i in 'a'..='c' {
        sum += i as i32;
    }
    assert_eq!(sum, 294);

    let names = [String::from("omar"), String::from("barra")];
    assert_eq!(names.len(), 2);
    for name in names {
        assert!(name == "omar" || name == "barra");
    }
    // does not compile, the for tool the ownership of the values in the array
    // assert_eq!(names.len(), 2);

    let names = [String::from("omar"), String::from("barra")];
    // to avoid taking the ownership, we can use iter()
    for name in names.iter() {
        assert!(name == "omar" || name == "barra");
    }
    assert_eq!(names.len(), 2);

    // other way: to avoid taking the ownership, we can use &
    for name in &names {
        assert!(name == "omar" || name == "barra");
    }
    assert_eq!(names.len(), 2);

    let a = [3, 2, 3];

    let mut index: usize = 0;
    let mut sum = 0;
    // enumerate returns a tuple with the index and the value
    for (i, v) in a.iter().enumerate() {
        index = index + i + 1;
        sum = sum + v;
    }
    assert_eq!(index, 6);
    assert_eq!(sum, 8);
}

#[test]
fn while_test() {
    let mut index: u8 = 0;
    while index < 10 {
        index += 1;
    }
    assert_eq!(index, 10);
}

#[test]
fn loop_test() {
    let mut count: u8 = 0;

    // infinite loop, usually used together with break and continue
    loop {
        count += 1;

        if count == 3 || count == 7 {
            continue;
        }

        if count == 10 {
            break;
        }
    }
    assert_eq!(count, 10);

    // loop is an expression, so it can return a value
    let mut count: u8 = 0;
    let r = loop {
        count += 1;
        if count == 10 {
            break count + 1;
        }
    };
    assert_eq!(r, 11);

    // Nesting and labels with loop
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }
    assert_eq!(count, 30);
}
