package main

import (
	"net/http"
	"os/exec"
	"strings"

	"github.com/go-playground/validator"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type (
	execReq struct {
		Command string `json:"command" validate:"required"`
	}

	execRes struct {
		Command string `json:"command"`
		Stdout  string `json:"stdout"`
		Stderr  string `json:"stderr"`
	}

	CustomValidator struct {
		validator *validator.Validate
	}
)

func main() {
	e := echo.New()
	e.Validator = &CustomValidator{validator: validator.New()}

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.POST("/exec", handleExec)
	e.GET("/health", health)

	e.Logger.Fatal(e.Start(":8080"))
}

func handleExec(c echo.Context) error {
	req := new(execReq)
	err := c.Bind(req)
	if err != nil {
		return err
	}
	err = c.Validate(req)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
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

func (cv *CustomValidator) Validate(i interface{}) error {
	if err := cv.validator.Struct(i); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}
	return nil
}

func health(c echo.Context) error {
	return c.String(http.StatusOK, "OK")
}
