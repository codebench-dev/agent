package main

import (
	"net/http"
	"os"
	"os/exec"
	"strings"

	"github.com/go-playground/validator"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/sirupsen/logrus"
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

	runCReq struct {
		ID   string `json:"id" validate:"required"`
		Code string `json:"code" validate:"required"`
	}

	runCRes struct {
		Message string `json:"message"`
		Stdout  string `json:"stdout"`
		Stderr  string `json:"stderr"`
	}
)

func main() {
	e := echo.New()
	e.Validator = &CustomValidator{validator: validator.New()}

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.POST("/exec", handleExec)
	e.POST("/run/c", handleRunC)
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

func handleRunC(c echo.Context) error {
	req := new(runCReq)
	err := c.Bind(req)
	if err != nil {
		return err
	}
	err = c.Validate(req)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}

	f, err := os.Create("/tmp/" + req.ID + ".c")

	if err != nil {
		logrus.WithError(err).Error()
		return c.JSON(http.StatusInternalServerError, runCRes{
			Stdout: "",
			Stderr: err.Error(),
		})
	}

	defer f.Close()

	_, err = f.WriteString(req.Code)

	if err != nil {
		logrus.WithError(err).Error()
		return c.JSON(http.StatusInternalServerError, runCRes{
			Stdout: "",
			Stderr: err.Error(),
		})
	}

	out, err := exec.Command("gcc", "/tmp/"+req.ID+".c", "-o", "/tmp/"+req.ID+".out").Output()

	if err != nil {
		return c.JSON(http.StatusBadRequest, runCRes{
			Message: "Failed to compile",
			Stdout:  string(out),
			Stderr:  err.Error(),
		})
	}

	out, err = exec.Command("/tmp/" + req.ID + ".out").Output()

	if err != nil {
		return c.JSON(http.StatusBadRequest, runCRes{
			Message: "Failed to run",
			Stdout:  string(out),
			Stderr:  err.Error(),
		})
	}

	return c.JSON(http.StatusOK, runCRes{
		Message: "Success",
		Stdout:  string(out),
		Stderr:  "",
	})
}
