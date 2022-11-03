use std::collections::BTreeMap;

fn duplicate_encode(word:&str)->String {
    let mut char_dups: BTreeMap<char, char> = BTreeMap::new();
    for c in word.chars() {
        char_dups.entry(c.to_ascii_lowercase()).and_modify(|curr| *curr = ')').or_insert('(');
    }
    word.chars().map(|c| char_dups.get(&c.to_ascii_lowercase()).unwrap()).collect()
}

#[test]
fn run_tests() {
  assert_eq!(duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode("(( @"),"))((");
}