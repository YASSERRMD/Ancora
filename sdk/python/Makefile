VENV = .venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip
MATURIN = $(PYTHON) -m maturin
PYTEST = $(VENV)/bin/pytest

.PHONY: venv develop test clean

venv:
	python3 -m venv $(VENV)
	$(PIP) install -q maturin pytest

develop: venv
	$(MATURIN) develop

test: develop
	$(PYTEST) tests/ -v

clean:
	rm -rf $(VENV) dist/ *.egg-info/
	find . -name "*.so" -delete
	find . -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
