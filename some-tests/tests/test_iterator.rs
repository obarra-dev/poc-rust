use std::{collections::HashMap, path::Iter};

#[test]
fn for_and_iterator() {
    let arr = [1, 2, 3];
    let mut count = 0;
    for e in arr {
        count += e; 
    }
    assert_eq!(count, 6);

        // it is equivalent to 'for e in arr'
    for e in arr.into_iter() {
        count += e; 
    }
    assert_eq!(count, 12);

    // this does not use iterator
    for i in 0..arr.len() {
        count += arr[i];
    }
    assert_eq!(count, 18);

    let v = vec![1, 2, 3];
    for e in v {
        count += e;
    }
    assert_eq!(count, 24);

    let mut v = Vec::new();
    // 1..100 creates an iterator
    for e in 0..100 {
        v.push(e);
    }
    assert_eq!(v.len(), 100);
}

#[test]
fn next() {
    let mut it = (0..1).into_iter();
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), None);
}

#[test]
fn iterators() {
    let v = vec![1, 2, 3];
    let mut count = 0;
    // into_iter takes ownership of the vector
    for e in v.into_iter() {
        count += e;   
    }
    assert_eq!(count, 6);
    // does not compile, error: value borrowed here after move
   // assert_eq!(v, [1, 2, 3]);


   let v = vec![1, 2, 3];
    let mut count = 0;
    // iter does not take ownership of the vector, it is a immutable borrowing
    for e in v.iter() {
        count += e;   
    }
    assert_eq!(count, 6);
   assert_eq!(v, [1, 2, 3]);


   let mut names = vec!["omar", "barra"];
   // iter_mut takes mutable reference to the vector
   for name in names.iter_mut() {
    *name = match name {
        &mut "barra" => "Barra",
        _ => "Unknown",
        
    }
   }
    assert_eq!(names, ["Unknown", "Barra"]);
}

#[test]
fn iter_mut_modifing_first_element() {
    let mut v = vec![1, 2, 3];
    let mut it = v.iter_mut();
    if let Some(e) = it.next() {
        *e = 4;
    }
    assert_eq!(v, [4, 2, 3]);
}


#[test]
fn custom_iterator() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 2 {
                self.count += 1;
                Some(self.count)
            } else {
                return None;
            }
        }
    }

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), None);
}

#[test]
fn custom_iterator_fibonacci() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;
    
        fn next(&mut self) -> Option<Self::Item> {
            let forward = self.curr + self.next;
    
            self.curr = self.next;
            self.next = forward;
    
            Some(self.curr)
        }
    }
    
    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));

}

#[test]
fn consumer_of_iterator() {
    let v = vec![1, 2, 3];
    let it = v.iter();
    // sum will take ownership of the iterator
    let total: i32 = it.sum();
    assert_eq!(total, 6);
    assert_eq!(v, [1, 2, 3]);


    let v = vec![1, 2, 3];
    // iter takes immutable reference to the vector, collector retuns a vector of immutable references
    let v2: Vec<&i32> = v.iter().collect();
    // TODO why does it work?
    assert_eq!(v2, [&1, &2, &3]);

    let names = [("omar", 5), ("barra", 4)];
    let folks: HashMap<&str, i32> = names.into_iter().collect();
    assert_eq!(folks["omar"], 5);
    assert_eq!(folks["barra"], 4);
}

#[test]
fn iterator_adaptors() {
    let v = vec![1, 2, 3];
    let v2 = v.iter().map(|x| x + 1).collect::<Vec<i32>>();
    assert_eq!(v2, [2, 3, 4]);
}