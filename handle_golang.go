package main

import (
	"bytes"
	"io/ioutil"
	"net/http"
	"os/exec"

	"github.com/labstack/echo/v4"
)

func copy(src string, dst string) error {
	data, err := ioutil.ReadFile(src)
	if err != nil {
		return err
	}
	err = ioutil.WriteFile(dst, data, 0644)
	return err
}

func golangHandler(c echo.Context, req *runReq) error {
	// TODO handle variant

	err := copy("/tmp/"+req.ID, "/tmp/"+req.ID+".go")
	if err != nil {
		return c.JSON(http.StatusInternalServerError, runCRes{
			Message: "Failed to copy file",
			Error:   err.Error(),
		})
	}
	// Compile code
	var compileStdOut, compileStdErr bytes.Buffer
	compileCmd := exec.Command("go", "build", "-o", "/tmp/"+req.ID+".out", "/tmp/"+req.ID+".go")
	compileCmd.Stdout = &compileStdOut
	compileCmd.Stderr = &compileStdErr
	err = compileCmd.Run()

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
