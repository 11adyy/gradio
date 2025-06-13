# gradio

A Rust desktop application for selecting numeric gradations, generating compatible value series, and combining them into candidate sets. The graphical interface is built with `eframe` and `egui`; results can be filtered to a requested range and saved as an RTF document for opening in word-processing software.

## Workflow

1. Select gradation series from the built-in table or enter gradations manually.
2. Set the minimum and maximum values for the final results.
3. Generate possible series from the selected gradations and remove duplicates.
4. Generate combinations of those series, sort and filter the candidates.
5. Save the final table to an RTF file or return to the selection stage.

The built-in source table provides several gradations, from small increments to larger steps. The table and series logic are separate from the GUI, so the generation code can be inspected independently.

## Build and run

Install the Rust toolchain and build the application with Cargo:

```bash
cargo build --release
cargo run --release
```

The project uses `eframe` and `egui` for its interface, `rfd` for the save-file dialog, and `chrono` for timestamped output names. Cargo downloads these crates on the first build.

## Source layout

- `src/main.rs` defines the application screens and user interaction.
- `src/setter/` contains series and table types plus combination generation.
- `src/processor/` contains experimental calculation routines.
- `Cargo.toml` declares the crate and GUI dependencies.

Generated output is saved through the operating-system file dialog. The RTF writer escapes non-ASCII characters for document output. For large inputs, the number of combinations can grow quickly; choose gradations and ranges with the resulting search space in mind.

## Status

This repository is an evolving numeric set-generation tool. The current package name and repository name differ: Cargo calls the crate `data-generator`, while the GitHub project is `gradio`. Consult the implementation for the currently supported generation rules and defaults.
