package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"math"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"syscall"
	"time"

	"github.com/coder/websocket"
)

const WSBaseURL = "wss://hackattic.com/_/ws/"

func main() {
	token := os.Getenv("HACKATTIC_TOKEN")
	if token == "" {
		panic("HACKATTIC_TOKEN environment variable not set")
	}
	fmt.Println("Hello World")

	// Keep the program alive until user interrupts (Ctrl+C) or the process is terminated.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	wsToken := getWebsocketToken(token)
	c, resp, err := websocket.Dial(ctx, WSBaseURL+wsToken, nil)
	pingTime := time.Now()
	if err != nil {
		panic(err)
	}

	logHandshake(resp)

	defer c.CloseNow()

	for {
		msgType, data, err := c.Read(ctx)
		if err != nil {
			if websocket.CloseStatus(err) == websocket.StatusNormalClosure {
				log.Println("Connection closed normally")
				break
			}
			panic(err)
		}
		go func() {
			log.Printf("msgType=%v, data=%s\n", msgType, data)
			if string(data) == "ping!" {
				interval := computeInterval(pingTime)
				pingTime = time.Now()
				err = c.Write(ctx, websocket.MessageText, strconv.AppendInt([]byte{}, interval, 10))
				if err != nil {
					panic(err)
				}
				log.Printf("\t=> sent interval %d\n", interval)
			}
		}()
	}
	//submitSolution(token, "zut")
}

func logHandshake(resp *http.Response) {
	log.Printf("Handshake response status: %s\n", resp.Status)
	log.Printf("Handshake response headers: %v\n", resp.Header)
	if resp.Body != nil {
		body, err := io.ReadAll(resp.Body)
		defer resp.Body.Close()
		if err != nil {
			panic(err)
		}
		log.Printf("Handshake response body: %s\n", body)
	}
}

type intervalDiff struct {
	interval int64
	diff     float64
}

func computeInterval(pingTime time.Time) int64 {
	duration := time.Since(pingTime).Milliseconds()
	intervals := []int64{700, 1500, 2000, 2500, 3000}
	bestInterval := intervalDiff{diff: math.MaxFloat64}
	for _, interval := range intervals {
		currentDiff := math.Abs(float64(duration - interval))
		if currentDiff < bestInterval.diff {
			bestInterval = intervalDiff{interval: interval, diff: currentDiff}
		}
	}
	return bestInterval.interval
}
