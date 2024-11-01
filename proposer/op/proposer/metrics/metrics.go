package metrics

import (
	"io"

	"github.com/ethereum/go-ethereum/log"

	"github.com/ethereum-optimism/optimism/op-service/eth"
	opmetrics "github.com/ethereum-optimism/optimism/op-service/metrics"
	txmetrics "github.com/ethereum-optimism/optimism/op-service/txmgr/metrics"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/prometheus/client_golang/prometheus"
)

const Namespace = "op_succinct_proposer"

// implements the Registry getter, for metrics HTTP server to hook into
var _ opmetrics.RegistryMetricer = (*OPSuccinctMetrics)(nil)

type OPSuccinctMetricer interface {
	RecordInfo(version string)
	RecordUp()

	// Records all L1 and L2 block events
	opmetrics.RefMetricer

	// Record Tx metrics
	txmetrics.TxMetricer

	// Record RPC metrics
	opmetrics.RPCMetricer

	StartBalanceMetrics(l log.Logger, client *ethclient.Client, account common.Address) io.Closer

	RecordL2BlocksProposed(l2ref eth.L2BlockRef)
}

type OPSuccinctMetrics struct {
	ns       string
	registry *prometheus.Registry
	factory  opmetrics.Factory

	opmetrics.RefMetrics
	txmetrics.TxMetrics
	opmetrics.RPCMetrics

	info prometheus.GaugeVec
	up   prometheus.Gauge

	NumProving     prometheus.Gauge
	NumWitnessGen  prometheus.Gauge
	NumUnrequested prometheus.Gauge

	L2FinalizedBlock               prometheus.Gauge
	LatestContractL2Block          prometheus.Gauge
	HighestProvenContiguousL2Block prometheus.Gauge
}

var _ OPSuccinctMetricer = (*OPSuccinctMetrics)(nil)

func NewMetrics(procName string) *OPSuccinctMetrics {
	if procName == "" {
		procName = "default"
	}
	ns := Namespace + "_" + procName

	registry := opmetrics.NewRegistry()
	factory := opmetrics.With(registry)

	return &OPSuccinctMetrics{
		ns:       ns,
		registry: registry,
		factory:  factory,

		RefMetrics: opmetrics.MakeRefMetrics(ns, factory),
		TxMetrics:  txmetrics.MakeTxMetrics(ns, factory),
		RPCMetrics: opmetrics.MakeRPCMetrics(ns, factory),

		info: *factory.NewGaugeVec(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "info",
			Help:      "Pseudo-metric tracking version and config info",
		}, []string{
			"version",
		}),
		up: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "up",
			Help:      "1 if the op-proposer has finished starting up",
		}),
		NumProving: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "num_proving",
			Help:      "Number of proofs currently being generated",
		}),
		NumWitnessGen: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "num_witness_gen",
			Help:      "Number of witness generations in progress",
		}),
		NumUnrequested: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "num_unrequested",
			Help:      "Number of unrequested proofs",
		}),
		L2FinalizedBlock: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "l2_finalized_block",
			Help:      "Latest finalized L2 block number",
		}),
		LatestContractL2Block: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "latest_contract_l2_block",
			Help:      "Latest L2 block number on the L2OO contract",
		}),
		HighestProvenContiguousL2Block: factory.NewGauge(prometheus.GaugeOpts{
			Namespace: ns,
			Name:      "highest_proven_contiguous_l2_block",
			Help:      "Highest proven L2 block contiguous with contract's latest block",
		}),
	}
}

func (m *OPSuccinctMetrics) Registry() *prometheus.Registry {
	return m.registry
}

func (m *OPSuccinctMetrics) StartBalanceMetrics(l log.Logger, client *ethclient.Client, account common.Address) io.Closer {
	return opmetrics.LaunchBalanceMetrics(l, m.registry, m.ns, client, account)
}

// RecordInfo sets a pseudo-metric that contains versioning and
// config info for the op-proposer.
func (m *OPSuccinctMetrics) RecordInfo(version string) {
	m.info.WithLabelValues(version).Set(1)
}

// RecordUp sets the up metric to 1.
func (m *OPSuccinctMetrics) RecordUp() {
	prometheus.MustRegister()
	m.up.Set(1)
}

const (
	BlockProposed = "proposed"
)

// RecordL2BlocksProposed should be called when new L2 block is proposed
func (m *OPSuccinctMetrics) RecordL2BlocksProposed(l2ref eth.L2BlockRef) {
	m.RecordL2Ref(BlockProposed, l2ref)
}

func (m *OPSuccinctMetrics) Document() []opmetrics.DocumentedMetric {
	return m.factory.Document()
}
