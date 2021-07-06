package main

import (
	"github.com/labstack/echo/v4"
)

func pythonHandler(c echo.Context, req *runReq) error {
	// TODO handle variant

	return execCmd(c, "python", "/tmp/"+req.ID)
}
