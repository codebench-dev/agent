package main

import (
	"bytes"
	"net/http"
	"os/exec"

	"github.com/labstack/echo/v4"
)

func cHandler(c echo.Context, req *runReq) error {
	// TODO handle variant

	// Compile code
	var compileStdOut, compileStdErr bytes.Buffer
	compileCmd := exec.Command("gcc", "-x", "c", "/tmp/"+req.ID, "-o", "/tmp/"+req.ID+".out")
	compileCmd.Stdout = &compileStdOut
	compileCmd.Stderr = &compileStdErr
	err := compileCmd.Run()

	if err != nil {
		return c.JSON(http.StatusBadRequest, runCRes{
			Message: "Failed to compile",
			Error:   err.Error(),
			Stdout:  compileStdOut.String(),
			Stderr:  compileStdErr.String(),
		})
	}

	// Run executable
	return execCmd(c, "/tmp/"+req.ID+".out")
}
