use cc;

// heavily inspired by 
// https://github.com/andersforsgren/robust2d/blob/master/build.rs

fn main() {
    cc::Build::new()
        .file("src/predicates.c")
        .flag("-w") // in order to hide maybe-uninitialized warnings
        .compile("predicates");
}