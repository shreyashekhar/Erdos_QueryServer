#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_of<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let mut any_matches = false;
    for i in 0..b.len() {
        let mut matching = true;

        for j in 0..a.len() {
            let this_matches = 
                !( i + j >= b.len() ||
                   a[j] != b[i+j] );

                   
            matching = matching && this_matches;
            
            if !this_matches {
                break;
            }
        }

        any_matches = any_matches || matching;
    }

    any_matches
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a_of_b = sublist_of(_first_list, _second_list);
    let b_of_a = sublist_of(_second_list, _first_list);
    
    if a_of_b && b_of_a {
        Comparison::Equal
    } else if a_of_b {
        Comparison::Sublist
    } else if b_of_a {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn main() {
    assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
}