package lockfile

import (
	"os"
	"testing"

	"github.com/nxpkg/nxpkg/cli/internal/nxpkgpath"
	"gotest.tools/v3/assert"
)

func getRustFixture(t *testing.T, fixture string) []byte {
	defaultCwd, err := os.Getwd()
	assert.NilError(t, err, "getRustFixture")
	cwd := nxpkgpath.AbsoluteSystemPath(defaultCwd)
	lockfilePath := cwd.UntypedJoin("../../../crates/nxpkgrepo-lockfiles/fixtures", fixture)
	if !lockfilePath.FileExists() {
		t.Errorf("unable to find 'nxpkgrepo-lockfiles/fixtures/%s'", fixture)
	}
	bytes, err := os.ReadFile(lockfilePath.ToString())
	assert.NilError(t, err, "unable to read fixture")
	return bytes
}

func getNpmFixture(t *testing.T, fixture string) Lockfile {
	bytes := getRustFixture(t, fixture)
	lf, err := DecodeNpmLockfile(bytes)
	assert.NilError(t, err)
	return lf
}

func TestAllDependenciesNpm(t *testing.T) {
	lf := getNpmFixture(t, "npm-lock.json")
	closures, err := AllTransitiveClosures(map[nxpkgpath.AnchoredUnixPath]map[string]string{
		nxpkgpath.AnchoredUnixPath(""): {
			"nxpkg":    "latest",
			"prettier": "latest",
		},
		nxpkgpath.AnchoredUnixPath("apps/web"): {
			"lodash": "^4.17.21",
			"next":   "12.3.0",
		},
	}, lf)
	assert.NilError(t, err)
	assert.Equal(t, len(closures), 2)
	rootClosure := closures[nxpkgpath.AnchoredUnixPath("")]
	webClosure := closures[nxpkgpath.AnchoredUnixPath("apps/web")]

	assert.Assert(t, rootClosure.Contains(Package{
		Key:     "node_modules/nxpkg",
		Version: "1.5.5",
		Found:   true,
	}))
	assert.Assert(t, rootClosure.Contains(Package{
		Key:     "node_modules/nxpkg-darwin-64",
		Version: "1.5.5",
		Found:   true,
	}))

	assert.Assert(t, webClosure.Contains(Package{
		Key:     "apps/web/node_modules/lodash",
		Version: "4.17.21",
		Found:   true,
	}))
	assert.Assert(t, webClosure.Contains(Package{
		Key:     "node_modules/next",
		Version: "12.3.0",
		Found:   true,
	}))
	assert.Assert(t, webClosure.Contains(Package{
		Key:     "node_modules/postcss",
		Version: "8.4.14",
		Found:   true,
	}))
}
