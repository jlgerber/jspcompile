use nom::{
    IResult,
    sequence::{delimited},
    bytes::complete::{tag},
    InputTakeAtPosition,
    error::ErrorKind,
    //character::complete::{char,},
};


#[inline]
fn is_ident_char(c: char) -> bool {
        // uppercase letters
        (c > '\x40' && c < '\x5B') || 
        // numbers
        (c > '\x2F'&& c < '\x3A')|| 
        // lowercase letters
        (c > '\x60'&& c < '\x7B') || 
        ['_', '-' ].contains(&c)
}

#[cfg(test)]
mod id_char {
    use super::*;
    #[test]
    fn is_ident_char_test() {
        for x in vec![
            'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','z',
            'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','Z',
            '1','2','3','4','5','6','7','8','9','0'] {
            assert!(
                is_ident_char(x)
            );
        }
    }

     #[test]
    fn is_not_id_char_test() {
        for x in vec![ '[',']','<','>','?', '^', '#', '@', '$', '%', '&', ',', '"', '/', /*'\\',*/'`', '+',')','(' ] {
            //println!("testing {}", x);
            assert!(
                !is_ident_char(x)
            );
        }
    }
}
#[inline]
fn is_regex_char(c: char) -> bool {
        // everything except for space
        !['"', '\''].contains(&c) && c > '\x20'  && c < '\x7F'  
}


#[cfg(test)]
mod regex_char {
    use super::*;
    #[test]
    fn is_regex_char_test() {
        for x in vec![ '!', '#', '$', '%', '^', '&', '*', '(', ')','_','-','+','=', ';',':',',','<','.','>','?','/',
            'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','z',
            'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','Z',
            '1','2','3','4','5','6','7','8','9','0'] {
            assert!(
                is_regex_char(x)
            );
        }
    }

     #[test]
    fn is_not_id_char_test() {
        for x in vec![ '"',  ] {
            //println!("testing {}", x);
            assert!(
                !is_ident_char(x)
            );
        }
    }
}

#[inline]
pub fn variable(input: &str) -> IResult<&str, &str> {
  input.split_at_position1_complete(|item| !is_ident_char(char::from(item )), ErrorKind::Alpha)
}

#[cfg(test)]
mod variable {
    use super::*;
    use nom::combinator::all_consuming;

    #[test]
    fn is_var_test() {
        let foo = variable("tHisIs_atest1-");
        assert!(foo.is_ok());
    }

    #[test]
    fn is_var_bad_test() {
        let foo = all_consuming(variable)("thisisatest!");
        assert!(!foo.is_ok());
    }

    #[test]
    fn is_var_bad2_test() {
        let foo = all_consuming(variable)("thisis atest");
        assert!(!foo.is_ok());
    }

}


#[inline]
pub fn regex_str(input: &str) -> IResult<&str, &str> {
  input.split_at_position1_complete(|item| !is_regex_char(char::from(item)), ErrorKind::Alpha)
}

#[cfg(test)]
mod regex_str {
    use super::*;
    use nom::combinator::all_consuming;

    #[test]
    fn is_regex_str_test() {
        let foo = regex_str("([a-zZ-Z0-9]+)*(foo|bar)$#{1,3}");
        assert!(foo.is_ok());
    }

    #[test]
    fn is_regex_str_bad_test() {
        let foo = all_consuming(regex_str)("thisi satest!");
        assert!(!foo.is_ok());
    }
}

pub fn quoted_regex_str(input: &str) -> IResult<&str, &str> {
    delimited(tag(r#"""#), regex_str, tag(r#"""#))(input)
}

#[cfg(test)]
mod quoted_regex_str {
    use super::*;
    use nom::combinator::all_consuming;

    #[test]
    fn is_quoted_regex_str_test() {
        let foo = quoted_regex_str(r#""([a-zZ-Z0-9]+)""#);
        println!("{:?}", foo);
        assert!(foo.is_ok());
    }

    #[test]
    fn is_quoted_regex_str_bad_test() {
        let foo = all_consuming(quoted_regex_str)(r#""(thisis'atest)""#);
        assert!(!foo.is_ok());
    }

    #[test]
    fn is_var_bad2_test() {
        let foo = all_consuming(variable)("thisis atest");
        assert!(!foo.is_ok());
    }

}
