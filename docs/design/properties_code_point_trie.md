# Unicode Properties and Code Point Tries in ICU4X

## Background

[Unicode Properties](https://unicode-org.github.io/icu/userguide/strings/properties.html) represent attributes of code points in the [Unicode specification](https://www.unicode.org/reports/tr44/#Properties). 

### Property Types

There are a half dozen [property type keys](https://www.unicode.org/reports/tr44/#About_Property_Table), which are used in places like the specification's [properties table](https://www.unicode.org/reports/tr44/#Property_List_Table): Catalog, Enumeration, Binary, String, Numeric, Miscellaneous. These property type keys more or less indicate the type of data that are associated to code points with that property.
 
Binary properties indicate whether a code point possesses a particular aspect, such as "does this represent whitespace?"

Enumerated properties indicate a two-level identification to a code point, specifically, when a particular aspect is associated with a known finite set of values (as the word "enumerated" implies). For the enumerated property that indicates that a code point is a "line break", one of the values in the associated set of property values might distinguish that a code point is a line-breaking hyphen, while another value might indicate that a code point is a line feed.

Catalog properties are much like enumerated properties, but their enumerated values usually get extended with successive updates to the Unicode Standard, while enumerate properties' values do not. Binary properties are officially considered a special case of enumerated properties, where the only 2 enumerated values are `true` and `false`. The value for binary properties is effectively a boolean value, and binary properties also go under the name "boolean properties" in the Unicode spec.

String Properties return a code point or sequence of code points. Numeric properties return numerical values, and Miscellaneous properties are properties that don't fit neatly in the other property type categories.

### Property Statuses

 The Unicode Specification [section on Properties](https://www.unicode.org/versions/Unicode13.0.0/ch03.pdf) (Section 3.5 in spec v13.0.0) also defines the property statuses: normative, informative, contributory, and provisional. Normative properties are properties that are defined by the Unicode spec, and compliant implementations of the Unicode spec must adhere to all of the guidelines when dealing with those properties. Informative properties give useful information about the properties representing expert experience, but implementations of the Unicode spec can decide to change / override such values and maintain compliance with the spec. Contributory properties exist as a collection of code points that are used in the derivation of other properties, and usually start with "Other_", ex: a hypothetical contributory property named "Other_Xyz" is probably used for computing property "Xyz", and where "Xyz" is directly user-facing and more meaningful as a concept. Provisional properties are unapproved or tentative.

 For returning useful properties to the user, we should exclude contributory and provisional properties from APIs.

### Notes About Properties

As [it is defined](https://www.unicode.org/reports/tr44/#General_Category_Values), the General_Category property of a code point "provides for the most general classification of that code point.
It is usually determined based on the primary characteristic of the assigned character for that code point.
For example, is the character a letter, a mark, a number, punctuation, or a symbol, and if so, of what type?" In the same section 5.7.1 of UAX #44, a table lists 30 single-category property values, as well as a few special multi-category property values (`LC`, `L`, `M`, `N`, `P`, `S`, `Z`, `C`) representing the union of multiple single-category values.
For example, the long name of the value `N` is `Number`, which is the union of `Nd` (`Decimal_Number`), `Nl` (`Letter_Number`), and `No` (`Other_Number`).
Each code point belongs to exactly one single-category property value for GeneralCategory, and it belongs to one or more of the multi-category values.
The multi-category values are handy for scenarios requiring "is the character a letter or number" style predicates. 

The [Lead_Canonical_Combining_Class](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/uchar_8h.html#ae40d616419e74ecc7c80a9febab03199a686db169e8d6dc82233ebdfdee777b5a) and [Trail_Canonical_Combining_Class](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/uchar_8h.html#ae40d616419e74ecc7c80a9febab03199a477985deea2b2c42f3af4c7174c60d6c) properties are ICU-specific properties that are useful for the implementation of algorithms. They are likely not generally useful for end-users.


### Use Cases

Before considering the design of APIs and efficient data structures, we first have to consider the shape of the data.
In enumerated properties, there are three dimensions: the enumerated property, the enumerated property value, and the code point.
In the binary properties case, we can reduce the scope to there being only two dimensions being associated—the binary property and the code point—if we maintain an implicit constraint that the property value must be `true`.

The use cases, or manner of data access, inform the designs of APIs and data structures.
For regular expression parsers (regex), we need to support a text description of a _set of code points_ sharing a property.
In this case, returning a [`CodePointInversionList`](https://unicode-org.github.io/icu4x/docs/icu/collections/codepointinvlist/struct.CodePointInversionList.html) (a set of code points, a.k.a. [`UnicodeSet`](https://unicode-org.github.io/icu/userguide/strings/unicodeset.html) in ICU) provides the most efficient usable data.
For binary properties, the property name is enough for input to determine the output.
For enumerated properties, the property name and a specific property value are required to uniquely determine a set of code points.
In these cases, all dimensions except the code point dimension are fixed (given as inputs).

In other cases, such as UAX \#29 segmentation algorithms, iteration through the code points of some input text is a typical implementation strategy.
During such iteration, the value of a code point property—usually, an enumerated property—can inform the algorithm in question.
In such cases, the code point value and enumerated property name dimensions must be fixed (provided as inputs), and the return value is the remaining dimension—the enumerated property value.
To support this use case, the [`CodePointTrie`](https://icu.unicode.org/design/struct/utrie) data structure is an optimal implementation for speed. 
Speed is an important consideration for such use cases since the number of code points in a text can vary in size as large as the input text, making the work heavily repeated.

The `CodePointTrie` data structure also serves Unicode data lookups to serve algorithms for Unicode normalization, collation, etc.

### Notes on Implementation

`CodePointInversionList` represents a set of Unicode code points.
The combination of 2 aspects—that the Unicode code point values fill the entire integer range from 0 to 0x10FFFF, and that a set has only 2 values—together allow for an [inversion list](https://en.wikipedia.org/wiki/Inversion_list) implementation that is optimally efficient.
An inversion list stores the boundaries of each range (contiguous stretch of code points) that are included in the set.
This makes the size of the inversion list range from O(1) to O(n) (and oftentimes O(1)) even when the cardinality of the values logically represented is O(n).
Checking for inclusion is just a matter of running binary search on the boundary values and checking if the corresponding inversion list index value is even or odd.

A `CodePointTrie` is an optimized implementation of what can be represented as an inversion map. An inversion map is similar to an inversion list, except that each range of code points is associated with a value. (By contrast, a range of code points in an inversion list is associated with an implied value of "true" for whether they are included in the set.) 

A `CodePointTrie` optimizes over a generic inversion map in different ways. One example is that for code point values in the BMP range (16 bits), the 16 bits can be split into the high-order 10 bits and the low-order 6 bits, where the low-order 6 bits can be used as an index into a table/index array. Also, `CodePointTrie`s can be created where the binary data values are serialized with 8-bit, 16-bit, and 32-bit encoding, to make lookups more efficient without making encoding conversions.

`CodePointTrie` code [in ICU4C](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/structUCPTrie.html) or [in ICU4J](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/) is implemented with a mutable builder, a method to convert the mutable builder to an immutable version, and code to read from the immutable version. The immutable version is stored in memory the same as it is serialized to persistent storage.

## Implementing `CodePointTrie` in ICU4X 

ICU4X benefits by reusing the optimized data structures produced by ICU.
In practice, this means exporting the code point trie binary data per property from ICU4C.
As a result, for the Rust enums / new types that represent Unicode properties in ICU4X, the discriminant integer values corresponding to each of the enum variants should match the corresponding enum integer values in ICU4C and ICU4J.
ICU4X code that can read the ICU code point trie binary data is a port of the original ICU code doing the same.
The code can traverse the binary (byte array) representation of the trie without any extra heap allocations.
Code in ICU4X ported from ICU includes getting the value for a code point input (`get()`), and getting the longest range of consecutive code points starting at the input code point whose trie values all match the provided input value (`get_range()`).

### Value Width

ICU `CodePointTrie`s are implemented logically as 2 arrays: an index array, and a data array.
The index array contains values needed for the iterative 1 or 3 lookups needed to traverse the trie prior to the last lookup.
Each element of the index array is 16 bits wide.

The penultimate lookup (the last lookup in the index array) returns a value that is the index into the data array.
The data array stores the values of the trie associated to each code point.
Data array elements are uniformly wide, with a width that is currently either 8 bits, 16 bits, or 32 bits.

### Trie Type

The features of code point tries in ICU are inherited in ICU4X.
One is that a trie can either be a "fast" type or a "small" type. 
The code point trie (ICU 3rd version CodePointTrie) is designed to return a value using either 2 or 4 array lookups.
For either trie type, there is a limit ("fast max limit") value that partitions the code space into code points that need 2 lookups (`[0x0000, fastMax)`) and code points that need 4 lookups (`[fastMax, 0x10FFFF]`).
For small type tries, the `fastMax` limit is `0x1000`, and for fast type tries, the `fastMax` limit is `0x10000`.
In effect, in either trie type, the first 1/16 of the first plane of the Unicode code space—`[0x0000, 0x0FFF]`—will only need 2 lookups.
Similarly, all code points in the 16 planes remaining planes after the first plane—`[0x10000, 0x10FFFF]`—will always have 4 array lookups.
Whether a code point trie is a small or fast type only affects the number of array lookups for code points in the range `[0x1000, 0x10000)`.
[This range](https://unicode.org/roadmaps/bmp/) contains various scripts, symbols, CJK characters, and other East Asian characters.

Fast type tries will be larger than small type tries. 
(Note: this is another instance of the classic computing tradeoff between time and speed.) 
Fast type tries are larger than small type tries because the minimum size of the index array is larger.
The index array will be larger because the range of code points only needing 2 array lookups will be larger, and a 2 array lookup is possible only when each such code point has a dedicated element in the index array.

### More Details

See previous discussion in [issue 131](https://github.com/unicode-org/icu4x/issues/131) regarding BytesTrie and `CodePointTrie`, and the [full meeting minutes](https://docs.google.com/document/d/1oaFovJiRbuBG-O9aq0h69gpGxMH0a_pHmMkO5YweS0E/edit#) of the related meeting for further details.

### Options

#### Option 1: Implement full-stack `CodePointTrie` (builder and reader) in Rust in ICU4X

This option entails writing new Rust code that supports all of the functionality of the `CodePointTrie`.

The advantage would be to have Rust-native code that can compile entirely within ICU4X and potentially be used elsewhere.

The disadvantages would be the large amount of time to implement all of the complicated logic (largely located in the builder), and the possibility of losing feature parity 'sync' if/when ICU makes further improvements to the `CodePointTrie` implementation.

#### Option 2: Implement the `CodePointTrie` reader in ICU4X Rust and import compiled data from ICU4C

This option entails writing Rust code that can interpret the binary serialization of the `CodePointTrie` and navigate it directly. It would also require creating an "offline" step (relative to ICU4X) in which [ICU4C binary data is exported](https://github.com/unicode-org/icu4x/issues/509) as a companion package in the data downloads for new each ICU release.

The advantages would be having code in ICU4X that shares the same precomputed optimized code point trie data, and is therefore guaranteed to stay in sync with future improvements in `CodePointTrie` implementation in order to use new versions of data. Only the `CodePointTrie` reader code needs to be ported to read the serialized data, which is much smaller than the C++ code in ICU4C for the builder that builds the structure that gets serialized.

The disadvantages would be similar to what are applicable for other external sources of data in ICU4X that go through a data provider (an extra data dependency that requires an offline step for downloading/exporting during build and/or installation time).

### Decision

The preferred approach is option 2, to implement a reader in Rust of the ICU binary format of `CodePointTrie`. The disadvantages are fairly minimal and not different from those of other external sources of data for ICU4X's data provider. However, the upsides of optimal performance, less code to write, and better integration with ICU are each considered to be significant. 

Choosing this option does not rule out a full implementation of the code point trie builder, but the cost benefit analysis in the meeting notes would need to be revisited for that. For the current constraints, using ICU4C data is a reasonable approach.

Because the binary format of the `CodePointTrie` can vary depending on the endian-ness of the target architecture, it will be easiest to export the bytes as plain text data literals (ex: in TOML format). For more details on the specific representation that will be used within the offline step in ICU4C to dump the code point trie information, see [this design doc](https://docs.google.com/document/d/1JkrL4pv477dIVnfilwlAEwqF9vstb_Uo_5XeUz2WXM8/edit#heading=h.9sqixr5cv2wn) for that topic.

## Building a `CodePointTrie`

Building a `CodePointTrie` is expensive because several optimizations are applied during build time in order to reduce the size with little to no effect on the runtime.
In ICU, when trie data is built for Unicode properties, it is done in a compile-time step and stored statically, which therefore does not affect runtime performance.
One example of an optimization is called compaction, in which subarrays which have identical contents can be collapsed and treated as being identical without affecting the trie lookup algorithm's result.
However, the algorithm to detect redundant blocks is inherently a pairwise comparison, and thus O(n^2).
The code to handle this and other optimizations is non-trivially complex.

Therefore, ICU4X implements the reader code for a trie, but it does not attempt to similarly port the ICU code for building a trie.
However, as a convenience, some code may exist in ICU4X which uses a wrapper over the WASM binary to which the ICU4C trie builder code is compiled.
