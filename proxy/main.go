package main

import (
	"fmt"
	"github.com/ethereum/go-ethereum/common"
	"gopkg.in/urfave/cli.v1"
	"os"
	"os/signal"
	db2 "proxy/db"
	"syscall"
)

var (
	OriginCommandHelpTemplate = `{{.Name}}{{if .Subcommands}} command{{end}}{{if .Flags}} [command options]{{end}} {{.ArgsUsage}}
{{if .Description}}{{.Description}}
{{end}}{{if .Subcommands}}
SUBCOMMANDS:
  {{range .Subcommands}}{{.Name}}{{with .ShortName}}, {{.}}{{end}}{{ "\t" }}{{.Usage}}
  {{end}}{{end}}{{if .Flags}}
OPTIONS:
{{range $.Flags}}   {{.}}
{{end}}
{{end}}`
)
var app *cli.App

var (
	portFlag = cli.StringFlag{
		Name:  "port",
		Usage: "restful rpc port",
		Value: "8546",
	}
	dbIPFlag = cli.StringFlag{
		Name:  "db",
		Usage: "db ip",
	}
	chainURLFlag = cli.StringFlag{
		Name:  "chain",
		Usage: "chain url",
	}
	passContractFlag = cli.StringFlag{
		Name:  "pass",
		Usage: "pass contract address",
	}
	targetHostFlag = cli.StringFlag{
		Name:  "host",
		Usage: "targe host",
		Value: "http://127.0.0.1:8080/",
	}
)

func init() {
	app = cli.NewApp()
	app.Version = "v1.0.0"
	app.Commands = []cli.Command{
		commandStart,
	}

	cli.CommandHelpTemplate = OriginCommandHelpTemplate
}

func main() {
	if err := app.Run(os.Args); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

var commandStart = cli.Command{
	Name:  "start",
	Usage: "start loading contract gas fee",
	Flags: []cli.Flag{
		dbIPFlag,
		chainURLFlag,
		portFlag,
		passContractFlag,
		targetHostFlag,
	},
	Action: Start,
}

func Start(ctx *cli.Context) {
	port := ctx.String(portFlag.Name)
	dbIP := ctx.String(dbIPFlag.Name)
	chainUrl := ctx.String(chainURLFlag.Name)
	passAddressStr := ctx.String(passContractFlag.Name)
	passContractAddress := common.HexToAddress(passAddressStr)
	targetHost := ctx.String(targetHostFlag.Name)
	db, err := db2.Init("", dbIP)
	//if err != nil {
	//	panic("db init error")
	//}
	checker := NewChecker(db, chainUrl, passContractAddress)
	checker.AuthOff()
	proxier, err := NewProxy(targetHost, port, checker)
	if err != nil {
		panic(err)
	}
	proxier.start()
	waitToExit()
}

func waitToExit() {
	exit := make(chan bool, 0)
	sc := make(chan os.Signal, 1)
	if !signal.Ignored(syscall.SIGHUP) {
		signal.Notify(sc, syscall.SIGHUP)
	}
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		for sig := range sc {
			fmt.Printf("received exit signal:%v", sig.String())
			close(exit)
			break
		}
	}()
	<-exit
}
