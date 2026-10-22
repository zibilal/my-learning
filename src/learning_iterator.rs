use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::iter;
use std::iter::Peekable;
use std::path::Path;
use std::str::FromStr;

#[test]
fn test_a_first() {
    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("element: {}", element)
    }

    // under hood calls `IntoIterator` and `Iterator` methods
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("element: {}", element);
    }
}

#[test]
fn test_an_iter() {
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);
}

#[test]
fn test_an_iter2() {
    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();

    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));




    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);
}


fn dump<T, U> (t: T)
where
    T: IntoIterator<Item=U>,
    U: Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

#[test]
fn test_dump() {
    let v = vec![4, 20, 12, 8, 6];
    dump(v);
}

#[test]
fn test_random() {
    let lengths: Vec<f64> = iter::from_fn(|| {
        Some((rand::random::<f64>() - rand::random::<f64>()).abs())
    })
        .take(1000)
        .collect();
    dump(lengths);
}

fn collatz_iterations(n: u64) -> u64 {
    let next = |n| match n {
        1 => None,
        n if n % 2 == 0 => Some(n / 2),
        n=> Some(3 * n + 1),
    };

    iter::successors(next(n), |&n| next(n)).count() as u64
}

#[test]
fn test_collatz_iterations() {
    println!("{}", collatz_iterations(8));
}

#[test]
fn test_flat_map() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{city}");
    }
}

#[test]
fn test_filter_map() {
    let text = "1\nfrond .25 289\n3.1415  estuary\n";
    for number in text.split_whitespace().filter_map(|w| f64::from_str(w).ok()) {
        println!("{number:4.2}");
    }
}

#[test]
fn test_flatten() {

    // A table mapping cities to their praks: each value is a vector
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mory Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Build a vector of all parks. `values` gives us an iterator producing
    // vectors, and then `flatten` produces each vector's elements in turn.
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(all_parks,
        vec![
            "Tadasu-no-Mory Forest", "Maruyama Koen", "Percy Warner Park", "Dragon Park", "Mt. Tabor Park", "Forest Park",
        ]
    );

    assert_eq!(
        vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
        vec!["day", "one"],
    );
}

#[test]
fn test_take_while() {
    let message = "\
To: jimb\r\n\
From: superego <editor@oreilly.com>\r\n\
\r\n\
Did you get any writing done today?\r\n\
When will you stop wasting time plotting fractals?\r\n";

    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{header}");
    }
}

#[test]
fn test_skip_while() {
    let message = "\
To: jimb\r\n\
From: superego <editor@oreilly.com>\r\n\
\r\n\
Did you get any writing done today?\r\n\
When will you stop wasting time plotting fractals?\r\n";

    for body in message.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{body}");
    }
}

#[test]
fn test_diff_take_skip() {
    let v = [1, 2, 3, 10, 4, 5];
    let a: Vec<_> = v.iter().take(3).collect();   // [1, 2, 3]
    let b: Vec<_> = v.iter().take_while(|&&x| x < 5).collect();
    // [1, 2, 3] (stops at 10, so 4 and 5 are never yielded)
    println!("a: {a:?}  b: {b:?}");

    let c: Vec<_> = v.iter().skip(3).collect();   // [10, 4, 5]
    let d: Vec<_> = v.iter().skip_while(|&&x| x < 5).collect();
    // [10, 4, 5]  (4 and 5 yielded even though they're < 5,
    //             because skipping already ended at 10)
    println!("c: {c:?}  d: {d:?}");
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n,
        }
        tokens.next();
    }
}

#[test]
fn test_parse_number() {
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    // Look, `parse_number` didn't consume the comma! So we will.
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);
}

#[test]
fn test_zip()  {
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    let endings = ["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = iter::repeat("going")
        .zip(endings)
        .collect();
    assert_eq!(
        rhyme,
        vec![
            ("going", "once"),
            ("going", "twice"),
            ("going", "chicken soup with rice"),
        ],
    );
}