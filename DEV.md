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

- [x] define_structs_from_file_contents
- [ ] in define_enum_from_filenames, optionally produce values too
    - [ ] but also don't even parse files if not
- [ ] test the entire gen module (via test crate)
- [ ] replace the guts of the public API to use gen
- [ ] delete old generation module (and rename gen to codegen)
- ...
- [ ] Delete unused pub(crate) functions in parsing modules
- [ ] Fix names (parse_whatever2 -> parse_whatever)
- [ ] Start designing public API:
    - generate or create (output type)
    - struct or enum
    - from_file (default) or from_source or from_files/from_file_names
    - 2 * 2 * 3 = 12 top-level functions, that's no too bad
- [ ] Implement proc macro crate
    - define_structs
    - define_enums
    - define_enum_from_dir
    - define_structs_from_dir
- [ ] Fix up (and refactor) Option types
- [ ] Fix up and refactor Error types
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

## Soon:

- [x] Remove `_with_format` variants of functions in lieu of an `Option<Format>` field in the Options structs.
- [x] Document enum generation
- [x] Document enum generation at the top level, and in README
- [x] Document that leaving out the const name will not generate a const
- [x] Add options to implement Display and FromStr
- [x] Add tests for enum generation
- [x] Go over docs and README
- [x] Release
- [/] Add feature-gated generation of an enum from filenames in a directory
    - Should be (option for) repr(<int-type>)
    - Should be guaranteed to be alphabetical
    - And ignore subdirectories
    - Should have a const method to return the original filename
    - Should ignore the full parent path - since that's known at generation time
- [ ] Release again
- [ ] Address TODOs

## Maybe:
- [ ] Consider replacing struct generation with use of the `quote!` crate


## Eventually:
1.  Try to avoid generating the same structs twice
2.  Move to a more elegant method of code generation (quote! macro?)
3.  Move to a more elegant method of comparing types (not strings!!)
4.  Be smarter about types of collections:
    -   `[1, 2.0]` should resolve to `[f32]`
    -   `[10, null]` should resolve to `[Option<i64>]`
    -   `[[1.0], [1]]` should resolve to `[[f32]]`
    -   `[1.0, "hello"]` should throw a useful error
    -   `[{"a": 1}, {"b": 5.0}]` should resolve to `[struct {a: Option<i64>, b: Option<f64>}]`
5.  Possibly allow some kind of type hints in the config itself
    -   e.g. `{"a__f32": 1.0, "b__f64": 1.0}`
6.  And also possibly custom structs to be imported
    -   e.g. `{ "a__MyStruct": {...}}` => `use MyStruct; ...` etc.
