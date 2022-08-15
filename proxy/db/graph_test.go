package db

import (
	"fmt"
	"github.com/ethereum/go-ethereum/common"
	"testing"
)

var (
	CoreContractAddress    = common.HexToAddress("0x9C3739D43a89cedf167204550267797F5931ebF5")
	ChannelContractAddress = common.HexToAddress("0xBb57EdbAaB0F56ECF494e77a73D5Fd951C295d48")
	UserAddress            = common.HexToAddress("0x20a7d78ee8536698B3eAB7afe16FA7F399a6eE02")
)

func newTestDB(t *testing.T) *NeoDB {
	target := "bolt://localhost:7687"
	user := "neo4j"
	pass := ""
	neo, err := InitNeoDB(target, user, pass)
	if err != nil {
		t.Fatal(err)
	}
	return neo
}

func TestInitNeoDB(t *testing.T) {
	target := "bolt://localhost:7687"
	user := "neo4j"
	pass := "cqmygysdss"
	neo, err := InitNeoDB(target, user, pass)
	if err != nil {
		t.Fatal(err)
	}
	neo.driver.Close()
}

func TestNewCoreContract(t *testing.T) {
	neo := newTestDB(t)

	err := neo.NewCoreContract(CoreContractAddress)
	if err != nil {
		t.Fatal(err)
	}
}

func TestNewChannel(t *testing.T) {
	neo := newTestDB(t)
	err := neo.NewChannel(1, CoreContractAddress, ChannelContractAddress)
	if err != nil {
		t.Fatal(err)
	}
}

func TestNewUser(t *testing.T) {
	neo := newTestDB(t)
	err := neo.NewUser(UserAddress)
	if err != nil {
		t.Fatal(err)
	}
}

func TestUserSub(t *testing.T) {
	neo := newTestDB(t)
	err := neo.UserSubscribeChannel(UserAddress, 1, CoreContractAddress)
	if err != nil {
		t.Fatal(err)
	}
}

func TestNewArticle(t *testing.T) {
	neo := newTestDB(t)
	err := neo.NewArticle(2, 1, CoreContractAddress)
	if err != nil {
		t.Fatal(err)
	}
}

func TestIfUserSubscribe(t *testing.T) {
	neo := newTestDB(t)
	ok := neo.IfUserSubscribe(UserAddress, 1, CoreContractAddress)
	fmt.Println(ok)
}
