# a_or_an TODO

- add negative tests

* [ ] 0 length string

* [ ] add numbers

* [ ] add other

* [ ] test more cases:

```
console.log(a('UGLY SWEATER', { caseInsensitive: true })); // 'an UGLY SWEATER'
console.log(a('2')); // 'a 2'
console.log(a('8')); // 'an 8'
console.log(a('1892')); // 'a 1892' -> read "a one thousand eight hundred ninety-two"
console.log(a('1892', { numbers: 'colloquial' })); // 'an 1892' -> read "an eighteen ninety-two"
```

* [ ] add non-USA support

* [ ] add docs (auto with rust?)

* [ ] pub as crate
