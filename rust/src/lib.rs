#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(vec_into_raw_parts)]
#![feature(trivial_bounds)]

mod bindings;
mod scandir_;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn gib_string() -> &'static str  {
        return "Löwe und Léopard";
    }
    #[test]
    fn no_x() {
        assert_eq!(gib_string().find('x'), None);
    }

    #[test]
    fn n_6() {
        assert_eq!(gib_string().find('n'), Some(7));
    }

    #[test]
    fn l() {
        assert_eq!(gib_string().find('L'), Some(0));
    }

    #[test]
    fn oe() {
        assert_eq!(gib_string().find('ö'), Some(1));
    }

    #[test]
    fn w() {
        assert_eq!(gib_string().find('w'), Some(3));
    }

    #[test]
    fn e() {
        assert_eq!(gib_string().find('e'), Some(4));
    }

    #[test]
    fn ws() {
        assert_eq!(gib_string().find(' '), Some(5));
    }

    #[test]
    fn u() {
        assert_eq!(gib_string().find('u'), Some(6));
    }
}
