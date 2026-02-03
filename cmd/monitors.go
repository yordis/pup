// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"fmt"

	"github.com/DataDog/datadog-api-client-go/v2/api/datadogV1"
	"github.com/DataDog/fetch/pkg/formatter"
	"github.com/spf13/cobra"
)

var monitorsCmd = &cobra.Command{
	Use:   "monitors",
	Short: "Manage monitors",
	Long:  `Create, update, delete, and query monitors for alerting.`,
}

var monitorsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all monitors",
	RunE:  runMonitorsList,
}

var monitorsGetCmd = &cobra.Command{
	Use:   "get [monitor-id]",
	Short: "Get monitor details",
	Args:  cobra.ExactArgs(1),
	RunE:  runMonitorsGet,
}

var monitorsDeleteCmd = &cobra.Command{
	Use:   "delete [monitor-id]",
	Short: "Delete a monitor",
	Args:  cobra.ExactArgs(1),
	RunE:  runMonitorsDelete,
}

var (
	monitorName string
	monitorTags string
)

func init() {
	monitorsListCmd.Flags().StringVar(&monitorName, "name", "", "Filter monitors by name")
	monitorsListCmd.Flags().StringVar(&monitorTags, "tags", "", "Filter monitors by tags (comma-separated)")

	monitorsCmd.AddCommand(monitorsListCmd)
	monitorsCmd.AddCommand(monitorsGetCmd)
	monitorsCmd.AddCommand(monitorsDeleteCmd)
}

func runMonitorsList(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	api := datadogV1.NewMonitorsApi(client.V1())

	opts := datadogV1.ListMonitorsOptionalParameters{}
	if monitorName != "" {
		opts.WithName(monitorName)
	}
	if monitorTags != "" {
		opts.WithTags(monitorTags)
	}

	resp, r, err := api.ListMonitors(client.Context(), opts)
	if err != nil {
		return fmt.Errorf("failed to list monitors: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runMonitorsGet(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	monitorID := args[0]
	api := datadogV1.NewMonitorsApi(client.V1())

	resp, r, err := api.GetMonitor(client.Context(), parseInt64(monitorID))
	if err != nil {
		return fmt.Errorf("failed to get monitor: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runMonitorsDelete(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	monitorID := args[0]

	// Check if auto-approve is enabled
	if !cfg.AutoApprove {
		fmt.Printf("⚠️  WARNING: This will permanently delete monitor %s\n", monitorID)
		fmt.Print("Are you sure you want to continue? (y/N): ")

		var response string
		fmt.Scanln(&response)
		if response != "y" && response != "Y" {
			fmt.Println("Operation cancelled")
			return nil
		}
	}

	api := datadogV1.NewMonitorsApi(client.V1())

	resp, r, err := api.DeleteMonitor(client.Context(), parseInt64(monitorID))
	if err != nil {
		return fmt.Errorf("failed to delete monitor: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}
