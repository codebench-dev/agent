package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"net/http"
	"os"

	"github.com/go-playground/validator"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/sirupsen/logrus"
)

type (
	CustomValidator struct {
		validator *validator.Validate
	}

	runReq struct {
		ID       string `json:"id" validate:"required"`
		Code     string `json:"code" validate:"required"`
		Language string `json:"language" validate:"required"`
		Variant  string `json:"variant" validate:"required"`
	}

	runCRes struct {
		Message      string `json:"message"`
		Error        string `json:"error"`
		Stdout       string `json:"stdout"`
		Stderr       string `json:"stderr"`
		ExecDuration int64  `json:"exec_duration"`
	}
)

type Language int

const (
	Python = iota + 1
	C
	Cpp
	Golang
)

func (s Language) String() string {
	return toString[s]
}

var toString = map[Language]string{
	Python: "python",
	C:      "c",
	Cpp:    "cpp",
	Golang: "golang",
}

var toID = map[string]Language{
	"python": Python,
	"c":      C,
	"cpp":    Cpp,
	"golang": Golang,
}

// MarshalJSON marshals the enum as a quoted json string
func (s Language) MarshalJSON() ([]byte, error) {
	buffer := bytes.NewBufferString(`"`)
	buffer.WriteString(toString[s])
	buffer.WriteString(`"`)
	return buffer.Bytes(), nil
}

// UnmarshalJSON unmashals a quoted json string to the enum value
func (s *Language) UnmarshalJSON(b []byte) error {
	var j string
	err := json.Unmarshal(b, &j)
	if err != nil {
		return err
	}

	if toID[j] == 0 {
		return errors.New("invalid language")
	}
	*s = toID[j]
	return nil
}

func main() {
	e := echo.New()
	e.Validator = &CustomValidator{validator: validator.New()}

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.POST("/run", handleCodeRun)
	e.GET("/health", health)

	e.Logger.Fatal(e.Start(":8080"))
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

func handleCodeRun(c echo.Context) error {
	req := new(runReq)
	err := c.Bind(req)
	if err != nil {
		return err
	}
	err = c.Validate(req)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}

	// Write code to file
	f, err := os.Create("/tmp/" + req.ID)

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

	// Call language handler
	switch toID[req.Language] {
	case Python:
		return pythonHandler(c, req)
	case C:
		return cHandler(c, req)
	case Cpp:
		return cppHandler(c, req)
	case Golang:
		return golangHandler(c, req)
	default:
		return echo.NewHTTPError(http.StatusBadRequest, "Invalid language")
	}
}
