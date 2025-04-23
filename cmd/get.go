/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/napkin-community/stapler/internal"
	"github.com/spf13/cobra"
)

// getCmd represents the get command
var getCmd = &cobra.Command{
	Use:   "get",
	Short: "Get the most recent draf of Napkin",
	Long: `Download the Napkin book from Web[1] to $HOME/.stapler/napkin-draft.pdf
If there was a pre-existing PDF file, prompt user to overwrite or not

[1]: https://venhance.github.io/napkin/Napkin.pdf`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("get called")
		for _, x := range args {
			fmt.Println(x)
		}
	},
}

func init() {
	getCmd.Flags().BoolP("force", "f", false, "Force overwrite pre-existing PDF file")
	rootCmd.AddCommand(getCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// getCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// getCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
