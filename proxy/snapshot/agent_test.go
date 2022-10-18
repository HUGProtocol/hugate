package snapshot

import (
	"testing"
)

func TestContainers(t *testing.T) {
	err := SingleFileSnapshot("https://www.baidu.com", "./index.html")
	if err != nil {
		t.Fatal(err)
	}
}
