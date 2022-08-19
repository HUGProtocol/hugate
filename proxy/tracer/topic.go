package tracer

import (
	"errors"
	"github.com/calehh/emitter"
	ethabi "github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"math/big"
	"proxy/abi"
	"strings"
)

var (
	ErrorTopicLenNotEnough = errors.New("topic len not enough")
)

var (
	ERC1155ABI, _ = ethabi.JSON(strings.NewReader(abi.ERC1155ABI))
	ERC721ABI, _  = ethabi.JSON(strings.NewReader(abi.ERC721ABI))
)

type TopicERC1155TransferSingle struct {
	EventERC1155TransferSingle
}

type EventERC1155TransferSingle struct {
	Operator common.Address
	From     common.Address
	To       common.Address
	Id       *big.Int
	Value    *big.Int
}

func (t TopicERC1155TransferSingle) GetName() string {
	return "TransferSingle"
}

//event TransferSingle(address indexed _operator, address indexed _from, address indexed _to, uint256 _id, uint256 _value);
func (t TopicERC1155TransferSingle) GetSignature() common.Hash {
	return emitter.EventSignatureHash("TransferSingle(address,address,address,uint256,uint256)")
}

func (t TopicERC1155TransferSingle) Unpack(log types.Log) (interface{}, error) {
	err := ERC1155ABI.UnpackIntoInterface(&t.EventERC1155TransferSingle, t.GetName(), log.Data)
	if err != nil {
		return nil, err
	}
	if len(log.Topics) < 4 {
		return nil, ErrorTopicLenNotEnough
	}
	t.EventERC1155TransferSingle.Operator = common.HexToAddress(log.Topics[1].Hex())
	t.EventERC1155TransferSingle.From = common.HexToAddress(log.Topics[2].Hex())
	t.EventERC1155TransferSingle.To = common.HexToAddress(log.Topics[3].Hex())
	return t.EventERC1155TransferSingle, nil
}

//todo: implement topic
type TopicERC1155TransferBatch struct{}

type TopicERC721Transfer struct {
	EventERC721Transfer
}

//event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
type EventERC721Transfer struct {
	From    common.Address
	To      common.Address
	TokenId *big.Int
}

func (t TopicERC721Transfer) GetName() string {
	return "Transfer"
}

func (t TopicERC721Transfer) GetSignature() common.Hash {
	return emitter.EventSignatureHash("Transfer(address,address,uint256)")
}

func (t TopicERC721Transfer) Unpack(log types.Log) (interface{}, error) {
	if len(log.Topics) < 4 {
		return nil, ErrorTopicLenNotEnough
	}
	t.EventERC721Transfer.From = common.HexToAddress(log.Topics[1].Hex())
	t.EventERC721Transfer.To = common.HexToAddress(log.Topics[2].Hex())
	t.EventERC721Transfer.TokenId = log.Topics[3].Big()
	return t.EventERC721Transfer, nil
}
