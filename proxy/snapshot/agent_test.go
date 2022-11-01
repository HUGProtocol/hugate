package snapshot

import (
	"context"
	"fmt"
	"github.com/chromedp/chromedp"
	"io/ioutil"
	"log"
	"os"
	"testing"
	"time"
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

func TestScreenshot(t *testing.T) {
	direction := "https://www.baidu.com"
	wsUrl := ""

	textTempFile, err := ioutil.TempFile("/Users/houmy/Documents/social/hugate/proxy/", "upload-*")
	if err != nil {
		t.Fatal(err)
	}
	defer textTempFile.Close()

	options := []chromedp.ExecAllocatorOption{
		chromedp.Flag("ignore-certificate-errors", true),
	}
	options = append(chromedp.DefaultExecAllocatorOptions[:], options...)
	ctxTO, cancel := context.WithTimeout(context.Background(), time.Second*30)
	defer cancel()
	allocCtx, cancel := chromedp.NewExecAllocator(ctxTO, options...)
	defer cancel()
	ctx, cancel := chromedp.NewContext(
		allocCtx,
	)
	defer cancel()
	remoteCtx, cancel := chromedp.NewRemoteAllocator(ctx, wsUrl)
	defer cancel()
	lastCtx, cancel := chromedp.NewContext(remoteCtx, chromedp.WithLogf(log.Printf))
	defer cancel()
	var buf []byte
	//var res string
	err = chromedp.Run(lastCtx,
		chromedp.Navigate(direction),
		chromedp.Sleep(time.Second*10),
		chromedp.FullScreenshot(&buf, 90),
	)
	if err != nil {
		t.Fatal(err)
	}
	if err = ioutil.WriteFile(textTempFile.Name(), buf, 0644); err != nil {
		t.Fatal(err)
	}
}
