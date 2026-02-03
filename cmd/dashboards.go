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

var dashboardsCmd = &cobra.Command{
	Use:   "dashboards",
	Short: "Manage dashboards",
	Long:  `Create, update, delete, and query visualization dashboards.`,
}

var dashboardsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all dashboards",
	RunE:  runDashboardsList,
}

var dashboardsGetCmd = &cobra.Command{
	Use:   "get [dashboard-id]",
	Short: "Get dashboard details",
	Args:  cobra.ExactArgs(1),
	RunE:  runDashboardsGet,
}

var dashboardsDeleteCmd = &cobra.Command{
	Use:   "delete [dashboard-id]",
	Short: "Delete a dashboard",
	Args:  cobra.ExactArgs(1),
	RunE:  runDashboardsDelete,
}

func init() {
	dashboardsCmd.AddCommand(dashboardsListCmd)
	dashboardsCmd.AddCommand(dashboardsGetCmd)
	dashboardsCmd.AddCommand(dashboardsDeleteCmd)
}

func runDashboardsList(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	api := datadogV1.NewDashboardsApi(client.V1())

	resp, r, err := api.ListDashboards(client.Context())
	if err != nil {
		return fmt.Errorf("failed to list dashboards: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runDashboardsGet(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	dashboardID := args[0]
	api := datadogV1.NewDashboardsApi(client.V1())

	resp, r, err := api.GetDashboard(client.Context(), dashboardID)
	if err != nil {
		return fmt.Errorf("failed to get dashboard: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runDashboardsDelete(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	dashboardID := args[0]

	// Check if auto-approve is enabled
	if !cfg.AutoApprove {
		fmt.Printf("⚠️  WARNING: This will permanently delete dashboard %s\n", dashboardID)
		fmt.Print("Are you sure you want to continue? (y/N): ")

		var response string
		fmt.Scanln(&response)
		if response != "y" && response != "Y" {
			fmt.Println("Operation cancelled")
			return nil
		}
	}

	api := datadogV1.NewDashboardsApi(client.V1())

	resp, r, err := api.DeleteDashboard(client.Context(), dashboardID)
	if err != nil {
		return fmt.Errorf("failed to delete dashboard: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}
