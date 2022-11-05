package utils

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"
)

func LoadInputFile(name string) (string, error) {
	absName, err := filepath.Abs("input/" + name)
	if err != nil {
		return "", err
	}

	ifh, err := os.Open(absName)
	if err != nil {
		return "", err
	}
	defer ifh.Close()

	lines := []string{}
	scanner := bufio.NewScanner(ifh)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return "", err
	}

	return strings.Join(lines, "\n"), nil
}
