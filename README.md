# btree-rs

Fundementally a port of
https://opendatastructures.org/ods-python/14_2_B_Trees.html to Rust.


```console
$ rustc btree.rs
$ ./btree
Inserting 10 = bif.
Inserting 9 = abe.
Inserting 8 = bar.
Inserting 6 = blub.
Inserting 3 = abc.
Inserting 1 = foo.

DONE INSERTING. VALIDATING.

  1 = "foo"
  3 = "abc"
6 = "blub"
  8 = "bar"
9 = "abe"
  10 = "bif"

VALIDATED. NOW FOR NOT FOUND.

0: Not found
2: Not found
7: Not found
100: Not found
```
