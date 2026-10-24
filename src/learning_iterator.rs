use std::cmp::Ordering;
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

struct Flaky(bool);

impl Iterator for Flaky {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true;
            None
        }
    }
}

#[test]
fn test_flaky() {
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
}

#[test]
fn test_reverse() {
    let bee_parts = ["head", "thorax", "abdomen"];

    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next_back(), Some(&"breakfast"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_inspect() {
    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {c:?}"))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("   after: {c:?}"))
        .collect();
    assert_eq!(upper_case, "GROSSE");
}

#[test]
fn test_chunks_mut() {
    let mut data = [1, 2, 3, 4, 5, 6, 7];
    for chunk in data.chunks_mut(3) {
        // Double every element in this chunk
        for x in chunk.iter_mut() {
            *x *= 2
        }
        chunk[0] += 100;
    }

    println!("{:?}", data);

    // RGBA example
    let mut pixels = vec![255u8, 0, 0, 255, 0, 255, 0, 255];
    for px in pixels.chunks_mut(4) {
        px[3] = 128; // set alpha to half
    }
}

#[test]
fn test_cloned() {
    let a = ['1', '2', '3', '∞'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));
    assert_eq!(a.iter().copied().next(), Some('1'));
}

#[test]
fn test_cycle() {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));

    // second cycle test
    let fizzes = iter::repeat_n("", 2).chain(iter::once("fizz")).cycle();
    let buzzes = iter::repeat_n("", 4).chain(iter::once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100)
        .zip(fizzes_buzzes)
        .map(|tuple| match tuple {
            (i, ("", "")) => i.to_string(),
            (_, (fizz, buzz)) => format!("{fizz}{buzz}"),
        });

    for line in fizz_buzz {
        println!("{line}");
    }
}

const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
fn count_vowels(s: &str) -> usize {
    s.to_uppercase()
        .chars()
        .filter(|c| VOWELS.contains(c))
        .count()
}

#[test]
fn test_count_vowels() {
    assert_eq!(count_vowels("A"), 1);
    assert_eq!(count_vowels("Bilal Muhammad"), 5);
}

fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[test]
fn test_both() {
    assert_eq!(triangle(20), 210);
    assert_eq!(factorial(20), 2432902008176640000);
}

#[test]
fn test_max_min() {
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
}


// Compare two f64 values. Panic if given a NaN
fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

#[test]
fn test_max_min_by() {
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
}

#[test]
#[should_panic(expected="called `Option::unwrap()` on a `None` value")]
fn test_max_panic() {
    let numbers = [1.0, 4.0, f64::NAN, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
}

#[test]
fn test_populations() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(
        populations.iter().max_by_key(|&(_name, pop) | pop),
        Some((&"Portland", &583_776)),
    );

    assert_eq!(
        populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Greenhorn", &2)),
    )
}

#[test]
fn test_comparing_sequences() {
    let packed = "Helen of Troy";
    let spaced = "Helen     of     Troy";
    let obscure = "Helen of Sandusky";   // nice person, just not famous

    assert_ne!(packed, spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

    // This is true because ' ' < 'o'
    assert!(spaced < obscure);

    // This is true because "Troy" > "Sandusky"
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}

#[test]
fn test_any_all() {
    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));
}

#[test]
fn test_position() {
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);

    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));
}

#[test]
fn test_fold() {
    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);            // count
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);     // sum
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200); // product

    // max
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10);


    let a = ["Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs"];
    // See also: the `join` method on slices, which won't
    // give you that extra space at the end
    let pangram = a.iter().fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");

    let weird_pangram = a.iter().rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");
}