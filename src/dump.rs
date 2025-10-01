use crate::outcome::Outcome;

pub type DumpOutcomeFn = fn(Vec<Outcome>);

fn text_dump(outcomes: Vec<Outcome>) {
    for outcome in outcomes.iter() {
        println!("{outcome}")
    }
}

fn json_dump(outcomes: Vec<Outcome>) {
    let json_pretty = serde_json::to_string_pretty(&outcomes).unwrap();
    println!("{json_pretty}");
}

/*
 * Could use an enum and bind the values to function.
 * Maybe if more formats are to be handled,
 * for now it just feels overkill.
*/
pub fn get_fn(json: bool) -> DumpOutcomeFn {
    if json {
        return json_dump;
    }
    text_dump
}

#[cfg(test)]
mod test {
    use crate::dump::get_fn;
    use crate::dump::{json_dump, text_dump};

    #[test]
    fn test_get_fn_text() {
        let dump_fn = get_fn(false);
        assert!(std::ptr::addr_eq(
            dump_fn as *const (),
            text_dump as *const ()
        ));
    }

    #[test]
    fn test_get_fn_json() {
        let dump_fn = get_fn(true);
        assert!(std::ptr::addr_eq(
            dump_fn as *const (),
            json_dump as *const ()
        ));
    }
}

#[cfg(test)]
mod test_dummy {
    use crate::dump::{json_dump, text_dump};
    use crate::outcome::Outcome;

    #[test]
    fn test_coverage_target() {
        let res = Outcome::new("test".to_string()).set("new".to_string());
        text_dump(vec![res.clone()]);
        json_dump(vec![res]);
    }
}
