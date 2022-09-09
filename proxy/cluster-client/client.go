package cluster_client

import (
	"context"
	"errors"
	"github.com/ipfs-cluster/ipfs-cluster/api"
	"github.com/ipfs-cluster/ipfs-cluster/api/rest/client"
	ma "github.com/multiformats/go-multiaddr"
	"time"
)

const (
	DefaultTempFilePath = "~/temp_file"
)

type ClusterClient struct {
	client     client.Client
	addTimeout time.Duration
	path       []string
	Url string
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
		addTimeout: time.Minute * 5,
		path:       []string{DefaultTempFilePath},
	}, nil
}

func (c *ClusterClient) Add() (api.AddedOutput, error) {
	ctx, cancel := context.WithTimeout(context.Background(), c.addTimeout)
	defer cancel()
	out := make(chan api.AddedOutput)
	err := c.client.Add(ctx, c.path, api.DefaultAddParams(), out)
	if err != nil {
		return api.AddedOutput{}, err
	}
	timer := time.NewTimer(c.addTimeout)
	select {
	case <-timer.C:
		return api.AddedOutput{}, errors.New("timeout")
	case output := <-out:
		return output, nil
	}
}
