package main

import (
	"container/ring"
	"context"
	"fmt"
	"io"
	"log"
	"math"
	"net"
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

	// Keep the program alive until user interrupts (Ctrl+C) or the process is terminated.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	avgRequestCh := make(chan struct{}, 1)
	avgResultCh := make(chan time.Duration)
	go serveLatencyAverage(ctx, "hackattic.com", avgRequestCh, avgResultCh)
	time.Sleep(4 * time.Second)

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
				interval := computeInterval(pingTime, avgRequestCh, avgResultCh)
				pingTime = time.Now()
				err = c.Write(ctx, websocket.MessageText, strconv.AppendInt([]byte{}, interval, 10))
				if err != nil {
					panic(err)
				}
				log.Printf("\t=> sent interval %d\n", interval)
			}
		}()

	}
}

func logHandshake(resp *http.Response) {
	log.Printf("Handshake response status: %s\n", resp.Status)
	log.Printf("Handshake response headers: %v\n", resp.Header)
	if resp.Body != nil {
		defer resp.Body.Close()
		body, err := io.ReadAll(resp.Body)
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

func computeInterval(pingTime time.Time, requestCh chan struct{}, resultCh chan time.Duration) int64 {
	requestCh <- struct{}{}
	lat := <-resultCh
	log.Println("computeInterval() Received avg latency:", lat.Milliseconds(), "ms")
	duration := time.Since(pingTime).Milliseconds()
	log.Println("computeInterval() raw Duration since last ping:", duration, "ms")
	duration = duration - lat.Milliseconds()
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

func measureTCPDialDuration(host string, port int) (time.Duration, error) {
	start := time.Now()
	conn, err := net.DialTimeout("tcp", host+":"+strconv.Itoa(port), 2*time.Second)
	if err != nil {
		return 0, fmt.Errorf("failed to connect to %s:%d: %w", host, port, err)
	}
	defer conn.Close()
	//log.Println("TCP connection established in", time.Since(start).Milliseconds(), "ms")
	return time.Since(start), nil
}

func serveLatencyAverage(ctx context.Context, host string, request chan struct{}, result chan time.Duration) {
	ticker := time.NewTicker(300 * time.Millisecond)
	r := ring.New(12)
	latency, err := measureTCPDialDuration(host, 80)
	if err != nil {
		panic(err)
	}
	r.Value = latency

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			latency, err = measureTCPDialDuration(host, 80)
			if err != nil {
				log.Println(err)
				continue
			}
			r = r.Next()
			r.Value = latency
		case <-request:
			avg := avgDurationRing(r)
			log.Println("Average latency:", avg)
			result <- avg
		}
	}
}

func avgDurationRing(r *ring.Ring) time.Duration {
	if r == nil || r.Len() == 0 {
		return 0
	}
	var sum time.Duration
	var count int64

	r.Do(func(v any) {
		if v == nil {
			return
		}
		if lat, ok := v.(time.Duration); ok {
			sum += lat
			count++
		}
	})
	if count == 0 {
		return 0
	}
	return sum / time.Duration(count)
}
