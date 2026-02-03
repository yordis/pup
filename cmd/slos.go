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

var slosCmd = &cobra.Command{
	Use:   "slos",
	Short: "Manage Service Level Objectives",
	Long:  `Create, update, delete, and query Service Level Objectives (SLOs).`,
}

var slosListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all SLOs",
	RunE:  runSlosList,
}

var slosGetCmd = &cobra.Command{
	Use:   "get [slo-id]",
	Short: "Get SLO details",
	Args:  cobra.ExactArgs(1),
	RunE:  runSlosGet,
}

var slosDeleteCmd = &cobra.Command{
	Use:   "delete [slo-id]",
	Short: "Delete an SLO",
	Args:  cobra.ExactArgs(1),
	RunE:  runSlosDelete,
}

func init() {
	slosCmd.AddCommand(slosListCmd)
	slosCmd.AddCommand(slosGetCmd)
	slosCmd.AddCommand(slosDeleteCmd)
}

func runSlosList(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	api := datadogV1.NewServiceLevelObjectivesApi(client.V1())

	resp, r, err := api.ListSLOs(client.Context())
	if err != nil {
		return fmt.Errorf("failed to list SLOs: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runSlosGet(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	sloID := args[0]
	api := datadogV1.NewServiceLevelObjectivesApi(client.V1())

	resp, r, err := api.GetSLO(client.Context(), sloID)
	if err != nil {
		return fmt.Errorf("failed to get SLO: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runSlosDelete(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	sloID := args[0]

	// Check if auto-approve is enabled
	if !cfg.AutoApprove {
		fmt.Printf("⚠️  WARNING: This will permanently delete SLO %s\n", sloID)
		fmt.Print("Are you sure you want to continue? (y/N): ")

		var response string
		fmt.Scanln(&response)
		if response != "y" && response != "Y" {
			fmt.Println("Operation cancelled")
			return nil
		}
	}

	api := datadogV1.NewServiceLevelObjectivesApi(client.V1())

	resp, r, err := api.DeleteSLO(client.Context(), sloID)
	if err != nil {
		return fmt.Errorf("failed to delete SLO: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}
