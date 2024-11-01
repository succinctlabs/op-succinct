package metrics

import (
	"fmt"
	"net/http"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	// Counters
	LatestContractL2Block = promauto.NewCounter(prometheus.CounterOpts{
		Name: "op_proposer_latest_contract_l2_block",
		Help: "Latest L2 block number on the contract",
	})

	LatestL2FinalizedBlock = promauto.NewCounter(prometheus.CounterOpts{
		Name: "op_proposer_latest_l2_finalized_block",
		Help: "Latest finalized L2 block number",
	})

	// Gauges
	NumProving = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "op_proposer_num_proving",
		Help: "Number of proofs currently being proven",
	})

	NumWitnessgen = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "op_proposer_num_witnessgen",
		Help: "Number of proofs in witness generation",
	})

	NumErrors = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "op_proposer_num_errors",
		Help: "Number of errors encountered",
	})
)

// StartPrometheusServer starts a Prometheus metrics server on the specified port
func StartPrometheusServer(port string) error {
	fmt.Println("Starting Prometheus metrics server on port", port)
	http.Handle("/metrics", promhttp.Handler())
	return http.ListenAndServe(":"+port, nil)
}
