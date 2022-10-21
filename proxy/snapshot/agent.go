package snapshot

import (
	"context"
	"errors"
	"fmt"
	"github.com/chromedp/cdproto/dom"
	"github.com/chromedp/chromedp"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"io/ioutil"
	"log"
	log2 "proxy/log"
	"sync"
	"time"
)

var SingleFileSnapshotTimeout = 30 * time.Second

type SnapShotReq struct {
	pic_filename         string
	text_filename        string
	text_backup_filename string
	direction            string
	resp                 chan error
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
			picFilename := req.pic_filename
			textFilename := req.text_filename
			text_backup := req.text_backup_filename
			shot := func() error {
				log2.Info(direction)
				wsUrl := agent.WsUrl
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
				fmt.Println(21)
				var res string
				err := chromedp.Run(lastCtx,
					chromedp.Navigate(direction),
					chromedp.Sleep(time.Second),
					chromedp.FullScreenshot(&buf, 90),
					chromedp.ActionFunc(func(ctx context.Context) error {
						node, err := dom.GetDocument().Do(ctx)
						if err != nil {
							return err
						}
						res, err = dom.GetOuterHTML().WithNodeID(node.NodeID).Do(ctx)
						return err
					}),
				)
				if err != nil {
					return err
				}
				fmt.Println(22)
				if err := ioutil.WriteFile(picFilename, buf, 0644); err != nil {
					return err
				}
				if err := ioutil.WriteFile(text_backup, []byte(res), 0644); err != nil {
					return err
				}
				return nil
			}
			fmt.Println(23)
			wg := sync.WaitGroup{}
			wg.Add(1)
			go func() {
				singleFileErr := SingleFileSnapshot(direction, textFilename)
				if singleFileErr != nil {
					log2.Error("singlefile snapshot error", singleFileErr)
				}
				wg.Done()
			}()
			err := shot()
			if err != nil {
				log2.Error(err)
			}
			fmt.Println(24)
			wg.Wait()
			req.resp <- err
		}
	}()
}

func SingleFileSnapshot(url string, htmlFileName string) error {
	ctx := context.Background()
	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
	if err != nil {
		return err
	}

	resp, err := cli.ContainerCreate(ctx, &container.Config{
		Image: "singlefile",
		Cmd:   []string{url},
	}, nil, nil, nil, "")
	if err != nil {
		return err
	}

	if err := cli.ContainerStart(ctx, resp.ID, types.ContainerStartOptions{}); err != nil {
		return err
	}

	defer func() {
		timeout := time.Duration(0)
		e := cli.ContainerStop(ctx, resp.ID, &timeout)
		if e != nil {
			log2.Error("stop container error", resp.ID, e)
		}
		e = cli.ContainerRemove(ctx, resp.ID, types.ContainerRemoveOptions{})
		if e != nil {
			log2.Error("remove container error", resp.ID, e)
		}
		cli.Close()
	}()

	statusCh, errCh := cli.ContainerWait(ctx, resp.ID, container.WaitConditionNotRunning)
	timer := time.NewTimer(SingleFileSnapshotTimeout)
	select {
	case err := <-errCh:
		if err != nil {
			panic(err)
		}
	case <-timer.C:
		return errors.New("timeout")
	case <-statusCh:
	}

	out, err := cli.ContainerLogs(ctx, resp.ID, types.ContainerLogsOptions{ShowStdout: true})
	if err != nil {
		return err
	}

	buf, err := ioutil.ReadAll(out)
	if err != nil {
		return err
	}
	err = ioutil.WriteFile(htmlFileName, buf, 0644)
	if err != nil {
		return err
	}
	return nil
}

func (agent *HeadlessAgent) ShotOne(direction string, pic_filename, text_filename, text_backup string) error {
	req := SnapShotReq{
		pic_filename:         pic_filename,
		text_filename:        text_filename,
		text_backup_filename: text_backup,
		direction:            direction,
		resp:                 make(chan error, 1),
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
