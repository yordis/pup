// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"fmt"

	"github.com/DataDog/datadog-api-client-go/v2/api/datadogV2"
	"github.com/DataDog/fetch/pkg/formatter"
	"github.com/spf13/cobra"
)

var incidentsCmd = &cobra.Command{
	Use:   "incidents",
	Short: "Manage incidents",
	Long:  `Create, update, and query incidents for incident management.`,
}

var incidentsListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all incidents",
	RunE:  runIncidentsList,
}

var incidentsGetCmd = &cobra.Command{
	Use:   "get [incident-id]",
	Short: "Get incident details",
	Args:  cobra.ExactArgs(1),
	RunE:  runIncidentsGet,
}

func init() {
	incidentsCmd.AddCommand(incidentsListCmd)
	incidentsCmd.AddCommand(incidentsGetCmd)
}

func runIncidentsList(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	api := datadogV2.NewIncidentsApi(client.V2())

	resp, r, err := api.ListIncidents(client.Context())
	if err != nil {
		return fmt.Errorf("failed to list incidents: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}

func runIncidentsGet(cmd *cobra.Command, args []string) error {
	client, err := getClient()
	if err != nil {
		return err
	}

	incidentID := args[0]
	api := datadogV2.NewIncidentsApi(client.V2())

	resp, r, err := api.GetIncident(client.Context(), incidentID)
	if err != nil {
		return fmt.Errorf("failed to get incident: %w (status: %d)", err, r.StatusCode)
	}

	output, err := formatter.ToJSON(resp)
	if err != nil {
		return err
	}

	fmt.Println(output)
	return nil
}
