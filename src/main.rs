// Find the last element of a list.
mod p01 {
    fn last<T>(vec: &Vec<T>) -> Option<&T> {
        vec.last()
    }

    #[test]
    fn test() {
        assert_eq!(last(&vec![1, 1, 2, 3, 5, 8]), Some(&8));
    }
}

// Find the last but one element of a list.
mod p02 {
    fn penultimate<T>(vec: &Vec<T>) -> Option<&T> {
        if vec.len() < 2 {
            return None;
        }

        Some(&vec[vec.len() - 2])
    }

    #[test]
    fn test() {
        assert_eq!(penultimate(&vec![1, 1, 2, 3, 5, 8]), Some(&5));
    }
}

// Find the Kth element of a list.
mod p03 {
    fn nth<T>(index: usize, vec: &Vec<T>) -> Option<&T> {
        if index >= vec.len() {
            return None;
        }

        Some(&vec[index])
    }

    #[test]
    fn test() {
        assert_eq!(nth(2, &vec![1, 1, 2, 3, 5, 8]), Some(&2));
    }
}

// Find the number of elements of a list.
mod p04 {
    fn length<T>(vec: &Vec<T>) -> usize {
        vec.len()
    }

    #[test]
    fn test() {
        assert_eq!(length(&vec![1, 1, 2, 3, 5, 8]), 6);
    }
}

// Reverse a list.
mod p05 {
    fn reverse<T>(vec: &Vec<T>) -> Vec<T>
        where T: Clone
    {
        let mut result = vec.clone();
        let len = vec.len();
        let middle = len / 2;

        for low in 0..middle {
            let high = len - low - 1;

            result[low] = vec[high].clone();
            result[high] = vec[low].clone();
        }

        result
    }

    #[test]
    fn test() {
        assert_eq!(reverse(&vec![1, 1, 2, 3, 5, 8]), [8, 5, 3, 2, 1, 1]);
    }
}

// Find out whether a list is a palindrome.
mod p06 {
    fn is_palindrome<T>(vec: &Vec<T>) -> bool
        where T: PartialEq
    {
        let len = vec.len();
        let middle = len / 2;

        for low in 0..middle {
            let high = len - low - 1;

            if vec[low] != vec[high] {
                return false;
            }
        }

        true
    }

    #[test]
    fn test() {
        assert!(is_palindrome(&vec![1, 2, 3, 2, 1]));
    }
}
