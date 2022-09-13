package cluster_client

import (
	"context"
	"errors"
	"github.com/ipfs-cluster/ipfs-cluster/api"
	"github.com/ipfs-cluster/ipfs-cluster/api/rest/client"
	ma "github.com/multiformats/go-multiaddr"
	"proxy/log"
	"time"
)

var DefaultTempFilePath = ""

var GatewayUrl = ""

type ClusterClient struct {
	client     client.Client
	addTimeout time.Duration
	GatewayUrl string
}

func NewClusterClient(url, username, pass string) (*ClusterClient, error) {
	apiMAddr, _ := ma.NewMultiaddr(url)
	cft := client.Config{
		Username: username,
		Password: pass,
		APIAddr:  apiMAddr,
	}

	c, err := client.NewDefaultClient(&cft)
	if err != nil {
		return nil, err
	}
	return &ClusterClient{
		client:     c,
		addTimeout: time.Minute * 2,
		GatewayUrl: GatewayUrl,
	}, nil
}

func (c *ClusterClient) Add(filename string) (api.AddedOutput, error) {
	ctx, cancel := context.WithTimeout(context.Background(), c.addTimeout)
	defer cancel()
	out := make(chan api.AddedOutput, 1)
	go func() {
		err := c.client.Add(context.Background(), []string{filename}, api.DefaultAddParams(), out)
		if err != nil {
			log.Error(err)
		}
	}()
	select {
	case <-ctx.Done():
		return api.AddedOutput{}, errors.New("timeout")
	case output := <-out:
		return output, nil
	}
}
