#![feature(box_into_raw_non_null)]

use pennant::Pennant;

#[test]
fn test_combining_two_one_element_pennants() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");

    x.combine(Box::new(y));

    assert_eq!(x.len(), 2);
    assert_eq!(x.degree(), 1);
    assert_eq!(x.fetch_element(), &"Mercury");
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert_eq!(middle.fetch_element(), &"Venus");
}

#[test]
fn test_combining_two_two_element_pennants() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    assert_eq!(x.len(), 4);
    assert_eq!(x.degree(), 2);
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());
    assert_eq!(x.fetch_element(), &"Mercury");

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.right.is_some());
    assert!(middle.middle.is_none());
    assert_eq!(middle.fetch_element(), &"Earth");

    let left;
    let right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert_eq!(left.fetch_element(), &"Venus");
    assert_eq!(right.fetch_element(), &"Mars");
}

#[test]
fn test_combining_two_four_element_pennants() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    let mut c = Pennant::new("Jupiter");
    let d = Pennant::new("Saturn");
    c.combine(Box::new(d));

    let mut e = Pennant::new("Uranus");
    let f = Pennant::new("Neptune");
    e.combine(Box::new(f));

    c.combine(Box::new(e));
    x.combine(Box::new(c));

    assert_eq!(x.len(), 8);
    assert_eq!(x.degree(), 3);
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert!(x.middle.is_some());

    let middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.right.is_some());
    assert!(middle.middle.is_none());
    assert_eq!(middle.fetch_element(), &"Jupiter"); 

    let left;
    let right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert!(left.left.is_some());
    assert!(left.right.is_some());
    assert!(left.middle.is_none());
    assert!(right.left.is_some());
    assert!(right.right.is_some());
    assert!(right.middle.is_none());
    assert_eq!(left.fetch_element(), &"Earth");
    assert_eq!(right.fetch_element(), &"Uranus");
}

#[test]
fn test_splitting_two_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");

    x.combine(Box::new(y));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.len(), 1);
    assert_eq!(x.degree(), 0);
    assert!(x.middle.is_none());
    assert_eq!(x.fetch_element(), &"Mercury");

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.len(), 1);
    assert_eq!(split_pennant.degree(), 0);
    assert!(split_pennant.middle.is_none());
    assert_eq!(split_pennant.fetch_element(), &"Venus");
}

#[test]
fn test_splitting_four_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.len(), 2);
    assert_eq!(x.degree(), 1);
    assert!(x.middle.is_some());
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert_eq!(x.fetch_element(), &"Mercury");

    let mut middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_none());
    assert!(middle.right.is_none());
    assert!(middle.middle.is_none());
    assert_eq!(middle.fetch_element(), &"Venus");

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.len(), 2);
    assert_eq!(split_pennant.degree(), 1);
    assert!(split_pennant.middle.is_some());
    assert!(split_pennant.left.is_none()); 
    assert!(split_pennant.right.is_none());
    assert_eq!(split_pennant.fetch_element(), &"Earth");

    unsafe {
        middle = Box::from_raw(split_pennant.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_none());
    assert!(middle.right.is_none());
    assert!(middle.middle.is_none()); 
    assert_eq!(middle.fetch_element(), &"Mars");
}

#[test]
fn test_splitting_eight_element_pennant() {
    let mut x = Pennant::new("Mercury");
    let y = Pennant::new("Venus");
    x.combine(Box::new(y));

    let mut a = Pennant::new("Earth");
    let b = Pennant::new("Mars");
    a.combine(Box::new(b));

    x.combine(Box::new(a));

    let mut c = Pennant::new("Jupiter");
    let d = Pennant::new("Saturn");
    c.combine(Box::new(d));

    let mut e = Pennant::new("Uranus");
    let f = Pennant::new("Neptune");
    e.combine(Box::new(f));

    c.combine(Box::new(e));
    x.combine(Box::new(c));

    let split = x.split();
    assert!(split.is_some());

    assert_eq!(x.len(), 4);
    assert_eq!(x.degree(), 2);
    assert!(x.middle.is_some());
    assert!(x.left.is_none());
    assert!(x.right.is_none());
    assert_eq!(x.fetch_element(), &"Mercury");

    let mut middle;
    unsafe {
        middle = Box::from_raw(x.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.right.is_some());
    assert_eq!(middle.fetch_element(), &"Earth");

    let mut left;
    let mut right;
    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert!(left.left.is_none());
    assert!(left.middle.is_none());
    assert!(left.right.is_none());
    assert_eq!(left.fetch_element(), &"Venus");
    assert_eq!(right.fetch_element(), &"Mars");

    let split_pennant = split.unwrap();

    assert_eq!(split_pennant.len(), 4);
    assert_eq!(split_pennant.degree(), 2);
    assert!(split_pennant.left.is_none());
    assert!(split_pennant.middle.is_some());
    assert!(split_pennant.right.is_none());
    assert_eq!(split_pennant.fetch_element(), &"Jupiter");

    unsafe {
        middle = Box::from_raw(split_pennant.middle.unwrap().as_ptr());
    }

    assert!(middle.left.is_some());
    assert!(middle.middle.is_none());
    assert!(middle.right.is_some());
    assert_eq!(middle.fetch_element(), &"Uranus");

    unsafe {
        left = Box::from_raw(middle.left.unwrap().as_ptr());
        right = Box::from_raw(middle.right.unwrap().as_ptr());
    }

    assert!(left.left.is_none());
    assert!(left.middle.is_none());
    assert!(left.right.is_none());
    assert_eq!(left.fetch_element(), &"Saturn");
    assert_eq!(right.fetch_element(), &"Neptune"); 
}
