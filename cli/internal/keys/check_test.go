package keys

import (
	"errors"
	"path/filepath"
	"strings"
	"testing"
)

func TestParseSemver(t *testing.T) {
	cases := []struct {
		in   string
		want semver
		ok   bool
	}{
		{"stellar 25.2.0\n", semver{25, 2, 0}, true},
		{"25.2.0", semver{25, 2, 0}, true},
		{"stellar-cli 22.0.1 (abcdef1)", semver{22, 0, 1}, true},
		{"no version here", semver{}, false},
	}
	for _, c := range cases {
		got, ok := parseSemver(c.in)
		if ok != c.ok {
			t.Fatalf("parseSemver(%q) ok = %v, want %v", c.in, ok, c.ok)
		}
		if ok && got != c.want {
			t.Fatalf("parseSemver(%q) = %v, want %v", c.in, got, c.want)
		}
	}
}

func TestSemverCompare(t *testing.T) {
	cases := []struct {
		a, b semver
		want int
	}{
		{semver{25, 2, 0}, semver{25, 2, 0}, 0},
		{semver{25, 2, 1}, semver{25, 2, 0}, 1},
		{semver{25, 1, 9}, semver{25, 2, 0}, -1},
		{semver{26, 0, 0}, semver{25, 2, 0}, 1},
		{semver{24, 9, 9}, semver{25, 2, 0}, -1},
	}
	for _, c := range cases {
		if got := c.a.compare(c.b); got != c.want {
			t.Fatalf("%v.compare(%v) = %d, want %d", c.a, c.b, got, c.want)
		}
	}
}

func TestCheckStellarCLIAcceptsTheMinimumVersion(t *testing.T) {
	bin := buildFakeStellar(t)
	t.Setenv("FAKESTELLAR_VERSION", MinimumStellarVersion)

	if err := CheckStellarCLI(bin); err != nil {
		t.Fatalf("CheckStellarCLI: %v", err)
	}
}

func TestCheckStellarCLIAcceptsANewerVersion(t *testing.T) {
	bin := buildFakeStellar(t)
	t.Setenv("FAKESTELLAR_VERSION", "26.0.0")

	if err := CheckStellarCLI(bin); err != nil {
		t.Fatalf("CheckStellarCLI: %v", err)
	}
}

func TestCheckStellarCLIRejectsATooOldVersion(t *testing.T) {
	bin := buildFakeStellar(t)
	t.Setenv("FAKESTELLAR_VERSION", "24.0.0")

	err := CheckStellarCLI(bin)
	if err == nil {
		t.Fatal("CheckStellarCLI succeeded with a version older than the minimum")
	}
	if !errors.Is(err, ErrStellarTooOld) {
		t.Fatalf("error does not wrap ErrStellarTooOld: %v", err)
	}
	// Actionable: names both what's required and where to get it.
	if !containsAll(err.Error(), MinimumStellarVersion, StellarInstallURL) {
		t.Fatalf("error is not actionable, missing version or install URL: %v", err)
	}
}

func TestCheckStellarCLIReportsAMissingBinaryDistinctly(t *testing.T) {
	missing := filepath.Join(t.TempDir(), "definitely-not-a-real-binary")

	err := CheckStellarCLI(missing)
	if err == nil {
		t.Fatal("CheckStellarCLI succeeded with a nonexistent binary")
	}
	if !errors.Is(err, ErrStellarNotFound) {
		t.Fatalf("error does not wrap ErrStellarNotFound: %v", err)
	}
	if !containsAll(err.Error(), StellarInstallURL) {
		t.Fatalf("error is not actionable, missing the install URL: %v", err)
	}
}

func TestCheckStellarCLIDistinguishesMissingFromTooOld(t *testing.T) {
	bin := buildFakeStellar(t)
	t.Setenv("FAKESTELLAR_VERSION", "24.0.0")
	tooOldErr := CheckStellarCLI(bin)

	missingErr := CheckStellarCLI(filepath.Join(t.TempDir(), "nope"))

	if errors.Is(tooOldErr, ErrStellarNotFound) {
		t.Fatal("a too-old version was misclassified as not-found")
	}
	if errors.Is(missingErr, ErrStellarTooOld) {
		t.Fatal("a missing binary was misclassified as too-old")
	}
}

func containsAll(s string, substrs ...string) bool {
	for _, sub := range substrs {
		if !strings.Contains(s, sub) {
			return false
		}
	}
	return true
}
