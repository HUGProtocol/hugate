package db

import (
	"errors"
	"fmt"
	"github.com/ethereum/go-ethereum/common"
	"github.com/neo4j/neo4j-go-driver/v4/neo4j"
	"proxy/log"
)

var (
	GetNodeFromResultNotOk = errors.New("get node from result not ok")
)

type NeoDB struct {
	driver neo4j.Driver
}

func InitNeoDB(target, userName, pass string) (*NeoDB, error) {
	driver, err := neo4j.NewDriver(target,
		neo4j.BasicAuth(userName, pass, ""))
	if err != nil {
		return nil, err
	}
	err = driver.VerifyConnectivity()
	if err != nil {
		return nil, err
	}
	return &NeoDB{driver: driver}, nil
}

func (neo *NeoDB) NewCoreContract(address common.Address) error {
	session := neo.driver.NewSession(neo4j.SessionConfig{})
	defer session.Close()
	addressStr, err := session.WriteTransaction(func(tx neo4j.Transaction) (interface{}, error) {
		result, err := tx.Run("MERGE(n:CoreContract {address: $address}) RETURN n",
			map[string]interface{}{"address": address.String()})
		if err != nil {
			return "", err
		}
		coreRecorde, err := result.Single()
		if err != nil {
			return "", err
		}
		coreNode, ok := coreRecorde.Get("n")
		if !ok {
			return "", GetNodeFromResultNotOk
		}
		core := coreNode.(neo4j.Node)
		return core.Props["address"], nil
	})
	if err != nil {
		return err
	}
	log.Info("neo4j new core contract node", addressStr)
	return nil
}

//create node
func (neo *NeoDB) NewChannel(channelId int64, coreContract common.Address, channelContract common.Address) error {
	session := neo.driver.NewSession(neo4j.SessionConfig{})
	defer session.Close()
	token_id, err := session.WriteTransaction(func(tx neo4j.Transaction) (interface{}, error) {
		result, err := tx.Run(
			"Create(chan:Channel {token_id: $token_id}) "+
				"WITH chan "+
				"set chan.address = $chan_address "+
				"WITH chan "+
				"MATCH(core:CoreContract {address:$core_address}) "+
				"MERGE (chan)-[:MINTED_IN]->(core) "+
				"RETURN chan",
			map[string]interface{}{
				"token_id":     channelId,
				"core_address": coreContract.String(),
				"chan_address": channelContract.String(),
			})
		if err != nil {
			return nil, err
		}
		channelRecord, err := result.Single()
		if err != nil {
			return nil, err
		}
		channelNode, ok := channelRecord.Get("chan")
		if !ok {
			return nil, GetNodeFromResultNotOk
		}
		node := channelNode.(neo4j.Node)
		id := node.Props["token_id"]
		return id, nil
	})
	if err != nil {
		return err
	}
	log.Infof("neo4j new channel %v minted in %v", token_id, coreContract.String())
	return nil
}

func (neo *NeoDB) NewUser(address common.Address) error {
	session := neo.driver.NewSession(neo4j.SessionConfig{})
	defer session.Close()
	userAddress, err := session.WriteTransaction(func(tx neo4j.Transaction) (interface{}, error) {
		result, err := tx.Run(
			"MERGE(u:User {address:$address}) RETURN u",
			map[string]interface{}{
				"address": address.String(),
			})
		if err != nil {
			return nil, err
		}
		userRecord, err := result.Single()
		if err != nil {
			return nil, err
		}
		userNode, ok := userRecord.Get("u")
		if !ok {
			return nil, GetNodeFromResultNotOk
		}
		node := userNode.(neo4j.Node)
		addr := node.Props["address"]
		return addr, nil
	})
	if err != nil {
		return err
	}
	log.Info("neo4j new user", userAddress)
	return nil
}

func (neo *NeoDB) NewArticle(articleId int64, channelId int64, coreContract common.Address) error {
	session := neo.driver.NewSession(neo4j.SessionConfig{})
	defer session.Close()
	id, err := session.WriteTransaction(func(tx neo4j.Transaction) (interface{}, error) {
		result, err := tx.Run(
			"MERGE(a:Article {token_id:$article_id}) "+
				"WITH a "+
				"MATCH(chan:Channel {token_id: $channel_id})-[:MINTED_IN]->(core:CoreContract {address:$core_address}) "+
				"WITH a,chan "+
				"MERGE (a)-[:POST_IN]->(chan) "+
				"RETURN a",
			map[string]interface{}{
				"article_id":   articleId,
				"channel_id":   channelId,
				"core_address": coreContract.String(),
			})
		if err != nil {
			return nil, err
		}
		articleRecord, err := result.Single()
		if err != nil {
			return nil, err
		}
		articleNode, ok := articleRecord.Get("a")
		if !ok {
			return nil, GetNodeFromResultNotOk
		}
		node := articleNode.(neo4j.Node)
		id := node.Props["token_id"]
		return id, nil
	})
	if err != nil {
		return err
	}
	log.Infof("neo4j channel %v new article %v", channelId, id)
	return nil
}

func (neo *NeoDB) UserSubscribeChannel(userAddr common.Address, channelId int64, coreContract common.Address) error {
	session := neo.driver.NewSession(neo4j.SessionConfig{})
	defer session.Close()
	_, err := session.WriteTransaction(func(tx neo4j.Transaction) (interface{}, error) {
		_, err := tx.Run(
			"MATCH(chan:Channel {token_id: $channel_id})-[:MINTED_IN]->(core:CoreContract {address:$core_address}) "+
				"MATCH(u:User {address:$user_address}) "+
				"MERGE(u)-[:SUBSCRIBE]->(chan)",
			map[string]interface{}{
				"channel_id":   channelId,
				"core_address": coreContract.String(),
				"user_address": userAddr.String(),
			})
		return nil, err
	})
	return err
}

func (neo *NeoDB) IfUserSubscribe(userAddress common.Address, channelId int64, coreContract common.Address) bool {
	session := neo.driver.NewSession(neo4j.SessionConfig{})
	defer session.Close()
	addr, err := session.ReadTransaction(func(tx neo4j.Transaction) (interface{}, error) {
		result, err := tx.Run(
			"MATCH(chan:Channel {token_id: $channel_id})-[:MINTED_IN]->(core:CoreContract {address:$core_address}) "+
				"MATCH(u:User {address:$user_address})-[:SUBSCRIBE]->(chan) "+
				"RETURN u",

			map[string]interface{}{
				"channel_id":   channelId,
				"core_address": coreContract.String(),
				"user_address": userAddress.String(),
			})
		if err != nil {
			return nil, err
		}
		userRecord, err := result.Single()
		if err != nil {
			return nil, err
		}
		userNode, ok := userRecord.Get("u")
		if !ok {
			return nil, GetNodeFromResultNotOk
		}
		node := userNode.(neo4j.Node)
		return node.Props["address"], nil
	})
	if err != nil {
		fmt.Println(err)
		return false
	}
	if addr != userAddress.String() {
		fmt.Println("user address incorrect")
		return false
	}
	return true
}
