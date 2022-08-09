package main

import (
	"errors"
	"fmt"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"math/big"
	"net/http"
	"proxy/abi"
	"proxy/db"
	"strconv"
)

var (
	ChainNetworkConnectionError = errors.New("connect chain node error")
	ParamsError                 = errors.New("params error")
	AttachContractError         = errors.New("attach contract error")
	ReadContractError           = errors.New("read contract error")
	NoPassNFT                   = errors.New("no pass nft")
)

type NFTChecker struct {
	db           *db.DBService
	chainUrl     string
	passContract common.Address
	authOff      bool
}

func NewChecker(db *db.DBService, url string, passContract common.Address) *NFTChecker {
	return &NFTChecker{
		db:           db,
		chainUrl:     url,
		passContract: passContract,
	}
}

func (c *NFTChecker) AuthOff() {
	c.authOff = true
}

func (c *NFTChecker) NFTPassChecker(w http.ResponseWriter, r *http.Request) (string, error) {
	if c.authOff{
		return "", nil
	}

	address := r.Header.Get("address")
	passTokenID := r.Header.Get("pass")
	channelId, err := strconv.ParseInt(passTokenID, 10, 64)
	if err != nil {
		return "", ParamsError
	}

	client, err := ethclient.Dial(c.chainUrl)
	if err != nil {
		return "", ChainNetworkConnectionError
	}
	erc1155, err := abi.NewERC1155(c.passContract, client)
	if err != nil {
		return "", AttachContractError
	}
	balance, err := erc1155.BalanceOf(&bind.CallOpts{Pending: false}, common.HexToAddress(address), big.NewInt(channelId))
	if err != nil {
		return "", ReadContractError
	}

	if balance.Int64() <= 0 {
		return "", NoPassNFT
	}

	//filename = contract_address | tokenID
	return c.passContract.String() + fmt.Sprintf("%v", channelId), nil
}
