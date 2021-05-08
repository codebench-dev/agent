package main

import (
	"net/http"
	"os/exec"
	"strings"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type (
	execReq struct {
		Command string `json:"command"`
	}

	execRes struct {
		Command string `json:"command"`
		Stdout  string `json:"stdout"`
		Stderr  string `json:"stderr"`
	}
)

func main() {
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.POST("/exec", handleExec)

	e.Logger.Fatal(e.Start(":8080"))
}

func handleExec(c echo.Context) error {
	req := new(execReq)
	err := c.Bind(req)
	if err != nil {
		return err
	}

	args := strings.Fields(req.Command)

	out, err := exec.Command(args[0], args[1:]...).Output()
	if err != nil {
		return c.JSON(http.StatusBadRequest, execRes{
			Command: req.Command,
			Stdout:  string(out),
			Stderr:  err.Error(),
		})
	}

	return c.JSON(http.StatusOK, execRes{
		Command: req.Command,
		Stdout:  string(out),
		Stderr:  "",
	})
}
