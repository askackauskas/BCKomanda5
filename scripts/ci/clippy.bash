#!/usr/bin/env bash

# An array lets us have comments between options without abusing backticks.
options=(
	# Check integration tests and benchmarks as well.
	--all-targets

	--profile test

	# Lint settings must be specified after a `--`.
	--

    --forbid unsafe_code

    # A subset of rustc lints that are allowed by default.
    # See the output of `rustc --warn help` for a full list.
    # A few notable ones that we do not enable:
    # - `variant_size_differences`
    #   `clippy::large_enum_variant` does nearly the same thing and is enabled by default.
    # - `missing_copy_implementations`
    #   This would be more useful if it only triggered for types that are `Clone` but not `Copy`.
    # - `elided_lifetimes_in_paths`
    #   It hurts readability and doesn't provide a clear benefit.
    --warn absolute_paths_not_starting_with_crate
    --warn anonymous_parameters
    --warn deprecated_in_future
    --warn indirect_structural_match
    --warn keyword_idents
    --warn macro_use_extern_crate
    --warn meta_variable_misuse
    --warn non_ascii_idents
    --warn trivial_casts
    --warn trivial_numeric_casts
    --warn unaligned_references
    --warn unused_crate_dependencies
    --warn unused_extern_crates
    --warn unused_import_braces
    --warn unused_lifetimes
    --warn unused_qualifications

    # Additional Clippy lint groups.
    --warn clippy::nursery
    --warn clippy::pedantic

    # A subset of the `clippy::restriction` group.
    # A few notable lints from it that we do not enable:
    # - `clippy::integer_arithmetic`
    #   It's the static equivalent of `overflow-checks` in `Cargo.toml`, but it hurts readability.
    # - `clippy::mem_forget`
    #   Setting it to deny (as opposed to forbid) makes no sense.
    #   `core::mem::forget` is impossible to use by mistake.
    # - `clippy::unimplemented`
    #   It's useful to leave some trait methods unimplemented.
    #   Also, the lint doesn't seem to work.
    --warn clippy::clone_on_ref_ptr
    --warn clippy::dbg_macro
    --warn clippy::decimal_literal_representation
    --warn clippy::float_arithmetic
    --warn clippy::float_cmp_const
    --warn clippy::get_unwrap
    --warn clippy::let_underscore_must_use
    --warn clippy::lossy_float_literal
    --warn clippy::multiple_inherent_impl
    --warn clippy::print_stdout
    --warn clippy::rest_pat_in_fully_bound_structs
    --warn clippy::string_add
    --warn clippy::todo
    --warn clippy::unwrap_used
    --warn clippy::verbose_file_reads
    --warn clippy::wrong_pub_self_convention

    # Clippy suggests using the infamously unstable `!` type even on stable Rust.
    --allow clippy::empty_enum

    # These are almost never helpful.
    --allow clippy::filter_map
    --allow clippy::find_map
    --allow clippy::map_unwrap_or
    --allow clippy::single_match_else

    --allow clippy::missing_errors_doc
    --allow clippy::non_ascii_literal
)

exec cargo clippy "${options[@]}" "$@"
