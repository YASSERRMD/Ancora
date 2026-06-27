VENV = .venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip
MATURIN = $(PYTHON) -m maturin
PYTEST = $(VENV)/bin/pytest

COVER_MIN ?= 70

.PHONY: venv develop test coverage clean

venv:
	python3 -m venv $(VENV)
	$(PIP) install -q maturin pytest

develop: venv
	$(MATURIN) develop

test: develop
	$(PYTEST) tests/ -v --asyncio-mode=auto

coverage: develop
	$(PYTEST) tests/ --cov=ancora --cov-report=term-missing --asyncio-mode=auto | \
	  awk -v min=$(COVER_MIN) '/^TOTAL/ { pct=$$4+0; if (pct<min) { print "Coverage " pct "% below threshold " min "%"; exit 1 } else { print "Coverage " pct "% OK" } }'

clean:
	rm -rf $(VENV) dist/ *.egg-info/
	find . -name "*.so" -delete
	find . -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
