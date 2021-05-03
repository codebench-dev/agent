package main

import (
	"os/exec"
	"strings"

	"github.com/gin-gonic/gin"
)

type execRequest struct {
	Command string `json:"command" binding:"required"`
}

func main() {
	r := gin.Default()
	r.POST("/exec", func(c *gin.Context) {
		var req execRequest
		c.BindJSON(&req)

		args := strings.Fields(req.Command)

		out, err := exec.Command(args[0], args[1:]...).Output()
		if err != nil {
			c.JSON(400, gin.H{
				"command": req.Command,
				"stdout":  string(out),
				"stderr":  err.Error(),
			})
		}
		c.JSON(200, gin.H{
			"command": req.Command,
			"stdout":  string(out),
			"stderr":  "",
		})

	})
	r.Run()
}
