use bag::Bag;

#[test]
fn test_inserting_into_empty_bag() {
    let mut bag = Bag::with_degree(2);
    bag.insert("Mercury");

    assert_eq!(bag.len(), 1);
    assert!(bag.spine[0].is_some());
}

#[test]
fn test_inserting_into_nonempty_bag() {
    let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");

    assert_eq!(bag.len(), 2);
    assert!(bag.spine[0].is_none());
    assert!(bag.spine[1].is_some());

    bag.insert("Earth");

    assert_eq!(bag.len(), 3);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[1].is_some());

    bag.insert("Mars");

    assert_eq!(bag.len(), 4);
    assert!(bag.spine[0].is_none());
    assert!(bag.spine[1].is_none());
    assert!(bag.spine[2].is_some());

    bag.insert("Jupiter");

    assert_eq!(bag.len(), 5);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[1].is_none());
    assert!(bag.spine[2].is_some());
}

#[test]
fn test_union_with_empty_bags() {
   let mut bag: Bag<i32> = Bag::new();
   let empty = Bag::new();
   bag.union(empty);

   assert_eq!(bag.len(), 0);
   assert!(bag.spine.iter().all(|x| x.is_none()));
}

#[test]
fn test_union_with_one_nonempty_bag_and_one_empty_bag() {
    let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");

    let empty = Bag::with_degree(2);

    bag.union(empty);

    assert_eq!(bag.len(), 2);
    assert!(bag.spine[1].is_some());
}

#[test]
fn test_union_with_nonempty_bags() {
    let mut bag = Bag::new();
    bag.insert("Mercury");
    bag.insert("Venus");

    let mut other = Bag::new();
    other.insert("Earth");
    other.insert("Mars");
    other.insert("Jupiter");
    other.insert("Saturn");
    other.insert("Uranus");
    other.insert("Neptune");
    other.insert("Pluto");

    bag.union(other);

    assert_eq!(bag.len(), 9);
    assert!(bag.spine[0].is_some());
    assert!(bag.spine[3].is_some());
}

#[test]
fn test_splitting_empty_bag() {
    let mut bag: Bag<i32> = Bag::with_degree(2);
    let other_bag = bag.split();

    assert_eq!(other_bag.len(), 0);
}

#[test]
fn test_splitting_bag_with_one_element() {
    let mut bag = Bag::with_degree(2);
    bag.insert("Pluto");

    let other_bag = bag.split();

    assert_eq!(bag.len(), 0);
    assert_eq!(other_bag.len(), 1);
}

#[test]
fn test_splitting_bag_with_even_elements() {
   let mut bag = Bag::with_degree(3);
    bag.insert("Mercury");
    bag.insert("Venus");
    bag.insert("Earth");
    bag.insert("Mars");

    let other_bag = bag.split();

    assert_eq!(bag.len(), 2);
    assert_eq!(other_bag.len(), 2);
}

#[test]
fn test_splitting_bag_with_odd_elements() {
    let mut bag = Bag::new();
    bag.insert("Mercury");
    bag.insert("Venus");
    bag.insert("Earth");
    bag.insert("Mars");
    bag.insert("Jupiter");
    bag.insert("Saturn");
    bag.insert("Uranus");
    bag.insert("Neptune");
    bag.insert("Pluto");

    let other_bag = bag.split();

    assert_eq!(bag.len(), 5);
    assert_eq!(other_bag.len(), 4);
}
