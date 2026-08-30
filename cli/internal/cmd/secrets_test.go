package cmd

import (
	"bytes"
	"regexp"
	"testing"
)

// secretPattern matches a Stellar StrKey secret seed (S... ed25519 secret
// key) — the shape nothing this CLI does should ever print, to stdout,
// stderr, or an error string. Nothing here handles real key material yet
// (see internal/keys), but this is a regression guard for when it does.
var secretPattern = regexp.MustCompile(`\bS[A-Z2-7]{55}\b`)

// assertNoSecretShapedOutput fails t if either buffer contains something
// shaped like a Stellar secret key.
func assertNoSecretShapedOutput(t *testing.T, label string, stdout, stderr *bytes.Buffer) {
	t.Helper()
	if m := secretPattern.FindString(stdout.String()); m != "" {
		t.Fatalf("%s: a secret-shaped value reached stdout: %q", label, m)
	}
	if m := secretPattern.FindString(stderr.String()); m != "" {
		t.Fatalf("%s: a secret-shaped value reached stderr: %q", label, m)
	}
}

func TestNoSecretShapedValueEverReachesOutput(t *testing.T) {
	isolateConfigDir(t)

	// A deliberately secret-key-shaped string fed in as if it were a public
	// key, an identity name, or a URL — every place user-controlled input
	// flows through the command tree. None of these are valid inputs (the
	// pattern doesn't have a 'G' prefix where a public key is expected), so
	// each run is expected to fail; the only thing under test is that the
	// bogus value, and nothing resembling a real secret, ever appears in
	// what the CLI printed.
	poison := "SASAAEJC6P5UZGRLYJ2I2KYLR7RXGF44JZXDYGCFBN7T5VIHECUUEMCD"

	cases := [][]string{
		{"link", "aquawolf", "--public-key", poison},
		{"link", "aquawolf", "--public-key", poison, "--json"},
		{"link", poison, "--public-key", "GASAAEJC6P5UZGRLYJ2I2KYLR7RXGF44JZXDYGCFBN7T5VIHECUUEMCD"},
		{"--source", poison},
		{"--url", poison, "link", "aquawolf", "--public-key", poison},
	}

	for _, args := range cases {
		root := newRootCmd("dev", "none")
		stdout := &bytes.Buffer{}
		stderr := &bytes.Buffer{}
		root.SetOut(stdout)
		root.SetErr(stderr)
		root.SetArgs(args)

		_ = root.Execute() // error or not: only the output shape is under test here

		assertNoSecretShapedOutput(t, "args="+args[0], stdout, stderr)
	}
}
