mod notes;

pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn foo_str() {
    strlen("hello world"); // &'static str
    strlen(String::from("hei verden")); // String: As
}

//////////////////////////
pub trait Hei {
    fn hei(&self);
}
impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    "J".hei();
}

pub fn bar_low(h: impl Hei) {
    h.hei();
}

// (Static) Dispatch, geenerating actual code when concretes are figured in code
// from generic declarations. You get multiple copies of same code
// for each concrete type
pub fn bar_str(h: &str) {
    h.hei();
}

///////////////////////////
/// IMP: when you use trait objects, you only retain knowledge to implement that particular trait
/// and nothing else
pub fn strlen_dyn2(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

fn call() {
    let y = "hahaha a";

    strlen_dyn2(&y);
}

///////////////////////////
// pub fn foo_adv() {
//     bar(&["J", "Jon"]);
//     bar(&[String::from("J"), String::from("Jon")]);
//     bar(&["J", String::from("Jon")]);
// }

pub fn strlen_dyn(s: Box<&str>) -> usize {
    s.as_ref().len()
}

// fn main() {
//     let x = strlen_dyn(Box::new("this is "));
//     println!("value of x: {}", x);
// }
