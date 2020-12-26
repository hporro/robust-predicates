# robust-predicates

Robust geometric predicates from Jonathan Schewchuk's library https://www.cs.cmu.edu/~quake/robust.html.

This crate provides the same API for the 4 geometric adaptive predicates as the original library. It takes the **predicates.c** file, compiles and link it to export those predicates as they are, operating on simple **[f64;2]** and **[f64;3]** arrays.

This crate might be seen as a simpler version of https://crates.io/crates/robust2d. This library uses the **cc** crate instead of **gcc** to compile **predicates.c** (used in https://crates.io/crates/robust2d), and doesn't wrap **[f64;2]** on point structures, or the functions with other names.

## Functions defined

``` rust
fn exactinit();
fn orient2d(pa:&[f64;2], pb:&[f64;2], pc:&[f64;2]) -> f64;
fn orient3d(pa:&[f64;3], pb:&[f64;3], pc:&[f64;3], pd:&[f64;3]) -> f64;
fn incircle(pa:&[f64;2], pb:&[f64;2], pc:&[f64;2], pd:&[f64;2]) -> f64;
fn insphere(pa:&[f64;3], pb:&[f64;3], pc:&[f64;3], pd:&[f64;3], pe:&[f64;3]) -> f64;
``` 


## Usage

There's only one thing that  has to be take in consideration when using this predicates. As in Schewchuk's library, this crate may work without the initialization of global variables, because is only needed for the adaptive filtering step (the hard cases). That's why is advised, but not mandated, to use the **exactinit** function before using the predicates.

## Example

``` rust
// main.rs

use robust_predicates::*;

fn main() {
    exactinit();
    let points: [[f64;2];4] = [[0.0,0.0],[1.0,0.0],[0.0,1.0],[0.5,0.5]];
    let c = incircle(&points[0],&points[1],&points[2],&points[3]);
    let d = orient2d(&points[1],&points[0],&points[2]);
    println!("{}",c);
    println!("{}",d);
}
```

You can use *insphere* and *orient3d* in the same way, but with 1 more parameter and *[f64;3]* arrays.

## Acknowledgments

The development of this crate is heavily influenced by https://crates.io/crates/robust2d. That crate is fantastic if you are looking for a wrapper for the 2d predicates.

You also can get the original **predicates.c** file in https://www.cs.cmu.edu/~quake/robust.html.