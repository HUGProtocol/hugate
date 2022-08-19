package tracer

import (
	"context"
	emitter2 "github.com/calehh/emitter"
	"github.com/ethereum/go-ethereum/common"
	"proxy/log"
	"testing"
	"time"
)

func TestEmitter(t *testing.T) {
	emitter, err := NewEmitter("./height",
		emitter2.Config{CheckDuration: time.Second, MaxRequestHeight: 100},
		[]int64{31337},
	)
	if err != nil {
		t.Fatal(err)
	}
	contractInfoList := []emitter2.ContractInfo{
		{
			Address: common.HexToAddress("0x5FC8d32690cc91D4c39d9d3abcBD16989F875707"),
			TopicList: []emitter2.Topic{TopicERC1155TransferSingle{},
			}},
		{
			Address: common.HexToAddress("0x23dB4a08f2272df049a4932a4Cc3A6Dc1002B33E"),
			TopicList: []emitter2.Topic{TopicERC721Transfer{},
			},
		}}
	chainInfo := emitter2.ChainInfo{
		RPC:            "http://127.0.0.1:8545",
		ChainID:        31337,
		FilterContract: contractInfoList,
	}
	eventCh := make(chan emitter2.Event)
	go func() {
		e := emitter.SubscribeChainEvent(context.Background(), chainInfo, eventCh)
		if e != nil {
			t.Fatal(e)
		}
	}()
	go func() {
		for {
			select {
			case event := <-eventCh:
				log.Infof("event %v", event)
			}
		}
	}()
	time.Sleep(time.Second * 10)
}
