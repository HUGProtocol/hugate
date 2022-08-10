package main

import (
	"fmt"
	"github.com/gorilla/websocket"
	"testing"
	"time"
)

func TestPingPong(t *testing.T) {
	writeWait := time.Second * 1
	pongWait := 10 * time.Second
	pingPeriod := 4 * time.Second
	c, _, err := websocket.DefaultDialer.Dial("ws://localhost:3333", nil)
	if err != nil {
		t.Fatal(err)
	}
	ticker := time.NewTicker(pingPeriod)
	timer := time.NewTimer(time.Second * 20)
	go func() {
		c.SetReadDeadline(time.Now().Add(pongWait))
		c.SetPongHandler(func(string) error {
			c.SetReadDeadline(time.Now().Add(pongWait))
			fmt.Println("got pong")
			return nil
		})
		for {
			_, _, er := c.ReadMessage()
			if er != nil {
				if websocket.IsUnexpectedCloseError(er, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
					fmt.Printf("error: %v\n", er)
				}
				panic(er)
			}
		}
	}()
	for {
		select {
		case <-ticker.C:
			c.SetWriteDeadline(time.Now().Add(writeWait))
			if er := c.WriteMessage(websocket.PingMessage, nil); er != nil {
				t.Fatal(er)
			} else {
				fmt.Println("send ping")
			}
		case <-timer.C:
			return
		}
	}
}
