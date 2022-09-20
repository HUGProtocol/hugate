package snapshot

import (
	"context"
	"errors"
	"fmt"
	"github.com/chromedp/chromedp"
	"io/ioutil"
	"log"
	log2 "proxy/log"
	"time"
)

type SnapShotReq struct {
	filename  string
	direction string
	resp      chan error
}

type HeadlessAgent struct {
	WsUrl   string
	pending chan SnapShotReq
}

func NewHeadlessAgent(wsUrl string) *HeadlessAgent {
	return &HeadlessAgent{
		WsUrl:   wsUrl,
		pending: make(chan SnapShotReq),
	}
}

func (agent *HeadlessAgent) Start() {
	go func() {
		for req := range agent.pending {
			direction := req.direction
			filename := req.filename
			shot := func() error {
				log2.Info(direction)
				wsUrl := agent.WsUrl
				options := []chromedp.ExecAllocatorOption{
					chromedp.Flag("ignore-certificate-errors", true),
				}
				options = append(chromedp.DefaultExecAllocatorOptions[:], options...)
				ctxTO, cancel := context.WithTimeout(context.Background(), time.Minute)
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
				fmt.Println(21)
				if err := chromedp.Run(lastCtx, fullScreenshot(direction, 90, &buf)); err != nil {
					return err
				}
				fmt.Println(22)
				if err := ioutil.WriteFile(filename, buf, 0644); err != nil {
					return err
				}
				return nil
			}
			fmt.Println(23)
			err := shot()
			if err != nil {
				log2.Error(err)
			}
			fmt.Println(24)
			req.resp <- err
		}
	}()
}

func (agent *HeadlessAgent) ShotOne(direction string, filename string) error {
	req := SnapShotReq{
		filename:  filename,
		direction: direction,
		resp:      make(chan error, 1),
	}
	ticker := time.NewTicker(time.Minute * 5)
	select {
	case agent.pending <- req:
		fmt.Println(11)
		err := <-req.resp
		return err
	case <-ticker.C:
		fmt.Println(12)
		return errors.New("headless chrome busy")
	}
}

func elementScreenshot(urlstr, sel string, res *[]byte) chromedp.Tasks {
	return chromedp.Tasks{
		chromedp.Navigate(urlstr),
		chromedp.Screenshot(sel, res, chromedp.NodeVisible),
	}
}

func fullScreenshot(urlstr string, quality int, res *[]byte) chromedp.Tasks {
	return chromedp.Tasks{
		chromedp.Navigate(urlstr),
		chromedp.Sleep(time.Second),
		chromedp.FullScreenshot(res, quality),
	}
}
