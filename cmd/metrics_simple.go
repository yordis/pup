// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var metricsCmd = &cobra.Command{
	Use:   "metrics",
	Short: "Query and manage metrics",
	Long:  `Query time-series metrics, list available metrics, and manage metric metadata.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		return fmt.Errorf("metrics command is under development - use monitors, dashboards, slos, or incidents instead")
	},
}
