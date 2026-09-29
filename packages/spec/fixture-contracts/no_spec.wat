;; Minimal module with no `contractspecv0` custom section.
;;
;; The reader must report "no interface" for this binary rather than a parse
;; error: absence of a spec is a different condition from a corrupt one (see
;; corrupt_section.wasm). Compiled with `wat2wasm` (WABT); the build script
;; regenerates the committed output from this source.
(module)
