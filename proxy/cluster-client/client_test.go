package cluster_client

import (
	"context"
	"fmt"
	client2 "github.com/ipfs-cluster/ipfs-cluster/api/rest/client"
	ma "github.com/multiformats/go-multiaddr"
	"testing"
)

func TestConnect(t *testing.T) {
	apiMAddr, _ := ma.NewMultiaddr("")
	cft := client2.Config{
		//SSL:               false,
		//NoVerifyCert:      false,
		Username: "",
		Password: "",
		APIAddr:  apiMAddr,
		//Host:              "",
		//Port:              "",
		//ProtectorKey:      nil,
		//ProxyAddr:         nil,
		//Timeout:           0,
		//DisableKeepAlives: false,
		//LogLevel:          "",
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
}


