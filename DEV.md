Rewrite
===

Current API is:
- create/generate struct/enum (from_source) (8 functions total)

Not very useful as a library. For this to be useful in proc macros, for example, it needs to be able to output quote-like stuff. And for it to be useful for _parts_ of files (instead of just entire files) it needs to accept more structured inputs.

The core of the library is this: data -> rust tokens

The data can be:
- A source file
- Source text
- A (generic) parsed structure

It's a pipeline!

1.  Inputs:
    - Source text
    - File -> source text
2.  Parsing:
    - json source -> GenericValue
    - yaml source -> GenericValue
    - etc...
3.  Core of the library:
    - GenericValue -> TokenStream
        - This could be map -> struct
        - Or map keys -> enum
4.  Post-processing:
    - TokenStream -> string
    - TokenStream -> file

- [x] Fix up (and refactor) Option types
- [x] Fix up and refactor Error types
- [w] Load fns? (let's skip this for now...)
    - load_from_file(path)
    - load
    - fetch(bool)
    - [ ] How best to implement these for enums etc.?
- [ ] Validation?
- [ ] Implement proc macro crate
    - define_structs
    - define_enums
    - define_enum_from_dir
    - define_structs_from_dir
- [ ] Update docs
- [ ] CI?

### source -> source
- create_struct
- create_struct_from_source
- generate_struct
- generate_struct_from_source
- create_enum
- create_enum_from_source
- generate_enum
- generate_enum_from_source

### data -> tokens
- struct_from_map
- enum_from_map_keys
- struct_from_map_values

### source -> data
- parse (format inferred/passed as argument)
- parse_json
- parse_yaml
- etc...

## New config options
- create extra struct from map values when making enum?

Development
===

## Eventually:
1.  Try to avoid generating the same structs twice
2.  Be smarter about types of collections:
    -   `[1, 2.0]` should resolve to `[f32]`
    -   `[10, null]` should resolve to `[Option<i64>]`
    -   `[[1.0], [1]]` should resolve to `[[f32]]`
    -   `[1.0, "hello"]` should throw a useful error
    -   `[{"a": 1}, {"b": 5.0}]` should resolve to `[struct {a: Option<i64>, b: Option<f64>}]`
3.  Possibly allow some kind of type hints in the config itself
    -   e.g. `{"a__f32": 1.0, "b__f64": 1.0}`
4.  And also possibly custom structs to be imported
    -   e.g. `{ "a__MyStruct": {...}}` => `use MyStruct; ...` etc.
5.  proc macros for generating things outside of build.rs
