use cyn::{file::File, TokenStream};

#[test]
fn test() {
    let input = include_str!("basic.c");
    let ts = match TokenStream::from_str(input) {
        Ok(ts) => ts,
        Err(err) => panic!("{err}",),
    };
    println!("tokenstream:\n{ts}");
    println!("\n\n\n");
    let res = ts.parse::<File>();
    let test = match res {
        Ok(res) => res,
        Err(err) => panic!("{err}"),
    };
    let ts = cyn::to_tokens(&test);
    println!("parsed:\n{ts}");
    panic!("");
}
