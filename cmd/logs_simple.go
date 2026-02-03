// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var logsCmd = &cobra.Command{
	Use:   "logs",
	Short: "Search and analyze logs",
	Long:  `Search and analyze log data with flexible queries and time ranges.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		return fmt.Errorf("logs command is under development - use monitors, dashboards, slos, or incidents instead")
	},
}
