package main

import (
	"fmt"
	"github.com/ethereum/go-ethereum/common"
	"gopkg.in/urfave/cli.v1"
	"gopkg.in/yaml.v2"
	"io/ioutil"
	"os"
	"os/signal"
	cluster_client "proxy/cluster-client"
	db2 "proxy/db"
	"proxy/log"
	"proxy/restful"
	"proxy/snapshot"
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
		Usage: "target host",
		Value: "http://127.0.0.1:8080/",
	}
	configPathFlag = cli.StringFlag{
		Name:  "config",
		Usage: "config path",
		Value: "./config.yml",
	}
	clusterUrlFlag = cli.StringFlag{
		Name:  "cluster_url",
		Usage: "cluster url",
	}
	clusterPass = cli.StringFlag{
		Name:  "cluster_pass",
		Usage: "cluster pass",
	}
	clusterName = cli.StringFlag{
		Name:  "cluster_name",
		Usage: "cluster name",
	}
	headlessUrl = cli.StringFlag{
		Name:  "headless_url",
		Usage: "cluster name",
	}
	tmpDir = cli.StringFlag{
		Name:  "tmp",
		Usage: "tmp dir",
	}
	gatewayUrl = cli.StringFlag{
		Name:  "gate",
		Usage: "gate",
	}
)

func init() {
	app = cli.NewApp()
	app.Version = "v1.0.0"
	app.Commands = []cli.Command{
		commandStart,
		commandFile,
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
		configPathFlag,
	},
	Action: Start,
}

var commandFile = cli.Command{
	Name:  "file",
	Usage: "start file server",
	Flags: []cli.Flag{
		portFlag,
		clusterPass,
		clusterUrlFlag,
		clusterName,
		headlessUrl,
		tmpDir,
		gatewayUrl,
	},
	Action: FileServer,
}

type ProxyConfig struct {
	Port                string   `yaml:"port"`
	DBIp                string   `yaml:"db_ip"`
	ChainUrl            string   `yaml:"chain_url"`
	PassContractAddress string   `yaml:"pass_contract_address"`
	HostList            []string `yaml:"host_list"`
	DBPass              string   `yaml:"db_pass"`
}

func Start(ctx *cli.Context) {
	conf := loadConfig(ctx)
	db, _ := db2.Init(conf.DBPass, conf.DBIp)
	//if err != nil {
	//	panic("db init error")
	//}
	checker := NewChecker(db, conf.ChainUrl, common.HexToAddress(conf.PassContractAddress))
	//checker.AuthOff()
	proxier := NewProxy(conf.HostList, conf.Port, checker)
	proxier.start()
	waitToExit()
}

func loadConfig(ctx *cli.Context) ProxyConfig {
	var proxyConfig ProxyConfig
	if ctx.IsSet(configPathFlag.Name) {
		configPath := ctx.String(configPathFlag.Name)
		b, err := ioutil.ReadFile(configPath)
		if err != nil {
			log.Fatal("read config error", err)
		}
		err = yaml.Unmarshal(b, &proxyConfig)
		if err != nil {
			log.Fatal(err)
		}
	}
	if ctx.IsSet(portFlag.Name) {
		proxyConfig.Port = ctx.String(portFlag.Name)
	}

	if ctx.IsSet(dbIPFlag.Name) {
		proxyConfig.DBIp = ctx.String(dbIPFlag.Name)
	}

	if ctx.IsSet(chainURLFlag.Name) {
		proxyConfig.ChainUrl = ctx.String(chainURLFlag.Name)
	}

	if ctx.IsSet(passContractFlag.Name) {
		proxyConfig.PassContractAddress = ctx.String(passContractFlag.Name)
	}

	if ctx.IsSet(targetHostFlag.Name) {
		proxyConfig.HostList = []string{ctx.String(targetHostFlag.Name)}
	}
	return proxyConfig
}

func FileServer(ctx *cli.Context) {
	port := ctx.String(portFlag.Name)
	if !(ctx.IsSet(clusterUrlFlag.Name) && ctx.IsSet(clusterName.Name) && ctx.IsSet(clusterPass.Name) && ctx.IsSet(headlessUrl.Name) && ctx.IsSet(tmpDir.Name) && ctx.IsSet(gatewayUrl.Name)) {
		log.Fatal("flag unset")
	}
	clusterUrl := ctx.String(clusterUrlFlag.Name)
	clusterNameStr := ctx.String(clusterName.Name)
	clusterPassStr := ctx.String(clusterPass.Name)
	chromeUrl := ctx.String(headlessUrl.Name)
	cluster_client.DefaultTempFilePath = ctx.String(tmpDir.Name)
	cluster_client.GatewayUrl = ctx.String(gatewayUrl.Name)

	//init chrome agent
	agent := snapshot.NewHeadlessAgent(chromeUrl)
	agent.Start()
	//init ipfs cluster client
	client, err := cluster_client.NewClusterClient(clusterUrl, clusterNameStr, clusterPassStr)
	if err != nil {
		log.Fatal(err)
	}
	//init rpc server
	rest := restful.InitRestService(port, client, agent)
	err = rest.Start()
	if err != nil {
		log.Fatal(err)
	}
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
