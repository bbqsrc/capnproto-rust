## v0.12.1
- Fix buggy Iterator::nth() implementation for ListIter.

## v0.12.0
- Add "unaligned" feature flag to allow use of unaligned memory.
- Remove `Word::bytes_to_words()` and `Word::bytes_to_words_mut()`.
- Change a bunch of interfaces to use `u8` instead of `Word`.
- Remove `read_message_from_words()` in favor of `read_message_from_flat_slice()`.
- Add new `serialize::SegmentLengthsBuilder` API.
- Bump minimum required rustc version to 1.40.0.

## v0.11.2
- Deprecate `read_message_from_words()` in favor of `read_message_from_flat_slice()`.
- Remove incorrect doc comments on `bytes_to_words()`. (Misaligned access is never okay.)

## v0.11.1
- Remove internal capnp::map::Map and use async/await instead.

## v0.11.0
- Remove the "futures" feature and the optional futures 0.1 dependency, in favor of std::future::Future.
- Bump minimum support rustc version to 1.39.0.

## v0.10.3
- Add serialize::read_message_from_flat_slice().

## v0.10.2
- Allow buffer passed to read_message_from_words() to be larger than the actual message.

## v0.10.1
- Remove dependency on byteorder crate, in favor of from_le_bytes() and to_le_bytes().

## v0.10.0
- Simplify handling of pointer defaults by adding default parameter to FromPointerReader.
- Add IntoInternalStructReader as a bound on OwnedStruct::Reader.
- Remove capnp_word!() macro in favor of const fn ::capnp::word().
- Remove deprecated items.
- Update to 2018 edition.
- Use dyn keyword for trait objects.
- Update minimum required rustc version to 1.35.

## v0.9.5
- Implement DerefMut for text::Builder
- Add any_pointer_list and raw::get_struct_pointer_section().
- Add support for pointer field defaults.

## v0.9.4
- Add optional rpc_try feature, implementing core::ops::Try for Promise.
- Add 'raw' module with get_struct_data_section(), get_list_bytes(), and other functions.
- Avoid potential undefined behavior in canonicalizaion.
- Update a bunch of internal usages of `try!()` to `?`.

## v0.9.3
- Add IntoInternalStructReader trait and struct_list::Builder::set_with_caveats() method.
- Update deprecation attributes, to satisfy clippy.

## v0.9.2
- Rename a bunch of as_reader() methods to into_reader(), to satisfy clippy.

## v0.9.1
- Avoid some unnecessary heap allocation that could occur when reading multisegment messages.

## v0.9.0
- Add message::Builder::set_root_canonical() method. Relies on a new signature for SetPointerBuilder.
- Mark bytes_to_words() and bytes_to_words_mut() as unsafe, due to possible alignment issues. Please
  refer to https://github.com/capnproto/capnproto-rust/issues/101 for discussion.
- Delete deprecated items.
- Drop support for automatically imbuing message builders with capabilities (was unsafe). You should
  use capnp_rpc::ImbuedMessageBuilder now if you want that functionality. See the calculator example.
- Bump minimum supported rustc version to 1.26.0.

## v0.8.17
- Deprecate borrow() in favor of reborrow().

## v0.8.16
- Add serialize::write_message_segments().
- Fix bug where is_canonical() could sometimes erroneously return true.

## v0.8.15
- Add message::Builder::into_reader() and message::Reader::into_typed().

## v0.8.14
- Add message::TypedReader.
- Appease new "tyvar_behind_raw_pointer" lint (see https://github.com/rust-lang/rust/issues/46906).

## v0.8.13
- Implement capability_list, to support List(Interface).

## v0.8.12
- Avoid constructing (zero-length) slices from null pointers, as it seems to be a possible
  source of undefined behavior.
- Add some IntoIterator implementations.

## v0.8.11
- Avoid some situations where we would construct (but not dereference) out-of-bounds pointers.

## v0.8.10
- Deprecate Word::from() in favor of capnp_word!().
- Add constant::Reader to support struct and list constants.

## v0.8.9
- In canonicalization, account the possibility of nonzero padding in primitive lists.
- Do bounds-checking by (ptr, size) pairs rather than (ptr, end_ptr) pairs.

## v0.8.8
- Fix some canonicalization bugs.

## v0.8.7
- Implement `as_reader()` for lists.
- Implement `canonicalize()` and `is_canonical()`.
- Fix bug where `total_size()` returned wrong answer on empty struct lists.

## v0.8.6
- Implement struct list upgrades.
- Fix bug where `message.init_root::<any_pointer::Builder>()` did not clear the old value.

## v0.8.5
- Eliminate possible void-list-amplification in total_size().

## v0.8.4
- Eliminate panics in total_size() and set_root().
- Eliminate possible void-list-amplification in zero_object_helper().

## v0.8.3
- Prevent integer overflow possible with very long struct lists on 32-bit systems.
- Fix bug where the capnp_word!() macro was not exported for big endian targets.

## v0.8.2
- Shave some bytes off the representation of StructReader and friends.
- Fix some potential integer overflows.

## v0.8.1
- Redesign segment arenas to require less unsafe code.

## v0.8.0
- Replace optional GJ dependency with futures-rs.
- Remove `ResultsDoneHook` hack.
- No breaking changes for non-RPC users.

## v0.7.5
- Implement DoubleEndedIter for ListIter.
- Implement From<core::str::Utf8Error> for ::capnp::Error.
- Address some new linter warnings.

## v0.7.4
- Fix rare case where `serialize_packed::read()` could fail on valid input.

### v0.7.3
- Get `message::Builder::get_root_as_reader()` to work on empty messages.

### v0.7.2
- Implement `From<alloc::string::FromUtf8Error>` for `capnp::Error`
- More and better iterators.
