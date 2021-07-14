package main

import (
	"bytes"
	"net/http"
	"os/exec"
	"syscall"
	"time"

	"github.com/labstack/echo/v4"
)

func execCmd(c echo.Context, program string, arg ...string) error {
	var execStdOut, execStdErr bytes.Buffer

	cmd := exec.Command(program, arg...)
	cmd.Stdout = &execStdOut
	cmd.Stderr = &execStdErr

	start := time.Now()
	err := cmd.Run()
	elapsed := time.Since(start)

	if err != nil {
		return c.JSON(http.StatusBadRequest, runCRes{
			Message:      "Failed to run",
			Error:        err.Error(),
			Stdout:       execStdOut.String(),
			Stderr:       execStdErr.String(),
			ExecDuration: elapsed.Microseconds(),
			MemUsage:     cmd.ProcessState.SysUsage().(*syscall.Rusage).Maxrss,
		})
	}

	return c.JSON(http.StatusOK, runCRes{
		Message:      "Success",
		Stdout:       execStdOut.String(),
		Stderr:       execStdErr.String(),
		ExecDuration: elapsed.Microseconds(),
		MemUsage:     cmd.ProcessState.SysUsage().(*syscall.Rusage).Maxrss,
	})
}
