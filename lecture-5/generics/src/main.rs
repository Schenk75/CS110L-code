use std::fmt;

pub struct MatchingPair<T> {
    first: T,
    second: T,
}

pub enum MyOption<T> {
    Sumthin(T), Nuthin
}

// impl fmt::Display for MyOption<u32> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MyOption::Sumthin(num) => write!(f, "Sumthin({})", num),
//             MyOption::Nuthin => write!(f, "Nuthin :("),
//         }
//     }
// }

impl<T> MatchingPair<T> {
    pub fn new(first: T, second: T) -> Self {
        MatchingPair {first, second}
    }
}

// impl fmt::Display for MatchingPair<char> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.first, self.second)
//     }
// }

impl<T: fmt::Display> fmt::Display for MyOption<T> { // more general!
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           MyOption::Sumthin(num) => write!(f, "Sumthin({})", num),
           MyOption::Nuthin => write!(f, "Nuthin :("),
       }
       
   }
}

impl<T> fmt::Display for MatchingPair<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "({}, {})", self.first, self.second)
   }
}

impl<T: Clone> Clone for MatchingPair<T> {
    fn clone(&self) -> Self {
        MatchingPair::new(self.first.clone(), self.second.clone())
    }
}

fn main() {
    let ps_in_a_pod: MatchingPair<char> = MatchingPair::new('p', 'P');
    println!("two ps in a pod: {}", ps_in_a_pod);

    let my_some_five: MyOption<u32> = MyOption::Sumthin(5);
    let my_nuthin: MyOption<u32> = MyOption::Nuthin;
    println!("my_some_five: {}", my_some_five);
    println!("my_nuthin: {}", my_nuthin);

    let ps_clone = ps_in_a_pod.clone();
    println!("cloned ps in a pod: {}", ps_clone);
}
