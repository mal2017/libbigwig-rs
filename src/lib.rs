#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));




/*
 TROUBLESHOOTING
 1. Correct version of llvm/clang
 2. correct name for system libbigwig



*/

pub struct BigWigFile {
    chroms: chromList_t,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
