# candy_cane

This crate attempts to implement a fast way to iterate over a buffer mutably from many
threads simultaneously. 

Unfortunately, it does not work at the speeds I would've liked, only barely beating
the naive implementation in the best case scenario.

Load `./results.csv` into some kind of spreadsheet editor.

The names of the results are in the following structure:

`cc_no_threads_iter_10_5000`: `candy_cane`, no threads,
`ref` iter, 10 chunks, 5000 elements.

Hypothetically, the best case scenario would've been large amounts of data
with about 10 or so chunks with a mutable streaming iterator.

Unfortunately, this only beat the naive `RwLock<Vec<T>>` implementation by 1ms
on average per iteration.
