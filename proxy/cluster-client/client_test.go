package cluster_client

import (
	"context"
	"fmt"
	"github.com/ipfs-cluster/ipfs-cluster/api"
	client2 "github.com/ipfs-cluster/ipfs-cluster/api/rest/client"
	ma "github.com/multiformats/go-multiaddr"
	"testing"
	"time"
)

func TestConnect(t *testing.T) {
	apiMAddr, _ := ma.NewMultiaddr("")
	cft := client2.Config{
		Username: "",
		Password: "",
		APIAddr:  apiMAddr,
	}

	client, err := client2.NewDefaultClient(&cft)
	if err != nil {
		t.Fatal(err)
	}
	id, err := client.ID(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	fmt.Println(id)
	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()
	out := make(chan api.AddedOutput, 1)
	go func() {
		err = client.Add(ctx, []string{""}, api.DefaultAddParams(), out)
		if err != nil {
			t.Error(err)
			return
		}
	}()
	select {
	case <-ctx.Done():
		t.Fatal("timeout")
	case output := <-out:
		fmt.Printf("%v", output)
	}
}
