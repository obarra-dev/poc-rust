use std::collections::VecDeque;

// A double-ended queue implemented with a growable ring buffer.
#[test]
fn vec_deque_as_queue() {
    let mut vec_deque = VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_back(4);
    vec_deque.push_back(6);

    assert_eq!(vec_deque.get(1), Some(&4));

    let option = vec_deque.pop_front();
    assert_eq!(option, Some(1));
    assert_eq!(vec_deque, VecDeque::from([4, 6]));
}

#[test]
fn vec_deque_as_stack() {
    let mut vec_deque = VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_back(4);
    vec_deque.push_back(6);

    let option = vec_deque.pop_back();
    assert_eq!(option, Some(6));
    assert_eq!(vec_deque, VecDeque::from([1, 4]));
}

