package snapshot

import (
	"fmt"
	"io/ioutil"
	"os"
	"testing"
)

func TestContainers(t *testing.T) {
	textTempFile, err := ioutil.TempFile("/Users/houmy/Documents/social/hugate/proxy/", "upload-*")
	if err != nil {
		t.Fatal(err)
	}
	defer textTempFile.Close()
	defer os.Remove(textTempFile.Name())
	err = SingleFileSnapshot("https://www.baidu.com", textTempFile.Name())
	if err != nil {
		t.Fatal(err)
	}
	data, err := ioutil.ReadFile(textTempFile.Name())
	if err != nil {
		t.Fatal(err)
	}
	fmt.Println(string(data))
}
