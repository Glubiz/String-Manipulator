pub trait Trim {
    fn trim(&self) -> Self;
}

pub trait ConvertCase {
    fn to_camel_case(&self) -> Self;
    fn to_snake_case(&self) -> Self;
    fn to_kebab_case(&self) -> Self;
    fn to_upper(&self) -> Self;
    fn to_lower(&self) -> Self;
}

pub trait Padding {
    fn pad_left(&self, n: usize, c: char) -> Self;
    fn pad_right(&self, n: usize, c: char) -> Self;
    fn pad(&self, n: usize, c: char) -> Self;
}

pub trait Split {
    fn split(&self, c: char) -> Vec<String>;
}

pub trait Join {
    fn join(&self, d: char) -> String;
}

pub trait Search {
    fn find_first_of(&self, s: &str) -> Option<usize>;
    fn find_last_of(&self, s: &str) -> Option<usize>;
    fn find_first_not_of(&self, s: &str) -> Option<usize>;
    fn find_last_not_of(&self, s: &str) -> Option<usize>;
    fn matches_count(&self, s: &str) -> usize;
    fn matches_indices(&self, s: &str) -> Vec<usize>;
    fn matches_indices_count(&self, s: &str) -> usize;
    fn contains_any(&self, s: &str) -> bool;
    fn contains_all(&self, s: &str) -> bool;
    fn contains_none(&self, s: &str) -> bool;
}

pub trait Replace {}

pub trait Substring {}

pub trait Formatting {}

pub trait Encoding {}

pub trait Decoding {}

pub trait Hashing {}

pub trait Compression {}

pub trait Encryption {}

pub trait Random {}

pub trait Validation {}

pub trait Conversion {}

pub trait Sorting {}

pub trait Math {}

pub trait Date {}

pub trait Time {}

pub trait DateTime {}

pub trait File {}

pub trait Directory {}

pub trait Path {}

pub trait Network {}

pub trait Web {}

pub trait Database {}

pub trait Email {}

pub trait Image {}

pub trait Audio {}

pub trait Video {}

pub trait Terminal {}

pub trait System {}

pub trait Process {}

pub trait Thread {}

pub trait Memory {}

pub trait Logging {}

pub trait Debugging {}

pub trait Testing {}

pub trait Benchmarking {}

pub trait Profiling {}

pub trait ErrorHandling {}

pub trait Serialization {}

pub trait Deserialization {}

pub trait Parsing {}

pub trait Printing {}
